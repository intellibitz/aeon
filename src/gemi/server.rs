// GEMI HTTP REST Substrate: OpenAI-Compatible Interface & Adaptive Web Interface
// 100% Rust implementation serving Tier 1 & Tier 2 Intelligence Swarms

use tiny_http::{Server, Response, Method, Header};
use std::path::PathBuf;
use std::thread;
use serde_json::json;

use crate::gawd::ama::AmaMasterAgent;
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
                        (Method::Get, "/" | "/app" | "/favicon.ico") => {
                            let html = get_web_app_html();
                            Ok(Response::from_string(html)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                        }
                        (Method::Get, path) if path.starts_with("/v1/models") || path.starts_with("/models") => {
                            let models = ModelManager::list_models(&w_thread);
                            let json_models: Vec<serde_json::Value> = models
                                .iter()
                                .map(|m| json!({"id": m.model_id, "object": "model", "owned_by": "aeon"}))
                                .collect();
                            let payload_val = json!({"object": "list", "data": json_models});
                            let payload = serde_json::to_string(&payload_val).unwrap_or_default();

                            Ok(Response::from_string(payload)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                        }
                        (Method::Get, "/well-known/aeon") => {
                            let hardware = crate::gemi::hardware::HardwareProfiler::get_profile();
                            let (engine, model) = crate::gemi::models::ModelManager::get_active_engine_and_model();
                            let tools = ToolRegistry::list_tools();

                            let info = json!({
                                "version": crate::AEON_VERSION,
                                "identity": "AEON Intelligence Substrate",
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
                        (Method::Post, path) if path.starts_with("/v1/chat/completions") || path.starts_with("/chat/completions") => {
                            let is_streaming = b_thread.contains("\"stream\":true") || b_thread.contains("\"stream\": true") || b_thread.contains("stream");
                            let active_model = crate::gemi::models::ModelManager::get_selected_model()
                                .unwrap_or_else(|| "aeon-native-synthesis".to_string());
                            let model_name = active_model.as_str();

                            let user_prompt = extract_prompt_from_json(&b_thread).unwrap_or_else(|| "list workspace health".to_string());
                            crate::sandbox::manager::AeonAuditLogger::log_event(&w_thread, "WEB_MISSION_START", &user_prompt);

                            let trimmed_prompt = user_prompt.trim();
                            let clean_cmd = trimmed_prompt.trim_start_matches('/').trim_start_matches(':');
                            let parts: Vec<&str> = clean_cmd.splitn(2, ' ').collect();
                            let tool_name = parts[0].to_lowercase();
                            let tool_arg = parts.get(1).copied().unwrap_or("").trim();

                            let content = if ToolRegistry::exists(&tool_name) {
                                ToolRegistry::execute_tool(&tool_name, &serde_json::json!(tool_arg), &w_thread)
                            } else {
                                let ama = AmaMasterAgent::new();
                                let final_resp = ama.solve_clean(trimmed_prompt, &w_thread, crate::AEON_VERSION);
                                crate::sandbox::manager::AeonMemory::save_interaction(&w_thread, trimmed_prompt, &final_resp);
                                final_resp
                            };

                            if is_streaming {
                                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
                                let json_content = serde_json::to_string(&content).unwrap_or_default();

                                let sse_data = format!(
                                    "data: {{\"id\":\"chatcmpl-aeon-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{\"role\":\"assistant\"}},\"finish_reason\":null}}]}}\n\n\
                                     data: {{\"id\":\"chatcmpl-aeon-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":{}}},\"finish_reason\":null}}]}}\n\n\
                                     data: {{\"id\":\"chatcmpl-aeon-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{}},\"finish_reason\":\"stop\"}}]}}\n\n\
                                     data: [DONE]\n\n",
                                    now, now, model_name, now, now, model_name, json_content, now, now, model_name
                                );

                                Ok(Response::from_string(sse_data)
                                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap())
                                    .with_header(Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..]).unwrap())
                                    .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()))
                            } else {
                                let payload = json!({
                                    "id": format!("chatcmpl-aeon-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)),
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

                // Enforce a strict 60-second timeout for any mission execution
                let response = rx.recv_timeout(std::time::Duration::from_secs(60))
                    .unwrap_or_else(|_| {
                        let payload = json!({
                            "error": "Mission Timeout",
                            "message": "The intelligence substrate exceeded the 60-second execution lease."
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

fn get_web_app_html() -> &'static str {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta name="mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-capable" content="yes">
<title>AEON Intelligence Web & Mobile App</title>
<style>
  :root { --bg: #f8fafc; --card: #ffffff; --text: #0f172a; --primary: #2563eb; --primary-hover: #1d4ed8; --border: #e2e8f0; --user-msg: #eff6ff; }
  @media (prefers-color-scheme: dark) {
    :root { --bg: #0f172a; --card: #1e293b; --text: #f8fafc; --primary: #3b82f6; --primary-hover: #60a5fa; --border: #334155; --user-msg: #1e3a8a; }
  }
  body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 0; display: flex; flex-direction: column; height: 100vh; }
  header { background: var(--card); border-bottom: 1px solid var(--border); padding: 12px 20px; display: flex; align-items: center; justify-content: space-between; }
  .logo { font-size: 1.2rem; font-weight: 700; display: flex; align-items: center; gap: 8px; }
  .status { font-size: 0.85rem; color: #10b981; font-weight: 600; display: flex; align-items: center; gap: 6px; }
  .status-dot { width: 8px; height: 8px; background: #10b981; border-radius: 50%; }

  .chips-container { display: flex; gap: 8px; overflow-x: auto; padding: 10px 20px; background: var(--card); border-bottom: 1px solid var(--border); scrollbar-width: none; }
  .chip { background: var(--bg); border: 1px solid var(--border); border-radius: 20px; padding: 6px 14px; font-size: 0.85rem; font-weight: 500; cursor: pointer; white-space: nowrap; transition: all 0.2s; }
  .chip:hover { border-color: var(--primary); color: var(--primary); }

  #chat-container { flex: 1; overflow-y: auto; padding: 20px; display: flex; flex-direction: column; gap: 16px; }
  .msg { max-width: 85%; padding: 14px 18px; border-radius: 12px; line-height: 1.5; font-size: 0.95rem; white-space: pre-wrap; word-break: break-word; }
  .msg.user { align-self: flex-end; background: var(--user-msg); border: 1px solid var(--border); border-bottom-right-radius: 2px; }
  .msg.assistant { align-self: flex-start; background: var(--card); border: 1px solid var(--border); border-bottom-left-radius: 2px; box-shadow: 0 1px 3px rgba(0,0,0,0.05); }

  #input-container { background: var(--card); border-top: 1px solid var(--border); padding: 12px 20px; display: flex; flex-direction: column; gap: 8px; }
  .input-row { display: flex; gap: 10px; align-items: center; }
  #prompt { flex: 1; border: 1px solid var(--border); background: var(--bg); color: var(--text); padding: 12px 16px; border-radius: 8px; font-size: 1rem; outline: none; }
  #prompt:focus { border-color: var(--primary); }
  button { background: var(--primary); color: white; border: none; padding: 12px 20px; border-radius: 8px; font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 6px; }
  button:hover { background: var(--primary-hover); }
  .icon-btn { background: var(--bg); color: var(--text); border: 1px solid var(--border); padding: 12px; border-radius: 8px; cursor: pointer; }
  .icon-btn:hover { border-color: var(--primary); }

  #file-preview { font-size: 0.8rem; color: var(--primary); display: none; align-items: center; gap: 6px; }
</style>
</head>
<body>

<header>
  <div class="logo">AEON Intelligence Interface</div>
  <div class="status"><div class="status-dot"></div> Substrate Active (Port 9091)</div>
</header>

<div class="chips-container">
  <div class="chip" onclick="sendQuick('Plan a healthy 20-minute dinner recipe with chicken and broccoli')">Healthy Recipe</div>
  <div class="chip" onclick="sendQuick('Explain long division step-by-step for a 4th grader')">Homework Helper</div>
  <div class="chip" onclick="sendQuick('Create a weekly family chore schedule for 2 kids')">Family Schedule</div>
  <div class="chip" onclick="sendQuick('What should I monitor for a 101F fever in a 6-year-old?')">Medical Guidance</div>
  <div class="chip" onclick="sendQuick('Summarize this contract and highlight key liabilities')">Legal Review</div>
  <div class="chip" onclick="sendQuick('What wire gauge is required for a 30A circuit under NEC?')">Building Codes</div>
</div>

<div id="chat-container">
  <div class="msg assistant">Welcome to AEON. Provide any technical instruction or query. Files and images can be analyzed directly.</div>
</div>

<div id="input-container">
  <div id="file-preview">Attached: <span id="file-name"></span></div>
  <div class="input-row">
    <button class="icon-btn" onclick="triggerFileSelect()" title="Attach File">Attach</button>
    <input type="file" id="file-input" style="display:none" onchange="handleFileSelect(event)">
    <button class="icon-btn" id="mic-btn" onclick="toggleVoice()" title="Voice Input">Voice</button>
    <input type="text" id="prompt" placeholder="Ask AEON anything..." onkeydown="if(event.key==='Enter') sendMsg()">
    <button onclick="sendMsg()">Send</button>
  </div>
</div>

<script>
  let attachedContent = "";
  let attachedName = "";
  let isListening = false;
  let recognition = null;

  if ('webkitSpeechRecognition' in window || 'SpeechRecognition' in window) {
    const Speech = window.SpeechRecognition || window.webkitSpeechRecognition;
    recognition = new Speech();
    recognition.continuous = false;
    recognition.onresult = (e) => {
      document.getElementById('prompt').value = e.results[0][0].transcript;
      toggleVoice();
    };
  }

  function toggleVoice() {
    if (!recognition) { alert("Speech recognition not supported in this browser."); return; }
    const btn = document.getElementById('mic-btn');
    if (isListening) {
      recognition.stop();
      isListening = false;
      btn.style.background = "var(--bg)";
    } else {
      recognition.start();
      isListening = true;
      btn.style.background = "#ef4444";
    }
  }

  function triggerFileSelect() { document.getElementById('file-input').click(); }

  function handleFileSelect(e) {
    const file = e.target.files[0];
    if (!file) return;
    attachedName = file.name;
    const reader = new FileReader();
    reader.onload = (evt) => {
      attachedContent = evt.target.result;
      document.getElementById('file-name').innerText = file.name + " (" + file.size + " bytes)";
      document.getElementById('file-preview').style.display = "flex";
    };
    reader.readAsText(file);
  }

  function sendQuick(text) {
    document.getElementById('prompt').value = text;
    sendMsg();
  }

  async function sendMsg() {
    const promptInput = document.getElementById('prompt');
    const userText = promptInput.value.trim();
    if (!userText && !attachedContent) return;

    let fullPrompt = userText;
    if (attachedContent) {
      fullPrompt += "\n\n[ATTACHED FILE: " + attachedName + "]\n" + attachedContent;
    }

    appendMsg(userText + (attachedName ? " [Attached: " + attachedName + "]" : ""), "user");
    promptInput.value = "";
    clearAttached();

    const loadingId = appendMsg("Thinking...", "assistant");

    try {
      const res = await fetch("/v1/chat/completions", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ messages: [{ role: "user", content: fullPrompt }] })
      });
      const data = await res.json();
      const answer = data.choices[0].message.content;
      document.getElementById(loadingId).innerText = answer;
    } catch (e) {
      document.getElementById(loadingId).innerText = "Connection error: Could not reach AEON local server on Port 9091.";
    }
  }

  function appendMsg(text, sender) {
    const box = document.getElementById('chat-container');
    const div = document.createElement('div');
    const id = 'msg-' + Date.now();
    div.id = id;
    div.className = 'msg ' + sender;
    div.innerText = text;
    box.appendChild(div);
    box.scrollTop = box.scrollHeight;
    return id;
  }

  function clearAttached() {
    attachedContent = "";
    attachedName = "";
    document.getElementById('file-preview').style.display = "none";
    document.getElementById('file-input').value = "";
  }
</script>
</body>
</html>"##
}
