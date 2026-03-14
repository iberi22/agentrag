---
name: cortex-memory
description: Use Cortex cognitive memory system as MCP backend for persistent agent state, experiment logs, and knowledge synthesis.
---

# Cortex Memory MCP Skill

## Overview

Cortex is a cognitive memory system running in Docker that provides hybrid search (BM25 + vectors), belief graphs, and session persistence via SurrealDB. This skill describes how to use Cortex as an MCP server for AI agent workflows.

## Prerequisites

- Cortex must be running in Docker: `docker compose up -d` from `e:\scripts-python\cortex`
- SurrealDB must be healthy (dependency of Cortex container)
- Default endpoint: `http://localhost:8003`

## Health Check

Before using Cortex memory, verify the service is running:

```bash
curl http://localhost:8003/health
```

Expected response: `200 OK` with system status.

## MCP Configuration

Add Cortex as an MCP server in your agent configuration:

```json
{
  "mcpServers": {
    "cortex-memory": {
      "url": "http://localhost:8003/mcp",
      "transport": "streamable-http"
    }
  }
}
```

## Available MCP Tools

### Memory Operations

| Tool | Description | Parameters |
|------|-------------|------------|
| `cortex_search` | Hybrid search (BM25 + semantic) | `query: string`, `mode: hybrid\|semantic\|keyword` |
| `cortex_store` | Store a memory document | `content: string`, `metadata: object` |
| `cortex_get` | Retrieve document by ID | `id: string` |
| `cortex_update` | Update existing document | `id: string`, `content: string` |
| `cortex_delete` | Delete a document | `id: string` |

### Graph Operations

| Tool | Description | Parameters |
|------|-------------|------------|
| `cortex_graph_query` | Query the belief graph | `query: string` |
| `cortex_graph_add_relation` | Add entity relationship | `subject: string`, `predicate: string`, `object: string` |

### Agent Operations

| Tool | Description | Parameters |
|------|-------------|------------|
| `cortex_agent_run` | Execute a Cortex agent (System 1/2/3) | `query: string`, `agent_type: string` |

## Usage Pattern for Evolve Module

The Evolve Module uses Cortex MCP to persist its state:

```
1. Store experiment hypothesis → cortex_store(content, {type: "hypothesis"})
2. After experiment → cortex_store(results, {type: "experiment_result", hypothesis_id: "..."})
3. Query past experiments → cortex_search("memory optimization", mode: "hybrid")
4. Build knowledge graph → cortex_graph_add_relation(technique, "improves", metric)
5. Retrieve evolution history → cortex_search("", {type: "experiment_result", status: "keep"})
```

## API Endpoints (REST fallback)

If MCP is not available, use REST API directly:

```
POST http://localhost:8003/memory/search   - Hybrid search
POST http://localhost:8003/memory/query    - Query with reasoning  
GET  http://localhost:8003/memory/{id}     - Get document
POST http://localhost:8003/memory/         - Create document
PUT  http://localhost:8003/memory/{id}     - Update document
DELETE http://localhost:8003/memory/{id}   - Delete document
GET  http://localhost:8003/memory/graph    - Get belief graph
POST http://localhost:8003/memory/graph/rel - Add relation
```

## Docker Compose Reference

Cortex services defined in `docker-compose.yml`:
- `cortex` — Main memory server (port 8003)
- `surrealdb` — Graph/vector database (port 8000)
- `prometheus` — Metrics (port 9090, monitoring profile)
- `grafana` — Dashboards (port 3001, monitoring profile)

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CORTEX_PORT` | 8003 | HTTP API port |
| `CORTEX_TOKEN` | dev-token | Auth token |
| `SURREALDB_URL` | ws://surrealdb:8000 | SurrealDB connection |
| `SURREALDB_USER` | root | DB username |
| `SURREALDB_PASS` | root | DB password |
