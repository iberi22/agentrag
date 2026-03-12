//! HTTP handlers for the minimal Cortex vertical slice.

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    memory::belief_graph::{BeliefNode, BeliefRelation},
    memory::qmd_memory::MemoryDocument,
    AppState,
};

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl HttpConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }

    pub fn with_tls(mut self, cert_path: String, key_path: String) -> Self {
        self.tls_enabled = true;
        self.tls_cert_path = Some(cert_path);
        self.tls_key_path = Some(key_path);
        self
    }
}

pub struct HttpServer {
    #[allow(dead_code)]
    config: HttpConfig,
}

impl HttpServer {
    pub fn new(config: HttpConfig) -> Self {
        Self { config }
    }

    pub async fn serve(&self) {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[derive(Debug, Deserialize)]
pub struct CodeScanRequest {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CodeFindRequest {
    #[serde(default)]
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub pattern: Option<String>,
}

fn default_limit() -> usize {
    10
}

#[derive(Debug, Serialize)]
pub struct CodeScanResponse {
    pub status: String,
    pub indexed_files: usize,
    pub indexed_chunks: usize,
    pub paths: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CodeFindResponse {
    pub status: String,
    pub results: Vec<CodeSymbol>,
}

#[derive(Debug, Serialize)]
pub struct CodeSymbol {
    pub path: String,
    pub symbol: String,
    pub symbol_type: String,
    pub line: usize,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CodeStatsResponse {
    pub status: String,
    pub total_files: usize,
    pub total_chunks: usize,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(Debug, Deserialize)]
pub struct AddMemoryRequest {
    pub content: String,
    pub path: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteMemoryRequest {
    pub id: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentRunRequest {
    pub query: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub status: String,
    pub results: Vec<serde_json::Value>,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub status: String,
    pub response: String,
    pub confidence: f32,
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct GraphResponse {
    pub status: String,
    pub nodes: Vec<BeliefNode>,
    pub edges: Vec<BeliefRelation>,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub status: String,
    pub session_id: String,
    pub response: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub status: String,
    pub synced: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMemoryResponse {
    pub status: String,
    pub deleted: bool,
    pub id: Option<String>,
    pub path: Option<String>,
}

pub async fn health() -> impl IntoResponse {
    // Optimized: return static JSON to reduce allocation/serialization time if called frequently
    const HEALTH_JSON: &str = r#"{"status":"ok","service":"cortex","version":"0.1.0"}"#;
    axum::response::Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(HEALTH_JSON))
        .unwrap()
}

pub async fn memory_add(
    State(state): State<AppState>,
    Json(payload): Json<AddMemoryRequest>,
) -> impl IntoResponse {
    info!(
        "➕ Add memory: {}",
        payload.content.chars().take(50).collect::<String>()
    );

    let path = payload.path.unwrap_or_else(|| "default".to_string());
    let metadata = payload.metadata.unwrap_or(serde_json::json!({}));

    state
        .memory
        .add_document(path, payload.content, metadata)
        .await
        .unwrap_or_default();

    Json(serde_json::json!({
        "status": "ok",
        "message": "Document added to memory"
    }))
}

pub async fn memory_search(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> impl IntoResponse {
    info!("🔍 Search request: {}", payload.query);

    let results = state
        .memory
        .search(&payload.query, payload.limit)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|doc: MemoryDocument| {
            serde_json::json!({
                "id": doc.id,
                "path": doc.path,
                "content": doc.content,
                "metadata": doc.metadata,
            })
        })
        .collect();

    Json(SearchResponse {
        status: "ok".to_string(),
        results,
        query: payload.query,
    })
}

pub async fn memory_delete(
    State(state): State<AppState>,
    Json(payload): Json<DeleteMemoryRequest>,
) -> impl IntoResponse {
    let target = payload.id.clone().or(payload.path.clone());

    let Some(target) = target else {
        return Json(DeleteMemoryResponse {
            status: "error: missing id or path".to_string(),
            deleted: false,
            id: payload.id,
            path: payload.path,
        });
    };

    info!("🗑️ Delete request: {}", target);

    match state.memory.delete(&target).await {
        Ok(Some(doc)) => Json(DeleteMemoryResponse {
            status: "ok".to_string(),
            deleted: true,
            id: doc.id,
            path: Some(doc.path),
        }),
        Ok(None) => Json(DeleteMemoryResponse {
            status: "not_found".to_string(),
            deleted: false,
            id: payload.id,
            path: payload.path,
        }),
        Err(error) => Json(DeleteMemoryResponse {
            status: format!("error: {}", error),
            deleted: false,
            id: payload.id,
            path: payload.path,
        }),
    }
}

pub async fn memory_query(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> impl IntoResponse {
    info!("🧠 Query request: {}", payload.query);

    let response = state.runtime.run(&payload.query, None).await;

    match response {
        Ok(result) => Json(QueryResponse {
            status: "ok".to_string(),
            response: result.response,
            confidence: result.confidence,
            session_id: result.session_id,
        }),
        Err(error) => Json(QueryResponse {
            status: format!("error: {}", error),
            response: String::new(),
            confidence: 0.0,
            session_id: String::new(),
        }),
    }
}

pub async fn memory_graph(State(state): State<AppState>) -> impl IntoResponse {
    info!("🔗 Graph request");

    let graph = state.belief_graph.read().await;

    Json(GraphResponse {
        status: "ok".to_string(),
        nodes: graph.list_nodes(),
        edges: graph.get_relations(),
    })
}

pub async fn agents_run(
    State(state): State<AppState>,
    Json(payload): Json<AgentRunRequest>,
) -> impl IntoResponse {
    info!("🤖 Agent run request: {}", payload.query);

    let response = state.runtime.run(&payload.query, payload.session_id).await;

    match response {
        Ok(result) => Json(AgentResponse {
            status: "ok".to_string(),
            session_id: result.session_id,
            response: result.response,
            confidence: result.confidence,
        }),
        Err(error) => Json(AgentResponse {
            status: format!("error: {}", error),
            session_id: String::new(),
            response: String::new(),
            confidence: 0.0,
        }),
    }
}

pub async fn sync_tier1(State(state): State<AppState>) -> impl IntoResponse {
    info!("🔄 Tier 1 sync request");

    let synced = state.memory.count().await.unwrap_or(0);

    Json(SyncResponse {
        status: "ok".to_string(),
        synced,
    })
}

pub async fn code_scan(
    State(state): State<AppState>,
    Json(payload): Json<CodeScanRequest>,
) -> impl IntoResponse {
    info!("📂 Code scan request: {:?}", payload.path);

    let path_str = payload.path.unwrap_or_else(|| ".".to_string());
    let path = std::path::Path::new(&path_str);

    match state.code_indexer.index(path).await {
        Ok(stats) => {
            Json(CodeScanResponse {
                status: "ok".to_string(),
                indexed_files: stats.total_files as usize,
                indexed_chunks: stats.total_symbols as usize, // Mapping symbols to chunks for compatibility
                paths: vec![path_str],
            })
        }
        Err(e) => Json(CodeScanResponse {
            status: format!("error: {}", e),
            indexed_files: 0,
            indexed_chunks: 0,
            paths: vec![],
        }),
    }
}

pub async fn code_find(
    State(state): State<AppState>,
    Json(payload): Json<CodeFindRequest>,
) -> impl IntoResponse {
    info!(
        "🔎 Code find request: {} (kind: {:?}, pattern: {:?})",
        payload.query, payload.kind, payload.pattern
    );

    // Filter by AST pattern if specified
    let symbols = if let Some(ref pattern) = payload.pattern {
        state
            .code_query
            .search_by_pattern(pattern, payload.limit)
            .unwrap_or_default()
    } else if let Some(ref kind) = payload.kind {
        match kind.to_lowercase().as_str() {
            "function" => state
                .code_query
                .functions(payload.limit)
                .unwrap_or_default(),
            "struct" => state.code_query.structs(payload.limit).unwrap_or_default(),
            "class" => state.code_query.classes(payload.limit).unwrap_or_default(),
            "enum" => state.code_query.enums(payload.limit).unwrap_or_default(),
            _ => state
                .code_query
                .search(&payload.query, payload.limit)
                .map(|r| r.symbols)
                .unwrap_or_default(),
        }
    } else {
        state
            .code_query
            .search(&payload.query, payload.limit)
            .map(|r| r.symbols)
            .unwrap_or_default()
    };

    let results: Vec<CodeSymbol> = symbols
        .into_iter()
        .map(|s| CodeSymbol {
            path: s.file_path,
            symbol: s.name,
            symbol_type: format!("{:?}", s.kind),
            line: s.start_line as usize,
            content: s.signature.unwrap_or_default(),
        })
        .collect();

    Json(CodeFindResponse {
        status: "ok".to_string(),
        results,
    })
}

pub async fn code_stats(State(state): State<AppState>) -> impl IntoResponse {
    info!("📊 Code stats request");

    // Get stats from code-graph database
    let stats = state
        .code_db
        .stats()
        .unwrap_or_else(|_| code_graph::types::IndexStats {
            total_files: 0,
            total_symbols: 0,
            total_imports: 0,
            languages: vec![],
            duration_ms: 0,
        });

    Json(CodeStatsResponse {
        status: "ok".to_string(),
        total_files: stats.total_files as usize,
        total_chunks: stats.total_symbols as usize,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use std::sync::Arc;
    use tower::util::ServiceExt;

    use crate::{
        agents::{AgentRuntime, RuntimeConfig},
        memory::{
            belief_graph::{BeliefGraph, SharedBeliefGraph},
            file_indexer::{FileIndexer, FileIndexerConfig},
            qmd_memory::QmdMemory,
        },
        AppState,
    };

    async fn test_state() -> AppState {
        let memory = Arc::new(QmdMemory::new(Arc::new(tokio::sync::RwLock::new(
            Vec::new(),
        ))));
        memory.init().await.unwrap();

        let runtime = Arc::new(AgentRuntime::new(
            Arc::clone(&memory),
            RuntimeConfig::default(),
        ));
        let belief_graph: SharedBeliefGraph =
            Arc::new(tokio::sync::RwLock::new(BeliefGraph::new()));
        let db_path = std::path::Path::new("data/code_graph_http_tests.db");
        let code_db = Arc::new(code_graph::db::CodeGraphDB::new(db_path).unwrap());
        let code_indexer = Arc::new(code_graph::indexer::Indexer::new(Arc::clone(&code_db)));
        let code_query = Arc::new(code_graph::query::QueryEngine::new(Arc::clone(&code_db)));

        AppState {
            memory,
            runtime,
            belief_graph,
            indexer: FileIndexer::new(FileIndexerConfig::default(), Some(code_indexer.clone())),
            code_indexer,
            code_query,
            code_db,
        }
    }

    fn test_router(state: AppState) -> Router {
        Router::new()
            .route("/memory/add", post(memory_add))
            .route("/memory/delete", post(memory_delete))
            .route("/memory/query", post(memory_query))
            .route("/memory/search", post(memory_search))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_runtime_and_http_share_same_memory_instance() {
        let state = test_state().await;

        assert!(Arc::ptr_eq(&state.memory, &state.runtime.memory()));
    }

    #[tokio::test]
    async fn test_memory_add_and_query_share_same_memory() {
        let app = test_router(test_state().await);

        let add_request = Request::builder()
            .method("POST")
            .uri("/memory/add")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "path": "shared-doc",
                    "content": "shared cortex memory document for system query",
                    "metadata": {"source": "http-test"}
                })
                .to_string(),
            ))
            .unwrap();

        let add_response = app.clone().oneshot(add_request).await.unwrap();
        assert_eq!(add_response.status(), StatusCode::OK);

        let query_request = Request::builder()
            .method("POST")
            .uri("/memory/query")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "query": "system query",
                    "limit": 5
                })
                .to_string(),
            ))
            .unwrap();

        let query_response = app.clone().oneshot(query_request).await.unwrap();
        assert_eq!(query_response.status(), StatusCode::OK);

        let query_body = to_bytes(query_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: QueryResponse = serde_json::from_slice(&query_body).unwrap();

        assert_eq!(payload.status, "ok");
        assert!(payload.confidence > 0.0);
        assert!(payload.response.contains("Found 1 relevant documents"));

        let search_request = Request::builder()
            .method("POST")
            .uri("/memory/search")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "query": "shared cortex",
                    "limit": 5
                })
                .to_string(),
            ))
            .unwrap();

        let search_response = app.oneshot(search_request).await.unwrap();
        assert_eq!(search_response.status(), StatusCode::OK);

        let search_body = to_bytes(search_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: SearchResponse = serde_json::from_slice(&search_body).unwrap();

        assert_eq!(payload.status, "ok");
        assert_eq!(payload.results.len(), 1);
    }

    #[tokio::test]
    async fn test_memory_delete_removes_document_from_shared_memory() {
        let app = test_router(test_state().await);

        let add_request = Request::builder()
            .method("POST")
            .uri("/memory/add")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "path": "delete-doc",
                    "content": "document to delete from cortex memory",
                    "metadata": {"source": "http-test"}
                })
                .to_string(),
            ))
            .unwrap();
        let add_response = app.clone().oneshot(add_request).await.unwrap();
        assert_eq!(add_response.status(), StatusCode::OK);

        let delete_request = Request::builder()
            .method("POST")
            .uri("/memory/delete")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "path": "delete-doc"
                })
                .to_string(),
            ))
            .unwrap();
        let delete_response = app.clone().oneshot(delete_request).await.unwrap();
        assert_eq!(delete_response.status(), StatusCode::OK);

        let delete_body = to_bytes(delete_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: DeleteMemoryResponse = serde_json::from_slice(&delete_body).unwrap();
        assert_eq!(payload.status, "ok");
        assert!(payload.deleted);
        assert_eq!(payload.path.as_deref(), Some("delete-doc"));

        let search_request = Request::builder()
            .method("POST")
            .uri("/memory/search")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "query": "delete from cortex",
                    "limit": 5
                })
                .to_string(),
            ))
            .unwrap();
        let search_response = app.oneshot(search_request).await.unwrap();
        assert_eq!(search_response.status(), StatusCode::OK);

        let search_body = to_bytes(search_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: SearchResponse = serde_json::from_slice(&search_body).unwrap();
        assert!(payload.results.is_empty());
    }
}
