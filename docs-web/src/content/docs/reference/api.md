---
title: API Reference
description: Complete API endpoints for Cortex
---

# API Reference

Complete reference for all Cortex HTTP endpoints.

## Base URL

```
http://localhost:8003
```

## Authentication

All endpoints (except health) require the `X-Cortex-Token` header:

```bash
curl -H "X-Cortex-Token: dev-token" ...
```

## Endpoints Overview

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/memory/add` | Add memory |
| POST | `/memory/search` | Search memories |
| POST | `/memory/query` | Query with LLM |
| GET | `/memory/:id` | Get memory by ID |
| DELETE | `/memory/:id` | Delete memory |
| GET | `/memory/graph` | Get belief graph |
| POST | `/belief/add` | Add belief |
| GET | `/session` | List sessions |
| POST | `/session` | Create session |

## Health

### GET /health

Check if the server is running.

**Response:**

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

## Memory

### POST /memory/add

Add a new memory item.

**Request:**

```json
{
  "content": "Memory content",
  "path": "project/context",
  "metadata": {
    "tags": ["tag1", "tag2"]
  }
}
```

**Response:**

```json
{
  "success": true,
  "id": "memory_abc123",
  "path": "project/context"
}
```

### POST /memory/search

Search memories by similarity.

**Request:**

```json
{
  "query": "search term",
  "limit": 10,
  "path_filter": "optional/path"
}
```

**Response:**

```json
{
  "results": [
    {
      "id": "memory_abc123",
      "content": "Memory content",
      "path": "project/context",
      "score": 0.95,
      "metadata": {}
    }
  ]
}
```

### POST /memory/query

Query memory using LLM.

**Request:**

```json
{
  "query": "What is Cortex?",
  "context": "You are a helpful AI assistant",
  "max_tokens": 500
}
```

**Response:**

```json
{
  "answer": "Cortex is a cognitive memory system...",
  "sources": [
    {
      "id": "memory_abc123",
      "content": "Cortex description"
    }
  ]
}
```

### GET /memory/:id

Get a specific memory by ID.

**Response:**

```json
{
  "id": "memory_abc123",
  "content": "Memory content",
  "path": "project/context",
  "metadata": {},
  "created_at": "2026-03-11T12:00:00Z",
  "updated_at": "2026-03-11T12:00:00Z"
}
```

### DELETE /memory/:id

Delete a memory.

**Response:**

```json
{
  "success": true
}
```

## Belief Graph

### GET /memory/graph

Get the belief graph visualization.

**Response:**

```json
{
  "nodes": [
    {"id": "node1", "label": "Cortex", "type": "concept"},
    {"id": "node2", "label": "Memory", "type": "concept"}
  ],
  "edges": [
    {"from": "node1", "to": "node2", "relation": "has"}
  ]
}
```

### POST /belief/add

Add a belief to the graph.

**Request:**

```json
{
  "subject": "Cortex",
  "predicate": "is_a",
  "object": "Memory System",
  "confidence": 0.95
}
```

## Sessions

### GET /session

List all sessions.

**Response:**

```json
{
  "sessions": [
    {
      "id": "session_abc123",
      "name": "Project Alpha",
      "created_at": "2026-03-11T12:00:00Z",
      "last_active": "2026-03-11T14:00:00Z"
    }
  ]
}
```

### POST /session

Create a new session.

**Request:**

```json
{
  "name": "New Session",
  "context": "optional context"
}
```

## Errors

All errors follow this format:

```json
{
  "error": {
    "type": "error_type",
    "message": "Human readable message",
    "code": 404
  }
}
```

### Error Types

| Type | Code | Description |
|------|------|-------------|
| `unauthorized` | 401 | Invalid or missing token |
| `not_found` | 404 | Resource not found |
| `validation_error` | 400 | Invalid request body |
| `rate_limit` | 429 | Too many requests |
| `internal_error` | 500 | Server error |

## Rate Limiting

Rate limiting is applied per API token:

- **Default**: 100 requests/minute
- **Authenticated**: 1000 requests/minute

Headers returned:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`
