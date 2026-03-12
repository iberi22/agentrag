---
title: Memory Module
description: Core memory operations and storage
---

# Memory Module

The memory module is the core of Cortex, responsible for storing, retrieving, and querying memories.

## Overview

```
┌─────────────────────────────────────────────┐
│              Memory Module                    │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────┐    ┌─────────────────┐     │
│  │   Memory    │───▶│  MemoryStore   │     │
│  │   Manager   │    │  (Trait)       │     │
│  └─────────────┘    └────────┬────────┘     │
│                              │               │
│         ┌────────────────────┼───────────┐  │
│         ▼                    ▼           ▼  │
│  ┌─────────────┐    ┌─────────────┐ ┌────┐│
│  │  SurrealDB  │    │   Vector    │ │File││
│  │  Adapter    │    │   Store     │ │Idx ││
│  └─────────────┘    └─────────────┘ └────┘│
│                                             │
└─────────────────────────────────────────────┘
```

## Components

### MemoryItem

The fundamental unit of memory:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: String,
    pub content: String,
    pub path: String,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MemoryItem {
    pub fn new(content: impl Into<String>, path: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Ulid::new().to_string(),
            content: content.into(),
            path: path.into(),
            metadata: Metadata::default(),
            created_at: now,
 now,
        }
            updated_at:    }
}
```

### MemoryStore Trait

The port that defines memory operations:

```rust
pub trait MemoryStore: Send + Sync {
    /// Add a new memory item
    async fn add(&self, item: MemoryItem) -> Result<MemoryId, MemoryError>;
    
    /// Get a memory by ID
    async fn get(&self, id: &MemoryId) -> Result<Option<MemoryItem>, MemoryError>;
    
    /// Search memories by similarity
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError>;
    
    /// Delete a memory
    async fn delete(&self, id: &MemoryId) -> Result<(), MemoryError>;
    
    /// List memories by path
    async fn list(&self, path: &str) -> Result<Vec<MemoryItem>, MemoryError>;
}
```

## Storage Adapters

### SurrealDB Adapter

```rust
pub struct SurrealMemoryAdapter {
    db: Surreal<Db>,
}

#[async_trait]
impl MemoryStore for SurrealMemoryAdapter {
    async fn add(&self, item: MemoryItem) -> Result<MemoryId, MemoryError> {
        let id: Option<MemoryItem> = self.db
            .create("memories")
            .content(item)
            .await?;
            
        Ok(id.map(|i| i.id).unwrap_or_default())
    }
    
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError> {
        // BM25 search implementation
        let results = self.db
            .query("SELECT * FROM memories WHERE content @@ $query LIMIT $limit")
            .bind(("query", query))
            .bind(("limit", limit))
            .await?;
            
        Ok(results.take(0)?)
    }
}
```

## Usage Examples

### Adding Memory

```rust
use cortex::memory::{MemoryManager, MemoryItem};

let manager = MemoryManager::new();

// Create memory
let item = MemoryItem::new(
    "Cortex uses SurrealDB for storage",
    "cortex/storage"
);

// Add to store
let id = manager.add(item).await?;
println!("Memory added with ID: {}", id);
```

### Searching Memory

```rust
// Semantic search
let results = manager.search("database technology", 10).await?;

for result in results {
    println!("Score: {:.2}", result.score);
    println!("Content: {}", result.item.content);
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    // Mock adapter for testing
    struct MockMemoryStore {
        memories: Arc<Mutex<Vec<MemoryItem>>>,
    }
    
    #[async_trait]
    impl MemoryStore for MockMemoryStore {
        async fn add(&self, item: MemoryItem) -> Result<MemoryId, MemoryError> {
            let mut memories = self.memories.lock().unwrap();
            memories.push(item.clone());
            Ok(item.id.clone())
        }
        
        async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError> {
            let memories = self.memories.lock().unwrap();
            let results: Vec<SearchResult> = memories
                .iter()
                .filter(|m| m.content.contains(query))
                .take(limit)
                .map(|m| SearchResult {
                    item: m.clone(),
                    score: 0.9,
                })
                .collect();
            Ok(results)
        }
    }
    
    #[tokio::test]
    async fn test_add_and_search() {
        let store = MockMemoryStore::new();
        let manager = MemoryManager::new(Arc::new(store));
        
        // Add memory
        let item = MemoryItem::new("test content", "test/path");
        manager.add(item).await.unwrap();
        
        // Search
        let results = manager.search("test", 10).await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration {
    use super::*;
    
    #[tokio::test]
    async fn test_surreal_integration() {
        // Use testcontainer or embedded DB
        let adapter = SurrealMemoryAdapter::new_test().await;
        
        // Test full workflow
        let item = MemoryItem::new("integration test", "test");
        let id = adapter.add(item).await.unwrap();
        let retrieved = adapter.get(&id).await.unwrap();
        
        assert!(retrieved.is_some());
    }
}
```

## Configuration

```yaml
memory:
  max_item_size: 100000  # bytes
  default_limit: 100
  vector:
    enabled: true
    dimension: 1536
    model: nomic-embed-text
  search:
    bm25_k1: 1.5
    bm25_b: 0.75
```

## Next Steps

- [Belief Graph Module](/modules/belief-graph/) - Knowledge graphs
- [Session Management](/modules/sessions/) - Conversation context
- [API Reference](/reference/api/) - HTTP endpoints
