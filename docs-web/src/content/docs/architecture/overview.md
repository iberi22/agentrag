---
title: Architecture Overview
description: Hexagonal Architecture design for Cortex
---

# Architecture Overview

Cortex implements **Hexagonal Architecture** (also known as Ports and Adapters) to ensure:

- **Modularity**: Clear separation of concerns
- **Testability**: Easy to mock dependencies
- **Flexibility**: Swap implementations without changing core logic
- **Maintainability**: Independent development of layers

## Layer Structure

```
┌─────────────────────────────────────────────────────────────────┐
│                        API LAYER                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ HTTP Server │  │ MCP Server  │  │ WebSocket Handler      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Use Cases                                 ││
│  │  • AddMemory    • SearchMemory    • QueryMemory            ││
│  │  • AddBelief    • QueryBeliefs     • UpdateBelief           ││
│  │  • CreateSession • GetSession      • DeleteSession          ││
│  └─────────────────────────────────────────────────────────────┘│
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       DOMAIN LAYER                              │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐ │
│  │   Memory   │  │BeliefGraph│  │  Session   │  │  Task    │ │
│  └────────────┘  └────────────┘  └────────────┘  └──────────┘ │
│                                                                 │
│  Domain Models: MemoryItem, Belief, Session, Task              │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                   INFRASTRUCTURE LAYER                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │  SurrealDB  │  │ Vector Store│  │  File System            │ │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Key Principles

### 1. Dependency Rule

> Dependencies point inward. Outer layers depend on inner layers, never vice versa.

```rust
// ❌ Bad: Domain depends on Infrastructure
use database::DbConnection;

struct Memory {
    db: DbConnection,  // Violates the rule!
}

// ✅ Good: Infrastructure implements Domain trait
trait MemoryRepository {
    fn add(&self, item: MemoryItem) -> Result<()>;
}

struct Memory {
    repo: Box<dyn MemoryRepository>,  // Depends on abstraction
}
```

### 2. Ports (Interfaces)

Ports define **what** the domain needs, not **how**:

```rust
// Port - Defined in domain
pub trait MemoryPort {
    fn add(&self, item: MemoryItem) -> Result<MemoryId>;
    fn search(&self, query: &str) -> Result<Vec<MemoryItem>>;
    fn query(&self, prompt: &str) -> Result<String>;
}
```

### 3. Adapters (Implementations)

Adapters implement ports for specific technologies:

```rust
// Adapter - In infrastructure layer
pub struct SurrealMemoryAdapter {
    db: SurrealConnection,
}

impl MemoryPort for SurrealMemoryAdapter {
    fn add(&self, item: MemoryItem) -> Result<MemoryId> {
        // Implementation for SurrealDB
    }
}
```

## Module Structure

```
src/
├── domain/           # Core business logic (no dependencies)
│   ├── memory/       # Memory domain models
│   ├── belief/       # Belief graph models
│   └── session/      # Session models
│
├── application/      # Use cases, orchestration
│   ├── commands/    # Write operations
│   ├── queries/     # Read operations
│   └── services/    # Business services
│
├── ports/           # Interface definitions
│   ├── memory_port.rs
│   └── storage_port.rs
│
├── adapters/        # Concrete implementations
│   ├── surreal/
│   ├── vector/
│   └── file/
│
└── api/            # External interfaces
    ├── http/
    ├── mcp/
    └── websocket/
```

## Testing Strategy

### Unit Tests (Domain)

Test business logic in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_item_creation() {
        let item = MemoryItem::new("test content", "test/path");
        assert!(!item.id.is_empty());
    }
}
```

### Integration Tests (Adapters)

Test adapter implementations:

```rust
#[tokio::test]
async fn test_surreal_add_and_retrieve() {
    let adapter = SurrealMemoryAdapter::new().await;
    let id = adapter.add(test_item()).await.unwrap();
    let retrieved = adapter.get(&id).await.unwrap();
    assert_eq!(retrieved.content, test_item().content);
}
```

### E2E Tests (API)

Test full request/response cycle:

```bash
# Using the test framework
cargo test --test e2e
```

## Next Steps

- [Memory Module](/modules/memory/) - Deep dive into memory
- [Belief Graph](/modules/belief-graph/) - Knowledge representation
- [Testing Strategy](/testing/overview/) - Comprehensive testing guide
