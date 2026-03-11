# Changelog

All notable changes to Cortex will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - 2026-03-11

### Added
- **Web Dashboard UI**: Production-ready HTML/CSS/JS interface
- **Docker Support**: Multi-stage Dockerfile and docker-compose.yml
- **Authentication Module**: JWT-based auth with RBAC
- **User Management**: Admin panel for user CRUD
- **Rate Limiting**: Per-user API rate limiting
- **Memory CRUD**: Full create, read, delete operations
- **Belief Graph Visualization**: Node and edge management
- **Agent Runtime**: System 1-2-3 agent execution
- **Code Indexing**: Source code search functionality
- **MCP Server**: Model Context Protocol integration

### Changed
- Renamed from AgentRAG to Cortex
- Improved memory search with hybrid BM25 + vector
- Updated API endpoints with consistent response format

### Fixed
- Memory persistence issues
- API token validation
- CORS configuration

---

## [0.1.0] - 2026-01-15

### Added
- **Initial Release**
- **Memory Store**: Basic document storage
- **Search**: Simple text search
- **HTTP Server**: Axum-based REST API
- **Basic Auth**: API token authentication
- **SurrealDB Integration**: Vector storage

### Known Issues
- Limited memory capacity
- No belief graph visualization
- No agent coordination
- Basic error handling

---

## [Unreleased]

### Planned for 0.3.0
- [ ] WebSocket support for real-time updates
- [ ] Advanced belief graph analytics
- [ ] Multi-agent coordination
- [ ] Plugin system
- [ ] Advanced RBAC with teams
- [ ] Audit logging
- [ ] API versioning

---

## Migration Guide

### From 0.1.0 to 0.2.0

1. **API Changes**:
   - Endpoint `/memory` renamed to `/memory/add`
   - New endpoint `/memory/search`
   - New endpoint `/memory/delete`

2. **Configuration**:
   - Add `CORTEX_TOKEN` environment variable
   - Configure database path in `config.yaml`

3. **Docker**:
   - Use new `docker-compose.yml`
   - Update image name to `iberi22/cortex:latest`

---

## Deprecated

| Feature | Deprecated in | Removed in |
|---------|---------------|------------|
| `/memory` endpoint | 0.2.0 | 0.3.0 |
| Basic auth (token only) | 0.2.0 | 0.3.0 |

---

## Statistics

- **Total Releases**: 2
- **Contributors**: 1 (Belal/iberi22)
- **First Release**: 2026-01-15
- **Days Since First Release**: 55
