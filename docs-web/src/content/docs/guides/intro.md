---
title: Introduction
description: Welcome to Cortex - Cognitive Memory for AI Swarms
---

# Cortex Documentation

> **Cortex** is a cognitive memory system designed for AI agent swarms, implementing a hexagonal architecture for maximum modularity and testability.

## What is Cortex?

Cortex is a **distributed memory system** that provides:

- **Belief Graph**: Knowledge representation as a directed graph
- **Vector Storage**: Semantic search capabilities
- **Session Management**: Persistent conversation context
- **File Indexing**: Code and document search
- **Multi-Agent Coordination**: Swarm memory synchronization

## Architecture Overview

Cortex follows **Hexagonal Architecture** (Ports & Adapters):

```
┌─────────────────────────────────────────────────────────────┐
│                        API Layer                             │
│              (HTTP Server, MCP Protocol)                    │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    Application Layer                         │
│              (Use Cases, Commands, Queries)                 │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                      Domain Layer                            │
│         (Memory, BeliefGraph, Sessions, Tasks)              │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                  Infrastructure Layer                       │
│      (SurrealDB, File System, Vector Store)                │
└─────────────────────────────────────────────────────────────┘
```

## Core Modules

| Module | Description | Status |
|--------|-------------|--------|
| `memory` | Core memory operations | ✅ Stable |
| `belief_graph` | Knowledge graph | ✅ Stable |
| `server` | HTTP/MCP servers | ✅ Stable |
| `agents` | Agent coordination | 🔄 Evolving |
| `tasks` | Task management | 🔄 Evolving |
| `scheduler` | Job scheduling | 🆕 New |

## Quick Links

- [Installation Guide](/guides/installation/)
- [Quick Start](/guides/quick-start/)
- [Architecture Details](/architecture/overview/)
- [API Reference](/reference/api/)
- [Testing Strategy](/testing/overview/)

## Tech Stack

- **Runtime**: Rust (async)
- **Database**: SurrealDB (embedded)
- **Search**: BM25 + Vector embeddings
- **Protocol**: MCP (Model Context Protocol)
- **API**: REST + WebSocket
