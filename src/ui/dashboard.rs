//! Cortex Dashboard - Modern Web UI for Production
//!
//! This module provides a production-ready web interface for Cortex.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dashboard metrics for real-time display
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardMetrics {
    pub total_memories: usize,
    pub total_beliefs: usize,
    pub active_agents: usize,
    pub queries_today: usize,
    pub avg_response_time_ms: u64,
    pub uptime_seconds: u64,
}

/// System status for health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: StatusLevel,
    pub version: String,
    pub database_connected: bool,
    pub mcp_server_running: bool,
    pub last_backup: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StatusLevel {
    Healthy,
    Warning,
    Error,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            status: StatusLevel::Healthy,
            version: env!("CARGO_PKG_VERSION").to_string(),
            database_connected: false,
            mcp_server_running: false,
            last_backup: None,
        }
    }
}

/// Memory item for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDisplay {
    pub id: String,
    pub content: String,
    pub path: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

impl MemoryDisplay {
    pub fn preview(&self, max_len: usize) -> String {
        if self.content.len() > max_len {
            format!("{}...", &self.content[..max_len])
        } else {
            self.content.clone()
        }
    }
}

/// Belief node for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefDisplay {
    pub id: String,
    pub label: String,
    pub belief_type: String,
    pub confidence: f32,
    pub connections: Vec<String>,
}

/// Search result with highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultDisplay {
    pub id: String,
    pub content: String,
    pub path: String,
    pub score: f32,
    pub highlights: Vec<String>,
}

/// User preferences for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: Theme,
    pub language: String,
    pub sidebar_collapsed: bool,
    pub notifications_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            language: "en".to_string(),
            sidebar_collapsed: false,
            notifications_enabled: true,
        }
    }
}

/// API Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
            }),
        }
    }
}

/// Pagination for list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, per_page: usize) -> Self {
        Self {
            has_next: page * per_page < total,
            has_prev: page > 1,
            total,
            page,
            per_page,
            items,
        }
    }
}

/// WebSocket message for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    MemoryAdded(MemoryDisplay),
    MemoryDeleted(String),
    BeliefUpdated(BeliefDisplay),
    MetricsUpdated(DashboardMetrics),
    TaskStatusChanged { task_id: String, status: String },
    AgentConnected { agent_id: String },
    AgentDisconnected { agent_id: String },
}

/// Configuration for the web UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebUIConfig {
    pub title: String,
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub accent_color: String,
    pub features: HashMap<String, bool>,
}

impl Default for WebUIConfig {
    fn default() -> Self {
        let mut features = HashMap::new();
        features.insert("belief_graph".to_string(), true);
        features.insert("kanban".to_string(), true);
        features.insert("analytics".to_string(), true);
        features.insert("api_docs".to_string(), true);

        Self {
            title: "Cortex".to_string(),
            logo_url: None,
            primary_color: "#6366f1".to_string(),
            accent_color: "#818cf8".to_string(),
            features,
        }
    }
}
