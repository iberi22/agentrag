//! Cortex UI - Standalone egui desktop application
//!
//! Run with: cargo run --features egui-standalone --bin cortex-gui
//!
//! Or build: cargo build --features egui-standalone --release --bin cortex-gui

use cortex::ui::{EguiState, KanbanState};
use eframe::egui;

fn main() -> eframe::Result<()> {
    // Configure native options for desktop
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.title = Some("Cortex - Kanban Board".to_string());
    native_options.viewport.inner_size = Some(egui::vec2(1200.0, 800.0));
    native_options.viewport.min_inner_size = Some(egui::vec2(800.0, 600.0));

    // Run the application
    eframe::run_native(
        "Cortex",
        native_options,
        Box::new(|_cc| Ok(Box::new(CortexApp::new()))),
    )
}

/// Main application struct
struct CortexApp {
    state: KanbanState,
}

impl CortexApp {
    fn new() -> Self {
        Self {
            state: KanbanState::new(),
        }
    }
}

impl eframe::App for CortexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update state and render
        self.state.render(ctx);

        // Request repaint for animations
        ctx.request_repaint();
    }
}
