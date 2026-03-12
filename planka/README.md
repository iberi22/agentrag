# Planka + Cortex Integration

Sistema de kanban para Southwest AI Labs integrado con Cortex.

## Servicios

| Servicio | Puerto | URL |
|----------|--------|-----|
| Planka (UI) | 3000 | http://localhost:3000 |
| kanban-mcp | 8081 | http://localhost:8081 |

## Quick Start

```bash
cd E:\scripts-python\cortex\planka
setup.bat
```

## Credenciales Iniciales

- **Email:** admin@swal.ai
- **Password:** admin123

> ⚠️ Cambiar password en producción

## Boards Recomendados

Crear los siguientes boards en Planka:

1. **Cortex** - Desarrollo del sistema de memoria
2. **ZeroClaw** - Runtime Rust
3. **Trading Bot** - Estrategias de trading
4. **ManteniApp** - SaaS mantenimiento
5. **Research** - Investigación
6. **Ops** - Infra y DevOps

## Etiquetas

| Etiqueta | Color | Uso |
|----------|-------|-----|
| urgent | 🔴 Rojo | Prioridad alta |
| blocked | 🟡 Amarillo | Bloqueado |
| done | 🟢 Verde | Completado |
| dev | 🔵 Azul | Desarrollo |
| research | 🟣 Púrpura | Investigación |

## API Endpoints (kanban-mcp)

Una vez corriendo, los agentes pueden usar:

```bash
# Listar boards
curl http://localhost:8081/boards

# Crear tarea
curl -X POST http://localhost:8081/tasks \
  -H "Content-Type: application/json" \
  -d '{"boardId": "...", "name": "Nueva tarea", "description": "..."}'

# Mover tarea
curl -X PUT http://localhost:8081/tasks/{id}/move \
  -H "Content-Type: application/json" \
  -d '{"position": 0, "columnId": "nueva-columna"}'
```

## Integración con Cortex

En Cortex, agregar como tool:

```json
{
  "type": "http",
  "url": "http://localhost:8081",
  "methods": ["GET", "POST", "PUT", "DELETE"]
}
```

## Comandos Docker

```bash
# Iniciar
docker-compose up -d

# Ver logs
docker-compose logs -f

# Parar
docker-compose down

# Reiniciar
docker-compose restart
```

## Troubleshooting

### Planka no responde
```bash
docker-compose logs planka
```

### kanban-mcp no conecta
```bash
docker-compose logs kanban-mcp
```

### Reiniciar todo
```bash
docker-compose down && docker-compose up -d
```
