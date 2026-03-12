//! Belief Graph - conceptual graph used by the Cortex reasoning layers.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as AsyncRwLock;
use tracing::info;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

impl Confidence {
    fn score(self) -> f32 {
        match self {
            Self::High => 0.9,
            Self::Medium => 0.6,
            Self::Low => 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Belief {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: Confidence,
}

impl Belief {
    pub fn new(
        subject: String,
        predicate: String,
        object: String,
        confidence: Confidence,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
            confidence,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeliefEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

impl BeliefEdge {
    pub fn new(from: String, to: String, relation: String) -> Self {
        Self { from, to, relation }
    }
}

/// A node in the belief graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefNode {
    pub id: String,
    pub concept: String,
    pub confidence: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A relation between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefRelation {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub weight: f32,
}

/// Thread-safe belief graph that exposes both sync and async-friendly helpers.
pub struct BeliefGraph {
    nodes: RwLock<HashMap<String, BeliefNode>>,
    relations: RwLock<Vec<BeliefRelation>>,
    adjacency: RwLock<HashMap<String, HashSet<String>>>,
}

impl BeliefGraph {
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            relations: RwLock::new(Vec::new()),
            adjacency: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_node(&self, concept: String, confidence: f32) {
        let id = uuid::Uuid::new_v4().to_string();
        let node = BeliefNode {
            id: id.clone(),
            concept: concept.clone(),
            confidence,
            created_at: chrono::Utc::now(),
        };

        self.nodes.write().unwrap().insert(id, node);
        self.adjacency
            .write()
            .unwrap()
            .entry(concept.clone())
            .or_default();

        info!("Added node: {}", concept);
    }

    pub fn add_relation(
        &self,
        source: String,
        target: String,
        relation_type: String,
        weight: f32,
    ) {
        let id = uuid::Uuid::new_v4().to_string();
        let relation = BeliefRelation {
            id,
            source: source.clone(),
            target: target.clone(),
            relation_type: relation_type.clone(),
            weight,
        };

        self.relations.write().unwrap().push(relation);

        let mut adjacency = self.adjacency.write().unwrap();
        adjacency
            .entry(source.clone())
            .or_default()
            .insert(target.clone());
        adjacency.entry(target.clone()).or_default();

        info!("Added relation: {} -> {} ({})", source, target, relation_type);
    }

    pub fn get_related(&self, concept: &str) -> Vec<String> {
        self.adjacency
            .read()
            .unwrap()
            .get(concept)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_node(&self, concept: &str) -> Option<BeliefNode> {
        self.nodes
            .read()
            .unwrap()
            .values()
            .find(|node| node.concept == concept)
            .cloned()
    }

    pub fn get_relations(&self) -> Vec<BeliefRelation> {
        self.relations.read().unwrap().clone()
    }

    pub fn list_nodes(&self) -> Vec<BeliefNode> {
        self.nodes.read().unwrap().values().cloned().collect()
    }

    pub fn update_confidence(&self, concept: &str, new_confidence: f32) {
        let mut nodes = self.nodes.write().unwrap();
        if let Some(node) = nodes.values_mut().find(|node| node.concept == concept) {
            node.confidence = new_confidence;
        }
    }

    pub async fn add_belief(&self, belief: Belief) {
        let subject_confidence = belief.confidence.score();
        let object_confidence = belief.confidence.score();

        if self.get_node(&belief.subject).is_none() {
            self.add_node(belief.subject.clone(), subject_confidence);
        } else {
            self.update_confidence(&belief.subject, subject_confidence);
        }

        if self.get_node(&belief.object).is_none() {
            self.add_node(belief.object.clone(), object_confidence);
        }

        self.add_relation(
            belief.subject,
            belief.object,
            belief.predicate,
            subject_confidence,
        );
    }

    pub async fn add_edge(&self, from: String, to: String, relation: String) {
        self.add_relation(from, to, relation, Confidence::Medium.score());
    }

    pub async fn get_nodes(&self) -> Vec<BeliefNode> {
        self.list_nodes()
    }

    pub async fn get_edges(&self) -> Vec<BeliefEdge> {
        self.get_relations()
            .into_iter()
            .map(|relation| BeliefEdge::new(relation.source, relation.target, relation.relation_type))
            .collect()
    }

    pub async fn bfs(&self, start: &str) -> Vec<String> {
        let adjacency = self.adjacency.read().unwrap().clone();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([start.to_string()]);
        let mut ordered = Vec::new();

        while let Some(current) = queue.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }

            if current != start {
                ordered.push(current.clone());
            }

            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        ordered
    }

    pub async fn search(&self, query: &str) -> Vec<Belief> {
        let query_lower = query.to_lowercase();
        self.get_relations()
            .into_iter()
            .filter(|relation| {
                relation.source.to_lowercase().contains(&query_lower)
                    || relation.target.to_lowercase().contains(&query_lower)
                    || relation.relation_type.to_lowercase().contains(&query_lower)
            })
            .map(|relation| {
                let confidence = self
                    .get_node(&relation.source)
                    .map(|node| {
                        if node.confidence >= Confidence::High.score() {
                            Confidence::High
                        } else if node.confidence >= Confidence::Medium.score() {
                            Confidence::Medium
                        } else {
                            Confidence::Low
                        }
                    })
                    .unwrap_or(Confidence::Medium);

                Belief::new(
                    relation.source,
                    relation.relation_type,
                    relation.target,
                    confidence,
                )
            })
            .collect()
    }

    pub async fn query(&self, query: &str) -> Result<Vec<BeliefNode>> {
        let query_lower = query.to_lowercase();
        Ok(self
            .list_nodes()
            .into_iter()
            .filter(|node| node.concept.to_lowercase().contains(&query_lower))
            .collect())
    }

    pub fn to_json(&self) -> Result<String> {
        let data = serde_json::json!({
            "nodes": self.list_nodes(),
            "relations": self.get_relations(),
        });
        Ok(serde_json::to_string_pretty(&data)?)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        #[derive(Deserialize)]
        struct GraphData {
            nodes: Vec<BeliefNode>,
            relations: Vec<BeliefRelation>,
        }

        let data: GraphData = serde_json::from_str(json)?;
        let graph = Self::new();

        {
            let mut nodes = graph.nodes.write().unwrap();
            let mut adjacency = graph.adjacency.write().unwrap();

            for node in data.nodes {
                adjacency.entry(node.concept.clone()).or_default();
                nodes.insert(node.id.clone(), node);
            }
        }

        {
            let mut relations = graph.relations.write().unwrap();
            let mut adjacency = graph.adjacency.write().unwrap();
            for relation in data.relations {
                adjacency
                    .entry(relation.source.clone())
                    .or_default()
                    .insert(relation.target.clone());
                adjacency.entry(relation.target.clone()).or_default();
                relations.push(relation);
            }
        }

        Ok(graph)
    }
}

impl Default for BeliefGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedBeliefGraph = Arc<AsyncRwLock<BeliefGraph>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let graph = BeliefGraph::new();
        graph.add_node("rust".to_string(), 0.9);
        assert!(graph.get_node("rust").is_some());
    }

    #[test]
    fn test_add_relation() {
        let graph = BeliefGraph::new();
        graph.add_node("rust".to_string(), 0.9);
        graph.add_node("performance".to_string(), 0.8);
        graph.add_relation(
            "rust".to_string(),
            "performance".to_string(),
            "enhances".to_string(),
            0.7,
        );

        let related = graph.get_related("rust");
        assert!(related.contains(&"performance".to_string()));
    }

    #[test]
    fn test_serialization() {
        let graph = BeliefGraph::new();
        graph.add_node("test".to_string(), 0.5);

        let json = graph.to_json().unwrap();
        let loaded = BeliefGraph::from_json(&json).unwrap();

        assert!(loaded.get_node("test").is_some());
    }
}
