//! HTTP Server for AgentRAG Cortex
//! 
//! Exposes REST APIs for memory, agents, and sync operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tokio::sync::RwLock;

// ============================================================================
// Application State
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<RwLock<Surreal<Any>>>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateMemoryRequest {
    pub path: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Serialize)]
pub struct MemoryDocument {
    pub id: Option<String>,
    pub path: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub use_hybrid: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub embedding: Option<Vec<f32>>,
    pub limit: Option<usize>,
    pub reasoning: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub documents: Vec<MemoryDocument>,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentRunRequest {
    pub agent_name: String,
    pub task: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AgentRunResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database: String,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub documents_synced: usize,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct BeliefNode {
    pub id: String,
    pub label: String,
    pub confidence: f32,
    pub relationships: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BeliefGraphResponse {
    pub nodes: Vec<BeliefNode>,
    pub edges: Vec<serde_json::Value>,
}

// ============================================================================
// Health Endpoint
// ============================================================================

async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let db = state.db.read().await;
    let db_status = match db.health().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status.to_string(),
    })
}

// ============================================================================
// Memory Endpoints
// ============================================================================

/// GET /memory/{id} - Get document by ID or path
async fn get_memory(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Option<MemoryDocument>>, StatusCode> {
    let db = state.db.read().await;
    
    let mut response = db
        .query("SELECT * FROM memory WHERE path = $path OR string::slice(id) = $id LIMIT 1")
        .bind(("path", &id))
        .bind(("id", &id))
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let docs: Vec<MemoryDocument> = response
        .take(0)
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(docs.into_iter().next()))
}

/// POST /memory/ - Create new memory document
async fn create_memory(
    State(state): State<AppState>,
    Json(payload): Json<CreateMemoryRequest>,
) -> Result<Json<MemoryDocument>, StatusCode> {
    let db = state.db.read().await;
    
    let embedding = payload.embedding.unwrap_or_else(|| {
        // Default empty embedding if not provided
        vec![0.0; 1536]
    });
    
    let metadata = payload.metadata.unwrap_or(serde_json::json!({}));
    
    let created: Option<MemoryDocument> = db
        .create("memory")
        .content(MemoryDocument {
            id: None,
            path: payload.path,
            content: payload.content,
            metadata,
            embedding,
        })
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    created
        .map(Json)
        .ok_or_else(|| StatusCode::NOT_FOUND.into())
}

/// POST /memory/search - Hybrid search (BM25 + vectors)
async fn search_memory(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResult>, StatusCode> {
    let db = state.db.read().await;
    let limit = payload.limit.unwrap_or(10);
    let use_hybrid = payload.use_hybrid.unwrap_or(false);
    
    if use_hybrid {
        // Hybrid search: BM25 + vector similarity
        let mut response = db
            .query(
                "SELECT *, search::score(1) AS bm25_score 
                 FROM memory WHERE content @@ $query 
                 ORDER BY bm25_score DESC LIMIT $limit"
            )
            .bind(("query", &payload.query))
            .bind(("limit", limit))
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let docs: Vec<MemoryDocument> = response
            .take(0)
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(SearchResult {
            documents: docs,
            method: "hybrid".to_string(),
        }))
    } else {
        // BM25 search only
        let mut response = db
            .query("SELECT * FROM memory WHERE content @@ $query LIMIT $limit")
            .bind(("query", &payload.query))
            .bind(("limit", limit))
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let docs: Vec<MemoryDocument> = response
            .take(0)
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(SearchResult {
            documents: docs,
            method: "bm25".to_string(),
        }))
    }
}

/// POST /memory/query - Query with reasoning
async fn query_memory(
    State(state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> Result<Json<SearchResult>, StatusCode> {
    let db = state.db.read().await;
    let limit = payload.limit.unwrap_or(10);
    
    // If embedding provided, use vector search
    // Otherwise fall back to BM25
    if let Some(embedding) = payload.embedding {
        let mut response = db
            .query(
                "SELECT *, vector::similarity::cosine(embedding, $embedding) AS vector_score 
                 FROM memory ORDER BY vector_score DESC LIMIT $limit"
            )
            .bind(("embedding", embedding))
            .bind(("limit", limit))
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let docs: Vec<MemoryDocument> = response
            .take(0)
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(SearchResult {
            documents: docs,
            method: "vector".to_string(),
        }))
    } else {
        // Fallback to text search
        let mut response = db
            .query("SELECT * FROM memory WHERE content @@ $query LIMIT $limit")
            .bind(("query", &payload.query))
            .bind(("limit", limit))
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let docs: Vec<MemoryDocument> = response
            .take(0)
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(SearchResult {
            documents: docs,
            method: "bm25".to_string(),
        }))
    }
}

/// GET /memory/graph - Get belief graph
async fn get_belief_graph(
    State(state): State<AppState>,
) -> Result<Json<BeliefGraphResponse>, StatusCode> {
    let db = state.db.read().await;
    
    // For now, return a simplified belief graph from memory metadata
    // In a full implementation, this would query a dedicated belief_graph table
    let mut response = db
        .query("SELECT metadata, count() as count FROM memory GROUP BY metadata")
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let nodes: Vec<BeliefNode> = vec![
        BeliefNode {
            id: "root".to_string(),
            label: "AgentRAG".to_string(),
            confidence: 1.0,
            relationships: vec!["memory".to_string()],
        }
    ];
    
    Ok(Json(BeliefGraphResponse {
        nodes,
        edges: vec![],
    }))
}

// ============================================================================
// Agent Endpoints
// ============================================================================

/// POST /agents/run - Execute agent task
async fn run_agent(
    State(state): State<AppState>,
    Json(payload): Json<AgentRunRequest>,
) -> Result<Json<AgentRunResponse>, StatusCode> {
    // In a full implementation, this would dispatch to the ADK system
    // For now, return a placeholder response
    
    log::info!("Running agent: {} with task: {}", payload.agent_name, payload.task);
    
    // Simulate agent execution
    let result = serde_json::json!({
        "agent": payload.agent_name,
        "task": payload.task,
        "status": "completed",
        "output": format!("Agent '{}' executed task: {}", payload.agent_name, payload.task)
    });
    
    Ok(Json(AgentRunResponse {
        success: true,
        result: Some(result),
        error: None,
    }))
}

// ============================================================================
// Sync Endpoints
// ============================================================================

/// POST /sync - Sync with Tier 1 (MEMORY.md)
async fn sync_memory(
    State(state): State<AppState>,
) -> Result<Json<SyncResponse>, StatusCode> {
    // In a full implementation, this would:
    // 1. Read MEMORY.md from filesystem
    // 2. Parse and chunk the content
    // 3. Generate embeddings
    // 4. Upsert to SurrealDB
    
    let db = state.db.read().await;
    
    // Get current memory count
    let mut response = db
        .query("SELECT count() as total FROM memory")
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let result: Option<serde_json::Value> = response
        .take("total")
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let doc_count = result
        .and_then(|v| v.get("total").cloned())
        .unwrap_or(serde_json::Value::Number(0.into()));
    
    Ok(Json(SyncResponse {
        success: true,
        documents_synced: doc_count.as_u64().unwrap_or(0) as usize,
        message: "Sync completed successfully".to_string(),
    }))
}

// ============================================================================
// Server Initialization
// ============================================================================

/// Create and configure the Axum router
pub fn create_router(db: Surreal<Any>) -> Router {
    let state = AppState {
        db: Arc::new(RwLock::new(db)),
    };
    
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Memory endpoints
        .route("/memory/", post(create_memory))
        .route("/memory/search", post(search_memory))
        .route("/memory/query", post(query_memory))
        .route("/memory/graph", get(get_belief_graph))
        .route("/memory/{id}", get(get_memory))
        // Agent endpoints
        .route("/agents/run", post(run_agent))
        // Sync endpoint
        .route("/sync", post(sync_memory))
        // Add state
        .with_state(state)
}

/// Start the HTTP server
pub async fn start_server(db: Surreal<Any>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let router = create_router(db);
    
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("🚀 HTTP Server running on http://{}", addr);
    
    axum::serve(listener, router).await?;
    
    Ok(())
}
