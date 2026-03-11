//! Tasks Module - Backend-agnostic task management
//!
//! Architecture:
//! - Tasks are stored in Cortex's own backend (TaskStore)
//! - Planka is just a sync target (optional)
//! - Can work fully offline without Planka

pub mod models;
pub mod store;
pub mod sync;

pub use store::{InMemoryTaskStore, TaskService, TaskStore};

// Re-exports

/// Default projects for SouthLabs
pub fn default_projects() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Cortex", "Sistema de memoria y cognitive"),
        ("ZeroClaw", "Runtime Rust para agentes"),
        ("Trading Bot", "Automatizacion de trading"),
        ("ManteniApp", "SaaS mantenimiento industrial"),
        ("Research", "Investigacion y experimentos"),
        ("Ops", "Infraestructura y DevOps"),
    ]
}
