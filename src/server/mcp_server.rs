//! MCP Server Implementation for Cortex
//! 
//! This module implements the Model Context Protocol server

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

// ============================================================================
// MCP Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResource {
    pub uri: String,
    pub name: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPToolResult {
    pub content: Vec<MCPTextContent>,
    pub is_error: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTextContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

// ============================================================================
// Cortex MCP Tools
// ============================================================================

pub fn get_cortex_tools() -> Vec<MCPTool> {
    vec![
        MCPTool {
            name: "create_memory".to_string(),
            description: "Create a new memory document in Cortex".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path/identifier for the memory"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content of the memory"
                    },
                    "metadata": {
                        "type": "object",
                        "description": "Optional metadata"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        MCPTool {
            name: "search_memory".to_string(),
            description: "Search memory documents in Cortex".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    },
                    "limit": {
                        "type": "number",
                        "description": "Maximum results",
                        "default": 10
                    }
                },
                "required": ["query"]
            }),
        },
        MCPTool {
            name: "get_memory".to_string(),
            description: "Get a specific memory by ID".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Memory ID"
                    }
                },
                "required": ["id"]
            }),
        },
        MCPTool {
            name: "list_projects".to_string(),
            description: "List all projects in Cortex".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        MCPTool {
            name: "get_project_context".to_string(),
            description: "Get full context for a project".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "project_id": {
                        "type": "string",
                        "description": "Project identifier"
                    }
                },
                "required": ["project_id"]
            }),
        },
        MCPTool {
            name: "sync_gitcore".to_string(),
            description: "Sync documentation from GitCore project".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "project_path": {
                        "type": "string",
                        "description": "Path to GitCore project"
                    }
                },
                "required": ["project_path"]
            }),
        },
    ]
}

pub fn get_cortex_resources() -> Vec<MCPResource> {
    vec![
        MCPResource {
            uri: "cortex://memory".to_string(),
            name: "Memory Store".to_string(),
            mime_type: "application/json".to_string(),
        },
        MCPResource {
            uri: "cortex://projects".to_string(),
            name: "Projects List".to_string(),
            mime_type: "application/json".to_string(),
        },
        MCPResource {
            uri: "cortex://health".to_string(),
            name: "System Health".to_string(),
            mime_type: "application/json".to_string(),
        },
    ]
}

// ============================================================================
// MCP Server State
// ============================================================================

pub struct MCPServer {
    pub tools: Vec<MCPTool>,
    pub resources: Vec<MCPResource>,
}

impl MCPServer {
    pub fn new() -> Self {
        Self {
            tools: get_cortex_tools(),
            resources: get_cortex_resources(),
        }
    }
}

impl Default for MCPServer {
    fn default() -> Self {
        Self::new()
    }
}
