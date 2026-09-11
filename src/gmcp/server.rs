// GMCP Server Substrate: Model Context Protocol JSON-RPC 2.0 Interface
// 100% Rust implementation serving Tier 1 Swarm & ToolRegistry

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use serde_json::json;

use crate::gmcp::tools::ToolRegistry;
use crate::gmcp::ProtocolDispatcher;

pub struct GmcpServer;

impl GmcpServer {
    pub fn run_stdio(workspace: &Path, _version: &str) {
        eprintln!("🔌 [GMCP Server] Started (Listening on stdio).");
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let server = GmcpProtocolHandler;

        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };

            let response = server.handle_request(&line, workspace);
            let _ = writeln!(stdout, "{}", response);
            let _ = stdout.flush();
        }
    }

    pub fn start_tcp_server(workspace: PathBuf, port: u16, _version: String) {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).expect("Failed to bind GMCP TCP server");
        eprintln!("🔌 [GMCP TCP] Substrate active on {}", addr);

        for stream in listener.incoming() {
            let mut stream = stream.expect("GMCP Stream Error");
            let mut out_stream = stream.try_clone().expect("Failed to clone GMCP stream");
            let workspace = workspace.clone();
            let server = GmcpProtocolHandler;

            thread::spawn(move || {
                let mut reader = BufReader::new(&mut stream);
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).is_err() || line.is_empty() { break; }
                    let response = server.handle_request(&line, &workspace);
                    if writeln!(out_stream, "{}", response).is_err() { break; }
                    let _ = out_stream.flush();
                }
            });
        }
    }
}

/// 🔋 GMCP Protocol Handler: Decoupled JSON-RPC implementation for the Substrate.
pub struct GmcpProtocolHandler;

impl ProtocolDispatcher for GmcpProtocolHandler {
    fn handle_request(&self, line: &str, workspace: &Path) -> String {
        let id = extract_id(line);
        let method = extract_method(line);

        match method.as_deref() {
            Some("initialize") => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": { "listChanged": false }
                        },
                        "serverInfo": { "name": "aeon-substrate", "version": crate::AEON_VERSION }
                    }
                }).to_string()
            }
            Some("tools/list") => {
                let tools = ToolRegistry::list_tools();
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": tools
                    }
                }).to_string()
            }
            Some("tools/call") => {
                let tool_name = extract_tool_name(line).unwrap_or_default();
                let tool_arg = extract_tool_arg(line).unwrap_or_default();

                // 🚀 Fully Meta Dispatch via ToolRegistry
                let result_text = ToolRegistry::execute_tool(&tool_name, &tool_arg, workspace);

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [
                            { "type": "text", "text": result_text }
                        ]
                    }
                }).to_string()
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": "Method not found" }
                }).to_string()
            }
        }
    }
}

fn extract_id(line: &str) -> serde_json::Value {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
        if let Some(id) = v.get("id") {
            return id.clone();
        }
    }
    json!(null)
}

fn extract_method(line: &str) -> Option<String> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
        if let Some(m) = v.get("method").and_then(|m| m.as_str()) {
            return Some(m.to_string());
        }
    }
    None
}

fn extract_tool_name(line: &str) -> Option<String> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
        if let Some(params) = v.get("params") {
            if let Some(name) = params.get("name").and_then(|n| n.as_str()) {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn extract_tool_arg(line: &str) -> Option<String> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
        if let Some(params) = v.get("params") {
            if let Some(arguments) = params.get("arguments") {
                return if let Some(s) = arguments.as_str() {
                    Some(s.to_string())
                } else if let Some(command) = arguments.get("command").and_then(|c| c.as_str()) {
                    Some(command.to_string())
                } else if let Some(path) = arguments.get("path").and_then(|p| p.as_str()) {
                    Some(path.to_string())
                } else {
                    Some(arguments.to_string())
                };
            }
        }
    }
    None
}
