// Cortex - Cognitive Memory System
// Minimal viable slice for production

use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use cortex::{
    agents::{AgentRuntime, RuntimeConfig},
    memory::{
        belief_graph::{BeliefGraph, SharedBeliefGraph},
        file_indexer::FileIndexer,
        qmd_memory::QmdMemory,
    },
    server, AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Cortex - Cognitive Memory System");

    // Initialize CodeGraph (SQLite storage in data/ for persistence)
    let db_path = std::path::Path::new("data/code_graph.db");
    let code_db = Arc::new(code_graph::db::CodeGraphDB::new(db_path)?);
    let code_indexer = Arc::new(code_graph::indexer::Indexer::new(code_db.clone()));
    let code_query = Arc::new(code_graph::query::QueryEngine::new(code_db.clone()));

    // In-memory storage (minimal viable slice)
    let docs: Arc<RwLock<Vec<cortex::memory::qmd_memory::MemoryDocument>>> =
        Arc::new(RwLock::new(Vec::new()));
    let memory = Arc::new(QmdMemory::new(docs));
    memory.init().await?;

    // Add default startup memories
    memory.add_document(
        "system/cortex".to_string(),
        "Cortex is the central memory system for SWAL agents. Use /memory/add to store, /memory/search to find, /memory/query for AI responses.".to_string(),
        serde_json::json!({"type": "system", "tags": ["cortex", "memory"]})
    ).await.unwrap();

    memory.add_document(
        "system/swal".to_string(),
        "SouthWest AI Labs (SWAL) builds AI agents. BELA is the developer. Projects: Cortex (memory), ZeroClaw (runtime), ManteniApp (SaaS), Trading Bot.".to_string(),
        serde_json::json!({"type": "company", "tags": ["swal", "company"]})
    ).await.unwrap();

    memory.add_document(
        "docs/api".to_string(),
        "Cortex API: POST /memory/add (content, path, metadata), POST /memory/search (query, limit), POST /memory/query (query). Auth: X-Cortex-Token header.".to_string(),
        serde_json::json!({"type": "docs", "tags": ["api"]})
    ).await.unwrap();

    let runtime = Arc::new(AgentRuntime::new(
        Arc::clone(&memory),
        RuntimeConfig::from_env(),
    ));
    let belief_graph: SharedBeliefGraph = Arc::new(RwLock::new(BeliefGraph::new()));
    let indexer = FileIndexer::new(
        cortex::memory::file_indexer::FileIndexerConfig::default(),
        Some(code_indexer.clone()),
    );

    let state = AppState {
        memory,
        runtime,
        belief_graph,
        indexer,
        code_indexer,
        code_query,
        code_db,
    };

    let app = Router::new()
        .route("/health", get(server::http::health))
        .route("/memory/add", post(server::http::memory_add))
        .route("/memory/delete", post(server::http::memory_delete))
        .route("/memory/search", post(server::http::memory_search))
        .route("/memory/query", post(server::http::memory_query))
        .route("/memory/graph", get(server::http::memory_graph))
        .route("/agents/run", post(server::http::agents_run))
        .route("/sync", post(server::http::sync_tier1))
        .route("/code/scan", post(server::http::code_scan))
        .route("/code/find", post(server::http::code_find))
        .route("/code/stats", get(server::http::code_stats))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    tracing::info!("Cortex HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn auth_middleware(req: Request<Body>, next: Next) -> Response {
    let dev_mode = std::env::var("CORTEX_DEV_MODE").is_ok();

    // Skip auth for health check and in dev mode if explicitly set
    if req.uri().path() == "/health" || dev_mode {
        return next.run(req).await;
    }

    let token = std::env::var("CORTEX_TOKEN").unwrap_or_else(|_| "dev-token".into());

    if let Some(auth_header) = req.headers().get("X-Cortex-Token") {
        if auth_header == token.as_str() {
            return next.run(req).await;
        }
    }

    (
        StatusCode::UNAUTHORIZED,
        "Unauthorized: Invalid or missing X-Cortex-Token",
    )
        .into_response()
}
