//! Cortex - Cognitive Memory System
//!
//! A cognitive memory system with agent runtime, task management, and native UI.

pub mod a2a;
pub mod agents;
pub mod checkpoint;
pub mod memory;
pub mod scheduler;
pub mod security;
pub mod server;
pub mod tasks;
pub mod tools;

#[cfg(feature = "egui")]
pub mod ui;

use std::sync::Arc;

use agents::AgentRuntime;
use memory::{belief_graph::SharedBeliefGraph, file_indexer::FileIndexer, qmd_memory::QmdMemory};

/// Application state for HTTP server
#[derive(Clone)]
pub struct AppState {
    pub memory: Arc<QmdMemory>,
    pub runtime: Arc<AgentRuntime>,
    pub belief_graph: SharedBeliefGraph,
    pub indexer: FileIndexer,
    pub code_indexer: Arc<code_graph::indexer::Indexer>,
    pub code_query: Arc<code_graph::query::QueryEngine>,
    pub code_db: Arc<code_graph::db::CodeGraphDB>,
}
