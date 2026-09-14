// GEMI HTTP REST Substrate: OpenAI-Compatible Interface & Adaptive Web Interface
// 100% Rust implementation serving Tier 1 & Tier 2 Intelligence Swarms

use tiny_http::{Server, Response, Method, Header};
use std::path::PathBuf;
use std::thread;
use serde_json::json;

use crate::gawd::ama::SusiMasterAgent;
use crate::gmcp::tools::ToolRegistry;
use crate::gemi::models::ModelManager;

pub struct GemiServer;

impl GemiServer {
    pub fn start_http_server(workspace: PathBuf, server: Server) {
        let addr = server.server_addr().to_string();
        eprintln!("[GEMI REST] Substrate active on {}", addr);
        eprintln!("[GEMI Web] UI Interface: http://localhost:{}/app", server.server_addr().to_ip().map(|a| a.port()).unwrap_or(0));

        for mut request in server.incoming_requests() {
            let workspace = workspace.clone();
            let method = request.method().clone();
            let url = request.url().to_string();

            let mut body_str = String::new();
            let _ = std::io::Read::read_to_string(request.as_reader(), &mut body_str);

            let workspace_thread = workspace.clone();
            let method_thread = method.clone();
            let url_thread = url.clone();
            let body_thread = body_str.clone();

            thread::spawn(move || {
                let (tx, rx) = std::sync::mpsc::channel::<Result<Response<std::io::Cursor<Vec<u8>>>, String>>();
                let w_thread = workspace_thread;
                let m_thread = method_thread;
                let u_thread = url_thread;
                let b_thread = body_thread;

                thread::spawn(move || {
                    let result = match (m_thread, u_thread.as_str()) {
                        (Method::Get, "/" | "/v1" | "/v1/" | "/health" | "/app" | "/favicon.ico") => {
                            let api_status = json!({
                                "object": "api_status",
                                "name": "SUSI OpenAI-Compatible REST Substrate",
                                "version": crate::SUSI_VERSION,
                                "status": "active",
                                "endpoints": [
                                    "/v1/chat/completions",
                                    "/v1/models",
                                    "/v1/completions"
                                ]
                            });
                            let payload = serde_json::to_string_pretty(&api_status).unwrap_or_default();
                            Ok(Response::from_string(payload)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"*"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap()))
                        }
                        (Method::Get, path) if path.starts_with("/v1/models") || path.starts_with("/models") => {
                            let models = ModelManager::list_models(&w_thread);
                            let json_models: Vec<serde_json::Value> = models
                                .iter()
                                .map(|m| json!({"id": m.model_id, "object": "model", "owned_by": "susi"}))
                                .collect();
                            let payload_val = json!({"object": "list", "data": json_models});
                            let payload = serde_json::to_string(&payload_val).unwrap_or_default();

                            Ok(Response::from_string(payload)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                        }
                        (Method::Get, "/well-known/susi") => {
                            let hardware = crate::gemi::hardware::HardwareProfiler::get_profile();
                            let (engine, model) = crate::gemi::models::ModelManager::get_active_engine_and_model();
                            let tools = ToolRegistry::list_tools();

                            let info = json!({
                                "version": crate::SUSI_VERSION,
                                "identity": "SUSI Intelligence Substrate",
                                "engine": engine,
                                "model": model,
                                "hardware": {
                                    "cpus": hardware.cpus,
                                    "gpu": hardware.gpu_info,
                                    "acceleration": hardware.acceleration_active,
                                    "os": hardware.os_info
                                },
                                "reflexes": tools.iter().map(|t| &t.name).collect::<Vec<_>>()
                            });

                            let payload = serde_json::to_string_pretty(&info).unwrap_or_default();
                            Ok(Response::from_string(payload)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                        }
                        (Method::Post, path) if path.starts_with("/v1/chat/completions") || path.starts_with("/chat/completions") || path.starts_with("/v1/completions") || path == "/" || path == "/v1" || path == "/v1/" => {
                            let is_streaming = b_thread.contains("\"stream\":true") || b_thread.contains("\"stream\": true") || b_thread.contains("stream");
                            let active_model = crate::gemi::models::ModelManager::get_selected_model()
                                .unwrap_or_else(|| "susi-native-synthesis".to_string());
                            let model_name = active_model.as_str();

                            let user_prompt = extract_prompt_from_json(&b_thread).unwrap_or_else(|| "list workspace health".to_string());
                            crate::sandbox::manager::SusiAuditLogger::log_event(&w_thread, "WEB_MISSION_START", &user_prompt);

                            let trimmed_prompt = user_prompt.trim();
                            let clean_cmd = trimmed_prompt.trim_start_matches('/').trim_start_matches(':');
                            let parts: Vec<&str> = clean_cmd.splitn(2, ' ').collect();
                            let tool_name = parts[0].to_lowercase();
                            let tool_arg = parts.get(1).copied().unwrap_or("").trim();

                            let content = if ToolRegistry::exists(&tool_name) {
                                ToolRegistry::execute_tool(&tool_name, &serde_json::json!(tool_arg), &w_thread)
                            } else {
                                let ama = SusiMasterAgent::new();
                                let final_resp = ama.solve_clean(trimmed_prompt, &w_thread, crate::SUSI_VERSION);
                                crate::sandbox::manager::SusiMemory::save_interaction(&w_thread, trimmed_prompt, &final_resp);
                                final_resp
                            };

                            if is_streaming {
                                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
                                let json_content = serde_json::to_string(&content).unwrap_or_default();

                                let sse_data = format!(
                                    "data: {{\"id\":\"chatcmpl-susi-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{\"role\":\"assistant\"}},\"finish_reason\":null}}]}}\n\n\
                                     data: {{\"id\":\"chatcmpl-susi-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":{}}},\"finish_reason\":null}}]}}\n\n\
                                     data: {{\"id\":\"chatcmpl-susi-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{}},\"finish_reason\":\"stop\"}}]}}\n\n\
                                     data: [DONE]\n\n",
                                    now, now, model_name, now, now, model_name, json_content, now, now, model_name
                                );

                                Ok(Response::from_string(sse_data)
                                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap())
                                    .with_header(Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..]).unwrap())
                                    .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                            } else {
                                let payload = json!({
                                    "id": format!("chatcmpl-susi-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)),
                                    "object": "chat.completion",
                                    "created": 1700000000,
                                    "model": model_name,
                                    "choices": [{
                                        "index": 0,
                                        "message": { "role": "assistant", "content": content },
                                        "finish_reason": "stop"
                                    }]
                                });

                                Ok(Response::from_string(serde_json::to_string(&payload).unwrap_or_default())
                                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                    .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                            }
                        }
                        (Method::Options, _) => {
                            Ok(Response::from_string("")
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"*"[..]).unwrap()))
                        }
                        _ => {
                            let payload = json!({"error": "Endpoint not found"}).to_string();
                            Ok(Response::from_string(payload)
                                .with_status_code(404)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                        }
                    };
                    let _ = tx.send(result);
                });

                // Enforce a fluid execution lease (Aspiration 20)
                let response = rx.recv_timeout(std::time::Duration::from_secs(600)) // 10 minute fluid lease
                    .unwrap_or_else(|_| {
                        let payload = json!({
                            "error": "Mission Timeout",
                            "message": "The intelligence substrate exceeded the 600-second execution lease."
                        }).to_string();
                        Ok(Response::from_string(payload)
                            .with_status_code(504)
                            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()))
                    });

                if let Ok(resp) = response {
                    let _ = request.respond(resp);
                }
            });
        }
    }
}

fn extract_prompt_from_json(body: &str) -> Option<String> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(messages) = v.get("messages").and_then(|m| m.as_array()) {
            if let Some(last) = messages.last() {
                if let Some(c) = last.get("content") {
                    if let Some(s) = c.as_str() {
                        return Some(s.to_string());
                    } else if let Some(arr) = c.as_array() {
                        for item in arr {
                            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                return Some(text.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
