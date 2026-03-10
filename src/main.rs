use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::{env, net::SocketAddr, sync::Arc};
use surrealdb::{
    engine::any::connect,
    opt::auth::Root,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod agents;
mod memory;
mod secrets;
mod server;
mod tools;

use agents::{AgentRuntime, RuntimeConfig};
use memory::{belief_graph::SharedBeliefGraph, qmd_memory::QmdMemory};

#[derive(Clone)]
pub(crate) struct AppState {
    pub memory: QmdMemory,
    pub runtime: Arc<AgentRuntime>,
    pub belief_graph: SharedBeliefGraph,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Cortex - Cognitive Memory System");

    let surreal_url = env::var("CORTEX_SURREALDB_URL")
        .unwrap_or_else(|_| "ws://localhost:8000".to_string());
    let surreal_username = env::var("CORTEX_SURREALDB_USERNAME")
        .unwrap_or_else(|_| "root".to_string());
    let surreal_password = env::var("CORTEX_SURREALDB_PASSWORD")
        .unwrap_or_else(|_| "root".to_string());
    let surreal_namespace = env::var("CORTEX_SURREALDB_NS")
        .unwrap_or_else(|_| "agentrag".to_string());
    let surreal_database = env::var("CORTEX_SURREALDB_DB")
        .unwrap_or_else(|_| "system3".to_string());

    let db = connect(&surreal_url).await?;
    db.signin(Root {
        username: surreal_username,
        password: surreal_password,
    })
    .await?;
    db.use_ns(surreal_namespace).use_db(surreal_database).await?;

    let memory = QmdMemory::new(Arc::new(db));
    memory.init().await?;

    let runtime = Arc::new(AgentRuntime::new(memory.clone(), RuntimeConfig::default()));
    let belief_graph = Arc::new(tokio::sync::RwLock::new(
        memory::belief_graph::BeliefGraph::new(),
    ));

    let state = AppState {
        memory,
        runtime,
        belief_graph,
    };

    let app = Router::new()
        .route("/health", get(server::http::health))
        .route("/memory/search", post(server::http::memory_search))
        .route("/memory/query", post(server::http::memory_query))
        .route("/memory/graph", get(server::http::memory_graph))
        .route("/agents/run", post(server::http::agents_run))
        .route("/sync", post(server::http::sync_tier1))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    tracing::info!("Cortex HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
