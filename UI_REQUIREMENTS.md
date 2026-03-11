# Cortex UI Requirements
## Comprehensive UI/UX Specifications for Production

---

## 1. Dashboard Overview

### 1.1 Main Metrics Display

| Metric | Endpoint | Description |
|--------|----------|-------------|
| Total Memories | `/memory/count` | Total documents stored |
| Memories Today | `/memory/stats` | Documents added today |
| Belief Nodes | `/belief/count` | Total belief graph nodes |
| Active Agents | `/agents/list` | Currently running agents |
| Queries Today | `/stats/queries` | API queries in 24h |
| Avg Response Time | `/stats/latency` | Average response in ms |
| Uptime | `/health` | System uptime |
| DB Status | `/health` | Database connection status |

### 1.2 Quick Actions Panel

- ✅ Add Memory (opens modal)
- ✅ Search Memories (opens search panel)
- ✅ Add Belief (opens belief form)
- ✅ Run Agent (opens agent runner)
- ✅ View Analytics (opens charts)
- ✅ Settings (opens config)

### 1.3 Recent Activity Feed

- Last 20 memory additions
- Last 10 queries
- Agent executions
- Belief graph updates

---

## 2. Memory Management UI

### 2.1 Memory List View

**Features:**
- Paginated list (20 per page)
- Sort by: date, relevance, path
- Filter by: path, tags, date range
- Search within memories

**Columns:**
- Preview (first 100 chars)
- Path
- Tags
- Created/Updated date
- Actions (view, edit, delete)

### 2.2 Add/Edit Memory Modal

**Fields:**
| Field | Type | Required | Validation |
|-------|------|----------|------------|
| Content | Textarea | Yes | Max 100KB |
| Path | Text | Yes | Alphanumeric + /_- |
| Tags | Multi-select | No | Max 10 tags |
| Metadata | JSON | No | Valid JSON |

### 2.3 Memory Search

**UI Components:**
- Search input with auto-complete
- Filter panel (path, tags, date)
- Results list with highlighted matches
- Pagination

---

## 3. Belief Graph UI

### 3.1 Graph Visualization

**Features:**
- Interactive force-directed graph
- Zoom/pan controls
- Node selection
- Edge highlighting
- Filter by belief type
- Search nodes

**Node Display:**
- Label
- Belief type (color coded)
- Confidence score (size)
- Connection count

### 3.2 Add Belief Form

| Field | Type | Description |
|-------|------|-------------|
| Subject | Text | Entity name |
| Predicate | Select | Relation type |
| Object | Text | Target entity |
| Confidence | Slider | 0-100% |
| Notes | Textarea | Optional notes |

---

## 4. Agent Management UI

### 4.1 Agent List

| Column | Description |
|--------|-------------|
| Name | Agent identifier |
| Status | Running/Idle/Error |
| Type | System 1/2/3 |
| Memory Used | Context tokens |
| Last Run | Timestamp |

### 4.2 Agent Configuration

**Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| Name | Text | - | Agent identifier |
| Model | Select | MiniMax-M2.5 | LLM model |
| Max Tokens | Number | 4096 | Response limit |
| Temperature | Slider | 0.7 | Creativity |
| System Prompt | Textarea | - | Instructions |
| Tools | Multi-select | [] | Enabled tools |
| Memory Context | Toggle | On | Use Cortex memory |

### 4.3 Agent Runner

- Query input
- Streaming response display
- Tool execution log
- Context visualization

---

## 5. Configuration UI

### 5.1 Server Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Port | Number | 8003 | HTTP server port |
| Host | Text | 0.0.0.0 | Bind address |
| Log Level | Select | info | Logging verbosity |
| CORS Origins | Text | * | Allowed origins |

### 5.2 Database Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Type | Select | memory | Storage type |
| Path | Text | data/ | DB file path |
| Embeddings | Toggle | On | Vector embeddings |

### 5.3 Security Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| API Token | Password | dev-token | Auth token |
| JWT Secret | Password | - | JWT signing key |
| Rate Limit | Number | 1000 | Requests/minute |
| JWT Expiry | Number | 3600 | Seconds |

### 5.4 Model Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Default Model | Select | MiniMax-M2.5 | Primary LLM |
| API Key | Password | - | LLM API key |
| Max Tokens | Number | 4096 | Default limit |
| Temperature | Number | 0.7 | Default temperature |

---

## 6. Admin Panel

### 6.1 User Management

**Features:**
- User list (admin only)
- Create user
- Edit user (name, email, role)
- Delete user
- Reset password

**User Fields:**
| Field | Type | Role |
|-------|------|------|
| Email | Email | Required |
| Name | Text | Required |
| Role | Select | admin/user/readonly |
| Status | Toggle | Active/Inactive |
| API Key | Password | Generated |

### 6.2 Role Permissions

| Permission | Admin | User | Readonly |
|-----------|-------|------|----------|
| View Dashboard | ✅ | ✅ | ✅ |
| Search Memory | ✅ | ✅ | ✅ |
| Add Memory | ✅ | ✅ | ❌ |
| Delete Memory | ✅ | ✅ | ❌ |
| Manage Beliefs | ✅ | ✅ | ❌ |
| Run Agents | ✅ | ✅ | ❌ |
| View Config | ✅ | ✅ | ✅ |
| Edit Config | ✅ | ❌ | ❌ |
| Manage Users | ✅ | ❌ | ❌ |

### 6.3 Audit Logs

**Log Entry Fields:**
- Timestamp
- User
- Action
- Resource
- IP Address
- Status (success/fail)

**Filters:**
- Date range
- User
- Action type
- Resource

---

## 7. Settings & Preferences

### 7.1 User Preferences

| Setting | Type | Default |
|---------|------|---------|
| Theme | Select | Dark |
| Language | Select | English |
| Sidebar Collapsed | Toggle | Off |
| Notifications | Toggle | On |
| Compact Mode | Toggle | Off |

### 7.2 Theme Options

- **Dark** (default): #0f172a bg, #6366f1 accent
- **Light**: #ffffff bg, #4f46e5 accent
- **System**: Follow OS preference

---

## 8. API Documentation

### 8.1 Built-in API Docs

Access: `/api/docs` (Swagger UI)

**Endpoints to document:**
- All REST endpoints
- Request/response schemas
- Authentication requirements
- Example requests

---

## 9. Monitoring & Metrics

### 9.1 Real-time Metrics

| Metric | Update Interval | Visualization |
|--------|---------------|---------------|
| Requests/sec | 1s | Line chart |
| Response Time | 1s | Line chart |
| Error Rate | 1s | Percentage |
| Memory Usage | 5s | Gauge |
| CPU Usage | 5s | Gauge |

### 9.2 Historical Charts

- Requests over time (24h, 7d, 30d)
- Response time distribution
- Error breakdown by type
- Popular endpoints

---

## 10. File Structure for Implementation

```
web/
├── index.html          # Main app
├── css/
│   ├── variables.css  # Theme variables
│   ├── components.css # UI components
│   └── charts.css     # Chart styles
├── js/
│   ├── api.js         # API client
│   ├── components.js  # UI components
│   ├── charts.js      # Chart rendering
│   └── router.js      # SPA routing
└── components/
    ├── sidebar.html
    ├── header.html
    ├── modal.html
    ├── dashboard.html
    ├── memory-list.html
    ├── belief-graph.html
    ├── settings.html
    └── admin.html
```

---

## 11. Implementation Priority

| Priority | Feature | Effort |
|----------|---------|--------|
| 1 | Dashboard Stats | Medium |
| 2 | Memory CRUD | Medium |
| 3 | Search UI | Low |
| 4 | Settings Panel | Low |
| 5 | Belief Graph | High |
| 6 | Agent Runner | Medium |
| 7 | Auth/RBAC | High |
| 8 | Admin Panel | Medium |
| 9 | API Docs | Low |
| 10 | Charts/Metrics | Medium |

---

*Generated: 2026-03-11*
