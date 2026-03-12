---
title: Quick Start
description: Get started with Cortex in 5 minutes
---

# Quick Start Guide

Get Cortex running and make your first API call in under 5 minutes.

## Start the Server

```bash
# From the cortex directory
cargo run -- serve
```

You should see:

```
🚀 Cortex starting...
   Version: 0.1.0
   Port: 8003
   Mode: development
✅ Server ready at http://localhost:8003
```

## Your First API Call

### 1. Add Memory

```bash
curl -X POST http://localhost:8003/memory/add \
  -H "X-Cortex-Token: dev-token" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Cortex is a cognitive memory system for AI agents",
    "path": "test/intro",
    "metadata": {"tags": ["ai", "memory"]}
  }'
```

**Response:**

```json
{
  "success": true,
  "id": "memory_abc123",
  "path": "test/intro"
}
```

### 2. Search Memory

```bash
curl -X POST http://localhost:8003/memory/search \
  -H "X-Cortex-Token: dev-token" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "cognitive memory system",
    "limit": 5
  }'
```

**Response:**

```json
{
  "results": [
    {
      "content": "Cortex is a cognitive memory system for AI agents",
      "path": "test/intro",
      "score": 0.95,
      "metadata": {"tags": ["ai", "memory"]}
    }
  ]
}
```

### 3. Query with LLM

```bash
curl -X POST http://localhost:8003/memory/query \
  -H "X-Cortex-Token: dev-token" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is Cortex?",
    "context": "You are a helpful AI assistant"
  }'
```

## Using the Web UI

Cortex includes a web interface:

```
http://localhost:8003/ui
```

Features:
- Visual memory browser
- Belief graph visualization
- Query playground
- System diagnostics

## Integrate with OpenClaw

Add to your OpenClaw config:

```json
{
  "tools": {
    "mcp": {
      "servers": {
        "cortex": {
          "enabled": true,
          "url": "http://localhost:8003/mcp"
        }
      }
    }
  }
}
```

## Next Steps

- [Architecture Overview](/architecture/overview/) - Deep dive
- [Memory Module](/modules/memory/) - Understand memory operations
- [Testing](/testing/overview/) - Verify your setup
