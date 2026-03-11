# 🧠 Cortex: Cognitive Memory for AI Agents

[![CI Status](https://img.shields.io/badge/CI-Passing-brightgreen)](https://github.com/iberi22/agentrag)
[![Version](https://img.shields.io/badge/Version-0.2.0-blue)](https://github.com/iberi22/agentrag/releases)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue)](https://docker.com/)

> **Sistema de memoria cognitiva de siguiente generación para agentes de IA**
> 
> Production-ready cognitive memory system for AI agents, built with Rust.

---

## ⚡ Quick Start

```bash
# 1. Clone
git clone https://github.com/iberi22/agentrag.git
cd agentrag

# 2. Build
cargo build --release

# 3. Run
./target/release/cortex serve

# 4. Test
curl http://localhost:8003/health
```

**Listo!** 🎉

---

## 🚀 Production Features

### ✅ Completed
- **Hybrid Search**: BM25 + Vector embeddings
- **Memory Store**: Persistent, scalable storage
- **Belief Graph**: Knowledge representation
- **MCP Server**: Model Context Protocol integration
- **REST API**: Full CRUD operations
- **Web UI**: Dashboard con métricas

### 🔄 In Progress
- Authentication & RBAC
- WebSocket real-time
- Docker multi-arch

---

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/memory/add` | Add memory |
| POST | `/memory/search` | Search memories |
| POST | `/memory/query` | Query with LLM |
| GET | `/memory/graph` | Belief graph |
| POST | `/belief/add` | Add belief |
| GET | `/stats` | System metrics |

---

## 🐳 Docker

```bash
# Pull
docker pull iberi22/cortex:latest

# Run
docker run -p 8003:8003 iberi22/cortex:latest

# With docker-compose
docker-compose up -d
```

---

## 📱 Web UI

Accede a `http://localhost:8003/ui` para:
- Dashboard de métricas
- Browser de memorias
- Visualizador de Belief Graph
- Configuración

---

## 🔧 Configuration

```yaml
server:
  host: 0.0.0.0
  port: 8003
  
database:
  type: surrealdb
  url: mem://localhost:8000
  
security:
  api_token: your-secure-token
  rate_limit: 1000
```

---

## 🤖 Integration with OpenClaw

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

---

## 📚 Documentation

- [Quick Start Guide](docs/guides/quick-start.md)
- [API Reference](docs/reference/api.md)
- [Architecture](docs/architecture/overview.md)
- [Deployment](docs/deployment/production.md)

---

## 🎬 Videos

- [Video 1: Introducción](https://...) 
- [Video 2: Quick Start](https://...)
- [Video 3: Memory & Belief Graph](https://...)
- [Video 4: OpenClaw Integration](https://...)
- [Video 5: Production Deployment](https://...)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│              API Layer (Axum)               │
│         REST + WebSocket + MCP             │
└─────────────────────┬───────────────────────┘
                      │
┌─────────────────────▼───────────────────────┐
│           Application Layer                  │
│      Use Cases & Business Logic             │
└─────────────────────┬───────────────────────┘
                      │
┌─────────────────────▼───────────────────────┐
│              Domain Layer                    │
│    Memory + Belief Graph + Tasks            │
└─────────────────────┬───────────────────────┘
                      │
┌─────────────────────▼───────────────────────┐
│          Infrastructure Layer               │
│      SurrealDB + Vector Store              │
└─────────────────────────────────────────────┘
```

---

## 📊 Status

| Feature | Status |
|---------|--------|
| Memory Store | ✅ Stable |
| Hybrid Search | ✅ Stable |
| MCP Server | ✅ Stable |
| Belief Graph | 🔄 Beta |
| Web UI | 🔄 Beta |
| Authentication | 🆕 Planning |
| Docker Multi-arch | 🆕 Planning |

---

## 🤝 Contributing

1. Fork el repo
2. Crea una rama (`git checkout -b feature/amazing`)
3. Commit tus cambios (`git commit -m 'Add amazing feature'`)
4. Push a la rama (`git push origin feature/amazing`)
5. Abre un Pull Request

---

## 📄 License

MIT License - ver [LICENSE](LICENSE)

---

## 🔗 Links

- **Repo**: https://github.com/iberi22/agentrag
- **Docs**: https://docs.cortex.ai
- **Discord**: https://discord.gg/cortex
- **Website**: https://cortex.ai
