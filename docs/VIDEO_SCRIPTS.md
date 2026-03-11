# Video Scripts - Cortex

## Video 1: Introducción a Cortex (5 min)

### Intro (30s)
- [VISUAL: Logo Cortex + tagline]
- "Bienvenidos a Cortex, el sistema de memoria cognitiva para agentes de IA"

### What is Cortex? (1min)
- [VISUAL: Diagrama arquitectura]
- Explicar: memoria + belief graph + MCP
- Casos de uso: OpenClaw, agentes, RAG

### Features principales (2min)
- [VISUAL: Demo de features]
- Hybrid Search (BM25 + Vector)
- Belief Graph
- MCP Server
- System 1-2-3

### Demo quick (1min)
- [VISUAL: Terminal + curl]
- Mostrar API en acción

### Outro (30s)
- [VISUAL: Links]
- Repo, docs, Discord

---

## Video 2: Quick Start (7 min)

### Intro (30s)
- "En este video, instalamos y usamos Cortex en 5 minutos"

### Prerequisites (1min)
- Rust instalado
- Docker (opcional)

### Installation (2min)
- [VISUAL: Terminal]
```bash
git clone https://github.com/iberi22/agentrag.git
cd agentrag
cargo build --release
```

### Configuration (1min)
- [VISUAL: Config file]
- Puerto, token, database

### First API calls (2min)
- [VISUAL: Terminal]
```bash
# Health check
curl http://localhost:8003/health

# Add memory
curl -X POST http://localhost:8003/memory/add ...

# Search
curl -X POST http://localhost:8003/memory/search ...
```

### Wrap up (30s)
- Próximos pasos

---

## Video 3: Memory & Belief Graph (10 min)

### Intro (1min)
- "Cortex no es solo storage, es memoria cognitiva"

### Memory System (3min)
- [VISUAL: Diagramas]
- QMD Memory
- Hybrid Search
- Metadata y tags

### Belief Graph (4min)
- [VISUAL: Graph visualization]
- Nodos y edges
- Confidence
- Traversals

### Demo (2min)
- [VISUAL: Terminal]
- Crear memories
- Crear beliefs
- Consultar

---

## Video 4: Integración con OpenClaw (7 min)

### Intro (1min)
- "Cortex es el cerebro de OpenClaw"

### OpenClaw setup (2min)
- [VISUAL: Config]
- Configurar MCP

### Integration demo (3min)
- [VISUAL: Bot en acción]
- Memoria persistente
- Contexto

### Benefits (1min)
- Por qué usar Cortex

---

## Video 5: Production Deployment (10 min)

### Intro (1min)
- "Cómo desplegar Cortex en producción"

### Docker (3min)
- [VISUAL: Docker files]
- Multi-stage build
- docker-compose

### Security (2min)
- Authentication
- Rate limiting
- SSL/TLS

### Monitoring (2min)
- [VISUAL: Dashboard]
- Métricas
- Logs

### Backup & Recovery (2min)
- Estrategias
- Scripts
