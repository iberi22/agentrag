# CORTEX - Sistema de Memoria Cognitiva

> **"Un sistema de memoria cognitiva para agentes de IA"**
>
> Open source cognitive memory system for AI agents.
> Fork de AgentRAG - ahora con implementación completa.

## Vision

Cortex es un **sistema de memoria cognitiva** que permite a los agentes de IA:
- Recordar conversaciones pasadas
- Mantener contexto a largo plazo
- Razonar sobre relaciones entre conceptos
- Operar de forma independiente con memoria integrada

## Diferencia con otros sistemas

| Sistema | Enfoque | Memoria |
|---------|---------|---------|
| OpenClaw actual | Agente → + Memoria | Externa (Tier 1-3) |
| **Cortex** | **Memoria → + Agente** | **Integrada desde el core** |

## Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                        CORTEX CORE                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Belief    │  │   Memory    │  │     Orchestrator    │  │
│  │   Graph     │  │   Store     │  │     (Qwen CLI)      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Plugin    │  │   Agent     │  │       HTTP API       │  │
│  │   Manager   │  │   Registry  │  │      (MCP Server)    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Módulos (Rust)

### 1. memory/ - Sistema de Memoria

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| `surreal_store.rs` | Storage unificado (SurrealDB) | ✅ |
| `qmd_memory.rs` | Búsqueda híbrida (BM25 + vectores) | ✅ |
| `belief_graph.rs` | Grafo de relaciones conceptuales | 🔄 |
| `session_store.rs` | Memoria de sesiones | 🔄 |
| `file_indexer.rs` | Indexador de archivos markdown | 🔄 |
| `cache.rs` | Cache de embeddings | 🔄 |

### 2. agents/ - Sistema de Agentes

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| `system1.rs` | Agente de retrieval (búsqueda) | ✅ |
| `system2.rs` | Agente de reasoning (razonamiento) | ✅ |
| `system3.rs` | Agente de action (ejecución) | ✅ |
| `registry.rs` | Registro de agentes | ✅ |
| `runtime.rs` | Runtime de ejecución | ✅ |

### 3. tools/ - Herramientas

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| `search_tools.rs` | Herramientas de búsqueda | ✅ |
| `memory_tools.rs` | CRUD de memoria | 🔄 |
| `reasoning_tools.rs` | Herramientas de razonamiento | 🔄 |
| `mcp_tools.rs` | Herramientas MCP | 🔄 |

### 4. server/ - Servidor

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| `http_server.rs` | API REST HTTP | 🔄 |
| `mcp_server.rs` | Server MCP (OpenClaw) | 🔄 |
| `websocket.rs` | WebSocket para streaming | 🔄 |

### 5. plugins/ - Sistema de Plugins

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| `manager.rs` | Plugin Manager | 🔄 |
| `loader.rs` | Loader dinámico | 🔄 |
| `registry.rs` | Registro de plugins | 🔄 |

## API Endpoints

### Memoria

```
POST /memory/search     - Búsqueda híbrida
POST /memory/query      - Query con reasoning
GET  /memory/{id}       - Obtener documento
POST /memory/           - Crear documento
PUT  /memory/{id}       - Actualizar documento
DELETE /memory/{id}    - Eliminar documento
GET  /memory/graph      - Obtener belief graph
POST /memory/graph/rel - Agregar relación
```

### Agentes

```
POST /agents/run        - Ejecutar agente
GET  /agents/list       - Listar agentes
POST /agents/register   - Registrar agente
GET  /agents/{id}/state - Estado del agente
```

### Sistema

```
GET  /health            - Health check
GET  /status            - Estado del sistema
POST /sync              - Sincronizar con Tier 1
```

## Integración con OpenClaw

Cortex funciona como **MCP Server** que OpenClaw puede llamar:

```json
{
  "name": "cortex-memory",
  "tools": [
    {
      "name": "cortex_search",
      "description": "Buscar en la memoria cognitiva",
      "input_schema": {
        "type": "object",
        "properties": {
          "query": {"type": "string"},
          "mode": {"type": "string", "enum": ["hybrid", "semantic", "keyword"]}
        }
      }
    }
  ]
}
```

## Stack

- **Lenguaje**: Rust (Tokio async)
- **Database**: SurrealDB (embedded)
- **Embeddings**: Qwen CLI / Ollama
- **Search**: BM25 (tantivy) + vectores
- **API**: Axum (HTTP) + MCP

## Roadmap

### Fase 1: Core (Ahora)
- [x] SurrealDB store
- [x] QMD memory (búsqueda híbrida)
- [ ] Belief Graph completo
- [ ] Session store
- [ ] File indexer

### Fase 2: Agentes
- [x] System 1 (retrieval)
- [x] System 2 (reasoning)
- [x] System 3 (action)
- [x] Runtime orchestration

### Fase 3: Integración
- [ ] HTTP Server
- [ ] MCP Server
- [ ] Plugin system

### Fase 4: Producto
- [ ] CLI tool
- [ ] Docker image
- [ ] Docs
- [ ] GitHub repo

## Nombre Anterior

- AgentRAG → Cortex

## Flujo de Ejecución RAG (Pipeline Determinista)
A diferencia de agentes LLM orquestadores de propósito general (ej. AutoGPT) que delegan la decisión de continuar o detenerse en el propio modelo, Cortex utiliza un pipeline determinista en la asimilación y ejecución de la memoria en tres fases:
1. **System 1 (Retrieval)**: Busca conocimiento semántico e híbrido (BM25 + Vec). 
2. **System 2 (Reasoning)**: Analiza el contexto. Extrae inferencias lógicas.
3. **System 3 (Action)**: Toma la decisión o formula la respuesta basada en el razonamiento de System 2.

**Condición de Parada**: El agente finaliza su ciclo de vida y recobra latencia tan pronto como concluye el procesamiento de System 3, encapsulando la respuesta generada con su **nivel de confianza**. En casos de bucles computacionales indeseados del modelo, el manejador `AgentRuntime` hace cumplir _timeouts_ duros obligando al agente a expirar el flujo actual.

## Links

- GitHub: (pendiente crear)
- Docs: cortex.openclaw.ai (pendiente)
- npm: @cortex-ai/core (pendiente)
