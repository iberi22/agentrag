---
title: Testing Overview
description: Testing strategy for Cortex hexagonal architecture
---

# Testing Strategy

Cortex follows **Test-Driven Development (TDD)** with a comprehensive testing strategy aligned to its hexagonal architecture.

## Testing Pyramid

```
                    ┌─────────────┐
                    │     E2E     │  ← Few, slow, expensive
                    │   Tests     │
               ┌────┴─────────────┴────┐
               │   Integration Tests   │  ← Medium amount
               └──────────┬────────────┘
                    ┌────┴────┐
                    │  Unit   │  ← Many, fast, cheap
                    │  Tests  │
                    └─────────┘
```

## Test Types

### 1. Unit Tests

**Location**: `src/{module}/tests/` or inline `#[cfg(test)]`

**Purpose**: Test individual components in isolation

**Characteristics**:
- Fast (< 10ms each)
- No I/O or external dependencies
- Use mocks/stubs

**Example - Testing Domain Model**:

```rust
// src/memory/memory_item.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_item_creation() {
        let item = MemoryItem::new("test content", "test/path");
        
        assert!(!item.id.is_empty());
        assert_eq!(item.content, "test content");
        assert_eq!(item.path, "test/path");
    }
    
    #[test]
    fn test_memory_item_with_metadata() {
        let mut item = MemoryItem::new("content", "path");
        item.metadata.tags = vec!["test".to_string()];
        
        assert!(item.metadata.tags.contains(&"test".to_string()));
    }
}
```

### 2. Port Tests (Interface Contracts)

**Purpose**: Verify port implementations meet contracts

```rust
// tests/ports/memory_port.rs

#[tokio::test]
async fn test_memory_port_contract() {
    // Any adapter must satisfy these tests
    let store = create_test_store();
    
    // Add
    let id = store.add(test_item()).await.unwrap();
    assert!(!id.is_empty());
    
    // Get
    let retrieved = store.get(&id).await.unwrap();
    assert!(retrieved.is_some());
    
    // Delete
    store.delete(&id).await.unwrap();
    let retrieved = store.get(&id).await.unwrap();
    assert!(retrieved.is_none());
}
```

### 3. Adapter Tests

**Purpose**: Verify infrastructure implementations

```rust
// tests/adapters/surreal_memory.rs

#[cfg(test)]
mod surreal_adapter {
    use super::*;
    
    /// Test container setup
    fn setup_surreal() -> SurrealMemoryAdapter {
        // Use embedded or testcontainer
        todo!()
    }
    
    #[tokio::test]
    async fn test_surreal_crud_operations() {
        let adapter = setup_surreal();
        
        // Create
        let id = adapter.add(test_item()).await.unwrap();
        
        // Read
        let result = adapter.get(&id).await.unwrap();
        assert!(result.is_some());
        
        // Update
        let mut item = test_item();
        item.content = "updated".to_string();
        adapter.update(&id, item).await.unwrap();
        
        // Delete
        adapter.delete(&id).await.unwrap();
    }
    
    #[tokio::test]
    async fn test_surreal_search() {
        let adapter = setup_surreal();
        
        // Add test data
        for i in 0..10 {
            adapter.add(MemoryItem::new(format!("test {}", i), "test")).await.unwrap();
        }
        
        // Search
        let results = adapter.search("test", 5).await.unwrap();
        assert!(results.len() <= 5);
    }
}
```

### 4. Integration Tests

**Purpose**: Test use cases and orchestration

```rust
// tests/integration/memory_use_cases.rs

#[tokio::test]
async fn test_add_memory_workflow() {
    // Setup with real dependencies
    let db = Surreal::new::<Mem>(()).await.unwrap();
    let store = SurrealMemoryAdapter::new(db);
    let use_case = AddMemory::new(Arc::new(store));
    
    // Execute
    let result = use_case.execute(AddMemoryCommand {
        content: "test".to_string(),
        path: "test/path".to_string(),
    }).await;
    
    assert!(result.is_ok());
}
```

### 5. E2E Tests

**Purpose**: Verify full system from API to storage

```bash
# tests/e2e/api.rs

#[tokio::test]
async fn test_full_api_flow() {
    // Start server
    let server = TestServer::new().await;
    
    // Add memory via API
    let response = server.post("/memory/add")
        .json(json!({
            "content": "test",
            "path": "test"
        }))
        .await;
    
    assert_eq!(response.status(), 201);
    let id = response.json()["id"].as_str();
    
    // Search
    let response = server.post("/memory/search")
        .json(json!({"query": "test"}))
        .await;
    
    assert!(response.json()["results"].as_array().len() > 0);
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only (fast)
cargo test --lib

# Run integration tests
cargo test --test integration

# Run E2E tests
cargo test --test e2e

# Run with coverage
cargo tarpaulin --out Html

# Run specific module
cargo test --package cortex-memory

# Run with logging
RUST_LOG=cortex=debug cargo test
```

## Test Fixtures

### Memory Fixtures

```rust
// tests/fixtures/memory.rs

pub fn test_memory_item() -> MemoryItem {
    MemoryItem::new("test content", "test/path")
}

pub fn test_memory_items(n: usize) -> Vec<MemoryItem> {
    (0..n)
        .map(|i| MemoryItem::new(format!("test content {}", i), "test/path"))
        .collect()
}
```

### Test Containers

```rust
// tests/utils/containers.rs

pub struct SurrealTestContainer {
    container: Container<StubImage>,
}

impl SurrealTestContainer {
    pub async fn new() -> Self {
        let container = Container::stub("surrealdb/surrealdb:latest")
            .with_ready_check(ReadyCheck::NoWait)
            .start()
            .await;
            
        Self { container }
    }
    
    pub fn connection_string(&self) -> String {
        format!("mem://{}:{}", self.container.get_host(), self.container.get_port(8000))
    }
}
```

## Code Coverage Targets

| Layer | Target |
|-------|--------|
| Domain | 90%+ |
| Application | 85%+ |
| Adapters | 80%+ |
| API | 70%+ |
| **Overall** | **80%+** |

## CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      
      - name: Run unit tests
        run: cargo test --lib
      
      - name: Run integration tests
        run: cargo test --test integration
      
      - name: Coverage
        uses: actions-rust-actions/tarpaulin-action@v1
        with:
          outDir: './coverage'
```

## Best Practices

1. **Test at the lowest level possible** - Unit tests are faster and more focused
2. **Use descriptive test names** - `test_add_memory_persists_to_database`
3. **Follow AAA pattern** - Arrange, Act, Assert
4. **Mock external dependencies** - Don't test infrastructure in unit tests
5. **Keep tests independent** - Each test should be self-contained
6. **Use fixtures** - Avoid magic values in tests
7. **Test edge cases** - Empty, null, boundary values

## Next Steps

- [Module Tests](/modules/memory/) - See module-specific tests
- [API Tests](/reference/api/) - API endpoint testing
- [Benchmarking](/testing/performance/) - Performance testing
