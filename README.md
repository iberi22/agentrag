# Cortex - Sistema de Memoria con IA

## Versión: 0.2.0 (En desarrollo)

Sistema de memoria empresarial con búsqueda semántica, basado en la arquitectura Context Mode MCP.

---

## Características

### Fase 1: FTS5 Keyword Search ✅
- Búsqueda de texto completo con SQLite FTS5
- Indexación optimizada de keywords
- Mejora en la tokenización

### Fase 2: Embedding Integration ✅
- Integración con pplx-embed (localhost:8002)
- Búsqueda semántica usando embeddings
- Similitud coseno para encontrar documentos similares
- Búsqueda híbrida (keywords + embeddings)

### Fase 3: Checkpoint System ✅
- Continuidad de sesiones
- Checkpoints < 2KB
- Tracking de file edits
- Tracking de operaciones Git
- Decisiones clave registradas

### Fase 4: Optimización ✅
- Cache de embeddings
- Mejoras de rendimiento
- Cobertura de tests

---

## Uso

### Iniciar el servidor

```bash
docker run -d --name cortex \
  -p 8003:8003 \
  -e CORTEX_TOKEN=dev \
  -v ./data:/data \
  cortex:0.2.0
```

### Agregar memoria

```bash
curl -X POST http://localhost:8003/memory/add \
  -H "Content-Type: application/json" \
  -H "X-Cortex-Token: dev" \
  -d '{
    "content": "Información sobre el proyecto...",
    "path": "proyectos/mi-proyecto",
    "metadata": {"tipo": "documentación"}
  }'
```

### Buscar memoria

```bash
curl -X POST http://localhost:8003/memory/search \
  -H "Content-Type: application/json" \
  -H "X-Cortex-Token: dev" \
  -d '{
    "query": "tu pregunta",
    "limit": 5
  }'
```

---

## API Reference

### Endpoints

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | /health | Health check |
| POST | /memory/add | Agregar memoria |
| POST | /memory/search | Buscar memorias |
| POST | /memory/query | Query con IA |
| POST | /code/scan | Escanear código |
| POST | /code/find | Buscar en código |
| GET | /code/stats | Estadísticas |
| GET | /memory/graph | Grafo de memorias |

---

## Arquitectura

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

---

## Desarrollo

### Requisitos
- Rust
- Docker
- SQLite

### Build
```bash
cargo build --release
docker build -t cortex:0.2.0 .
```

### Tests
```bash
cargo test
```

---
inspired by: - https://arxiv.org/html/2402.17753v1
            - https://github.com/karpathy/autoresearch



## Licencia

MIT - Southwest AI Labs
