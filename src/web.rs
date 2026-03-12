//! Cortex Web - WASM entry point for web-based UI

use cortex::ui::{KanbanState};
use eframe::wasm_bindgen::prelude::*;

/// Start the egui web application
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    // Log startup
    web_sys::console::log_1(&"Cortex UI starting...".into());
    
    // Run the application
    eframe::WebRunner::new()
        .start(
            "canvas", // canvas element ID
            eframe::WebOptions::default(),
            Box::new(|_cc| Ok(Box::new(CortexWebApp::new()))),
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    web_sys::console::log_1(&"Cortex UI initialized!".into());
    
    Ok(())
}

/// Main web application struct
struct CortexWebApp {
    state: KanbanState,
}

impl CortexWebApp {
    fn new() -> Self {
        Self {
            state: KanbanState::new(),
        }
    }
}

impl eframe::App for CortexWebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.render(ctx);
        ctx.request_repaint();
    }
}
