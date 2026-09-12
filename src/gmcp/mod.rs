pub mod client;
pub mod server;
pub mod tools;
pub mod reflexes;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMcpEntry {
    pub name: String,
    pub description: String,
    pub package: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

/// GMCP Host: The unified execution entry point for the Meta-Intelligence Substrate.
pub struct GmcpHost;

impl GmcpHost {
    pub fn dispatch(name: &str, arg: &str, workspace: &Path) -> String {
        let val = serde_json::from_str(arg).unwrap_or(serde_json::json!(arg));
        self::tools::ToolRegistry::execute_tool(name, &val, workspace)
    }
}

/// Protocol Dispatcher: Trait for handling cross-protocol JSON-RPC requests.
pub trait ProtocolDispatcher: Send + Sync {
    fn handle_request(&self, line: &str, workspace: &Path) -> String;
}

/// Capability Resolver: Trait for dynamic discovery and resolution of tool capabilities.
pub trait CapabilityResolver: Send + Sync {
    fn resolve(&self, name: &str) -> Option<String>;
}
