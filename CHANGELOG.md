# Changelog - Cortex Project

## [Unreleased] - 2026-03-13

### Added
- Phase 1: FTS5 Keyword Search Improvement
  - Enhanced keyword indexing
  - Better tokenization for search
  - Fixed empty search results bug

- Phase 2: Embedding Integration
  - pplx-embed integration (localhost:8002)
  - Semantic search using embeddings
  - Cosine similarity for document matching
  - Hybrid search (keywords + embeddings)

- Phase 3: Checkpoint System
  - Session continuity support
  - Checkpoint struct (< 2KB)
  - File edits tracking
  - Git operations tracking
  - Tasks and key decisions

- Phase 4: Optimization
  - Embedding caching
  - Performance improvements
  - Test coverage

### Changed
- Code Graph: Added TypeScript/JavaScript support
- Memory API: Better error handling

### Fixed
- Code Graph indexing only supporting Rust files
- Memory search returning empty results

---

## [0.1.0] - 2026-03-12

### Added
- Initial release
- Memory system (QmdMemory)
- Code Graph indexing
- REST API endpoints
- Docker container

### Features
- /health - Health check
- /memory/add - Add memory
- /memory/search - Search memories
- /code/scan - Scan code
- /code/find - Find in code

---

## Architecture

### System Components
```
┌─────────────────────────────────────────────┐
│              CORTEX API                     │
├─────────────────────────────────────────────┤
│  ┌─────────────┐   ┌──────────────────┐   │
│  │  Keyword    │   │   Embedding      │   │
│  │  Search     │   │   Search         │   │
│  │  (FTS5)     │   │   (Similitud)   │   │
│  └─────────────┘   └──────────────────┘   │
│         │                   │               │
│         └─────────┬─────────┘               │
│                   ▼                          │
│         ┌─────────────────┐               │
│         │  Fusion/Rerank  │               │
│         │  (Best Results)  │               │
│         └─────────────────┘               │
│                   │                          │
│         ┌────────▼────────┐               │
│         │   Checkpoint     │               │
│         │   System         │               │
│         └──────────────────┘               │
└─────────────────────────────────────────────┘
```

### API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| POST | /memory/add | Add memory document |
| POST | /memory/search | Search memories |
| POST | /memory/query | Query with AI |
| POST | /code/scan | Scan code |
| POST | /code/find | Find in code |
| GET | /code/stats | Code statistics |
| GET | /memory/graph | Memory graph |

### Authentication
- Header: `X-Cortex-Token: dev`

---

## Contributors

- SWAL (Southwest AI Labs)
