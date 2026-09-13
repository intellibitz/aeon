// GMCP Server Substrate: Model Context Protocol JSON-RPC 2.0 Interface
// 100% Rust implementation serving Tier 1 Swarm & ToolRegistry

use tiny_http::{Server, Response, Method, Header};
use std::path::{Path, PathBuf};
use std::thread;
use serde_json::json;

use crate::gmcp::tools::ToolRegistry;
use crate::gmcp::ProtocolDispatcher;

pub struct GmcpServer;

impl GmcpServer {
    pub fn run_stdio(workspace: &Path, _version: &str) {
        eprintln!("[GMCP Server] Started (Listening on stdio).");
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let server = GmcpProtocolHandler;

        for line in std::io::BufRead::lines(stdin.lock()) {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };

            let response = server.handle_request(&line, workspace);
            let _ = std::io::Write::write_all(&mut stdout, format!("{}\n", response).as_bytes());
            let _ = std::io::Write::flush(&mut stdout);
        }
    }

    pub fn start_tcp_server(_workspace: PathBuf, _port: u16, _version: String) {
        // TCP server now consolidated into HTTP/SSE via tiny_http for reliability
        eprintln!("[GMCP TCP] Protocol deprecated. Use GMCP HTTP/SSE on 9093.");
    }

    pub fn start_http_server(workspace: PathBuf, server: Server) {
        let addr = server.server_addr().to_string();
        eprintln!("[GMCP HTTP/SSE] Substrate active on {}", addr);

        for mut request in server.incoming_requests() {
            let workspace = workspace.clone();
            let method = request.method().clone();
            let url = request.url().to_string();
            let server_handler = GmcpProtocolHandler;

            match (method, url.as_str()) {
                (Method::Get, "/sse") => {
                    let endpoint_event = format!("event: endpoint\ndata: /messages?session={}\n\n", "default-session");
                    let response = Response::from_string(endpoint_event)
                        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap())
                        .with_header(Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..]).unwrap())
                        .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                    let _ = request.respond(response);
                }
                (Method::Post, path) if path.starts_with("/messages") => {
                    let mut body = String::new();
                    let _ = std::io::Read::read_to_string(request.as_reader(), &mut body);

                    let workspace_thread = workspace.clone();
                    let body_thread = body.clone();
                    let server_handler_thread = server_handler;

                    thread::spawn(move || {
                        let (tx, rx) = std::sync::mpsc::channel();
                        let b_thread = body_thread.clone();
                        let w_thread = workspace_thread.clone();

                        thread::spawn(move || {
                            let response_json = server_handler_thread.handle_request(&b_thread, &w_thread);
                            let _ = tx.send(response_json);
                        });

                        // Enforce a fluid execution lease (Aspiration 20)
                        let response_json = rx.recv_timeout(std::time::Duration::from_secs(600)) // 10 minute fluid lease
                            .unwrap_or_else(|_| {
                                json!({
                                    "jsonrpc": "2.0",
                                    "error": { "code": -32000, "message": "Mission Timeout: Substrate saturation exceeded 600s lease." }
                                }).to_string()
                            });

                        let response = Response::from_string(response_json)
                            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                            .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                        let _ = request.respond(response);
                    });
                }
                _ => {
                    let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
                }
            }
        }
    }
}

/// GMCP Protocol Handler: Decoupled JSON-RPC implementation for the Substrate.
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
            Some("prompts/list") => {
                let external = crate::gmcp::client::GmcpClient::list_external_prompts();
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "prompts": external
                    }
                }).to_string()
            }
            Some("resources/list") => {
                let external = crate::gmcp::client::GmcpClient::list_external_resources();
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "resources": external
                    }
                }).to_string()
            }
            Some("locks/acquire") => {
                let arg = extract_tool_val(line).unwrap_or(json!(null));
                let resource_id = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
                let success = ToolRegistry::acquire_local_lock(&resource_id);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": if success { "SUCCESS" } else { "DENIED" }
                }).to_string()
            }
            Some("locks/release") => {
                let arg = extract_tool_val(line).unwrap_or(json!(null));
                let resource_id = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
                ToolRegistry::release_meta_lock(&resource_id);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": "RELEASED"
                }).to_string()
            }
            Some("tools/call") => {
                let tool_name = extract_tool_name(line).unwrap_or_default();
                let tool_arg = extract_tool_val(line).unwrap_or(json!(null));

                // Fully Meta Dispatch via ToolRegistry
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

fn extract_tool_val(line: &str) -> Option<serde_json::Value> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
        if let Some(params) = v.get("params") {
            if let Some(arguments) = params.get("arguments") {
                return Some(arguments.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_tool_name_and_arguments() {
        let json_line = r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"test_tool","arguments":{"command":"ls"}},"id":1}"#;
        assert_eq!(extract_tool_name(json_line), Some("test_tool".to_string()));
        assert_eq!(extract_tool_val(json_line), Some(json!({"command":"ls"})));
    }

    #[test]
    fn test_gmcp_protocol_handler_malformed_jsonrpc() {
        let handler = GmcpProtocolHandler;
        let response = handler.handle_request("not valid json", Path::new("."));
        let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert!(parsed.get("error").is_some());
    }
}
