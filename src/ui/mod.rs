//! Kanban UI - Native egui-based kanban board for Cortex
//!
//! This provides a native kanban UI that can run:
//! - As a desktop app (via eframe)
//! - In the browser (via WASM + web_sys)
//! - Embedded in the Cortex HTTP server
//!
//! ## Usage
//!
//! ```rust
//! use cortex::ui::kanban::KanbanApp;
//! use cortex::tasks::{TaskService, InMemoryTaskStore};
//! use std::sync::Arc;
//!
//! let store = Arc::new(InMemoryTaskStore::new());
//! let app = KanbanApp::new(store);
//! ```

// UI modules - only compile with egui feature
#[cfg(feature = "egui")]
pub mod board;
#[cfg(feature = "egui")]
pub mod card;
#[cfg(feature = "egui")]
pub mod state;

#[cfg(feature = "egui")]
pub use board::BoardView;
#[cfg(feature = "egui")]
pub use card::CardView;
#[cfg(feature = "egui")]
pub use state::{EguiState, KanbanState};
