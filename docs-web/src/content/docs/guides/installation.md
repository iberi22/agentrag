---
title: Installation
description: How to install and configure Cortex
---

# Installation Guide

## Prerequisites

- **Rust** 1.70+ with cargo
- **SurrealDB** (optional, can use embedded)
- **OpenSSL** development libraries
- **Node.js** 18+ (for documentation)

## Build from Source

### 1. Clone the Repository

```bash
git clone https://github.com/southwest-ai-labs/cortex.git
cd cortex
```

### 2. Build the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CORTEX_PORT` | `8003` | HTTP server port |
| `CORTEX_HOST` | `0.0.0.0` | Bind address |
| `CORTEX_DB_PATH` | `./data/cortex.db` | Database path |
| `CORTEX_TOKEN` | `dev-token` | API authentication |
| `CORTEX_LOG_LEVEL` | `info` | Logging verbosity |

### Configuration File

Create `config.yaml`:

```yaml
server:
  host: 0.0.0.0
  port: 8003
  cors:
    allowed_origins:
      - "*"

database:
  path: ./data/cortex.db
  embedded: true

memory:
  max_context_tokens: 200000
  vector_dim: 1536

logging:
  level: info
  format: json
```

## Running Cortex

### Development Mode

```bash
cargo run -- dev
```

### Production Mode

```bash
# Build release
cargo build --release

# Run with config
./target/release/cortex serve --config config.yaml
```

### Docker

```bash
# Pull image
docker pull southwestailabs/cortex:latest

# Run container
docker run -p 8003:8003 -v cortex-data:/data \
  -e CORTEX_TOKEN=your-token \
  southwestailabs/cortex:latest
```

## Verify Installation

```bash
# Health check
curl http://localhost:8003/health

# Expected response:
# {"status":"ok","version":"0.1.0"}
```

## Next Steps

- [Quick Start Guide](/guides/quick-start/) - Run your first query
- [Architecture Overview](/architecture/overview/) - Understand the design
