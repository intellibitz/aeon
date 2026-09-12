// GMCP Universal Meta MCP Tool Registry
// 100% Pure Rust implementation for Dynamic MCP Server Proxying, Meta Tool Routing & Wasm Reflexes

use std::fs;
use std::path::{Path, PathBuf, Component};
use std::process::Command;
use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::gmcp::client::GmcpClient;
use crate::gemi::hardware::HardwareProfiler;
use crate::gemi::models::ModelManager;
use crate::gemi::engine::NativeInferenceEngine;
use crate::error::{EaiError, EaiResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
}

/// Dynamic Trait for AEON Substrate Tools
pub trait AeonTool: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn execute(&self, arg: &serde_json::Value, workspace: &Path) -> EaiResult<String>;
}

/// Enum representing Meta-Tool Category in AEON Substrate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaCategory {
    SystemPrimitive,
    WorkspaceIo,
    McpProxy,
    WasmReflex,
    IntelligenceBridge,
}

/// Generic Meta-Tool Struct
pub struct MetaTool {
    pub tool_name: String,
    pub tool_desc: String,
    pub category: MetaCategory,
    pub handler: Arc<dyn Fn(&serde_json::Value, &Path) -> EaiResult<String> + Send + Sync>,
}

impl AeonTool for MetaTool {
    fn name(&self) -> String { self.tool_name.clone() }
    fn description(&self) -> String { self.tool_desc.clone() }
    fn execute(&self, arg: &serde_json::Value, workspace: &Path) -> EaiResult<String> {
        (self.handler)(arg, workspace)
    }
}

/// Ensure path is normalized and contained within workspace
fn secure_path(workspace: &Path, user_path: &str) -> EaiResult<PathBuf> {
    let user_path = user_path.trim().trim_matches('"').trim_matches('\'');
    let path = PathBuf::from(user_path);

    if path.is_absolute() {
        return Err(EaiError::filesystem("Absolute paths not allowed"));
    }

    let canonical_workspace = workspace.canonicalize()
        .map_err(|e| EaiError::filesystem(format!("Workspace error: {}", e)))?;

    let full_path = workspace.join(&path);
    let canonical_path = full_path.canonicalize()
        .ok()
        .unwrap_or_else(|| full_path.clone());

    if !canonical_path.starts_with(&canonical_workspace) {
        return Err(EaiError::filesystem(format!("Path escape attempt: {}", user_path)));
    }

    for component in path.components() {
        if let Component::ParentDir = component {
            return Err(EaiError::filesystem("Parent directory traversal not allowed"));
        }
    }

    Ok(canonical_path)
}

pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Arc<dyn AeonTool>>>,
    locks: Arc<Mutex<HashMap<String, u64>>>,
}

impl ToolRegistry {
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<ToolRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let registry = ToolRegistry {
                tools: RwLock::new(HashMap::new()),
                locks: Arc::new(Mutex::new(HashMap::new())),
            };
            registry.bootstrap();
            registry
        })
    }

    fn bootstrap(&self) {
        let mut tools = self.tools.write().unwrap();

        // INTERNAL META-CAPABILITIES (Tier 0 & 1 Primitives)

        Self::register_meta_tool(&mut tools, "status", "AEON Substrate status report", MetaCategory::SystemPrimitive, |_arg, _ws| {
            let hardware = HardwareProfiler::get_profile();
            let mut out = format!("AEON Engine Version: {}\n", crate::AEON_VERSION);
            out.push_str(&format!("System Environment: {} CPUs | RAM: {}GB | {}\n", hardware.cpus, hardware.ram_gb, hardware.gpu_info));
            out.push_str("Status: Operational.\n");
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "identity", "AEON substrate identity report", MetaCategory::SystemPrimitive, |_arg, workspace| {
            let brain = crate::gawd::brain::AlphaBrainContext::initialize(workspace);
            let mut report = String::new();
            report.push_str("# aeon Substrate - Identity Report\n\n");
            report.push_str("## 1. CORE CONFIGURATION (Compiled Binary Axiomatic Core)\n");
            report.push_str(&format!("- Version: {}\n", crate::gawd::self_core::AlphaSelf::VERSION));
            report.push_str(&format!("- Core Paradigm: {}\n", crate::gawd::self_core::AlphaSelf::CORE_PARADIGM));
            report.push_str(&format!("- Axiom Rules: {}\n", crate::gawd::self_core::AlphaSelf::RULES.len()));
            report.push_str(&format!("- AoA Pillar: {}\n", crate::gawd::self_core::AlphaSelf::AOA_COMPONENTS.len()));
            report.push_str(&format!("- Agents Pillar: {}\n", crate::gawd::self_core::AlphaSelf::AGENT_COMPONENTS.len()));
            report.push_str(&format!("- Engines Pillar: {}\n", crate::gawd::self_core::AlphaSelf::ENGINE_COMPONENTS.len()));
            report.push_str(&format!("- Models Pillar: {}\n", crate::gawd::self_core::AlphaSelf::MODEL_COMPONENTS.len()));
            report.push_str(&format!("- MCPs Pillar: {}\n\n", crate::gawd::self_core::AlphaSelf::MCP_COMPONENTS.len()));
            report.push_str("## 2. SYSTEM ENVIRONMENT\n");
            report.push_str(&format!("- CPUs: {}\n- RAM: {}GB\n- Workspace: {}\n", brain.system_cpus, brain.system_ram_gb, brain.workspace_path.display()));
            Ok(report)
        });

        Self::register_meta_tool(&mut tools, "distill_genome", "Distill the hard-compiled genome into the Tier 2 reasoning model", MetaCategory::SystemPrimitive, |_arg, workspace| {
            match crate::gawd::reason_trainer::ReasoningTrainer::audit_reasoning_substrate(workspace) {
                Ok(report) => Ok(format!("# Genome Distillation Successful\n\n{}", report)),
                Err(e) => Ok(format!("# Genome Distillation Failed\n\nError: {}", e)),
            }
        });

        Self::register_meta_tool(&mut tools, "self_validate", "Execute autonomous substrate self-validation", MetaCategory::SystemPrimitive, |_arg, workspace| {
            match crate::daemon::runtime_admin::AeonRuntimeAdmin::execute_autonomous_self_validation(workspace) {
                Ok(report) => Ok(format!("# Substrate Self-Validation Successful\n\n{}", report)),
                Err(e) => Ok(format!("# Substrate Self-Validation Failed\n\nError: {}", e)),
            }
        });

        Self::register_meta_tool(&mut tools, "list_models", "List available model substrates", MetaCategory::SystemPrimitive, |_arg, workspace| {
            let models = ModelManager::list_models(workspace);
            let mut out = format!("Active Model Substrates (Count: {})\n\n", models.len());
            for m in &models {
                out.push_str(&format!("- [{}] {} ({})\n", if m.is_local { "LOCAL" } else { "CLOUD" }, m.name, m.model_id));
            }
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "scout_model", "Scout or install model substrate", MetaCategory::SystemPrimitive, |arg, _workspace| {
            let arg_s = arg.as_str().unwrap_or("");
            if arg_s.trim().is_empty() {
                return Ok("Usage: scout_model <model_name_or_url>".to_string());
            }
            let res = ModelManager::install_model(arg_s.trim());
            Ok(res)
        });

        Self::register_meta_tool(&mut tools, "train_reflexes", "Manually trigger native neural reflex distillation", MetaCategory::SystemPrimitive, |_arg, workspace| {
            crate::gawd::reflex_trainer::ReflexTrainer::force_train(workspace).map(|r| r)
        });

        Self::register_meta_tool(&mut tools, "read_file", "Read file content in workspace", MetaCategory::WorkspaceIo, |arg, workspace| {
            let arg_s = arg.as_str().ok_or_else(|| EaiError::protocol("Invalid argument type"))?;
            let path = secure_path(workspace, arg_s)?;
            let content = fs::read_to_string(&path).map_err(|e| EaiError::filesystem(e.to_string()))?;
            Ok(content)
        });

        Self::register_meta_tool(&mut tools, "write_file", "Write content to workspace file", MetaCategory::WorkspaceIo, |arg, workspace| {
            let path_s = arg.get("path").and_then(|v| v.as_str());
            let content_s = arg.get("content").and_then(|v| v.as_str());

            if let (Some(p), Some(content)) = (path_s, content_s) {
                let dest = secure_path(workspace, p)?;
                if let Some(parent) = dest.parent() { let _ = fs::create_dir_all(parent); }
                fs::write(&dest, content).map_err(|e| EaiError::filesystem(e.to_string()))?;
                Ok(format!("Wrote to {}", p))
            } else {
                Err(EaiError::protocol("Usage: write_file {path: <path>, content: <content>}"))
            }
        });

        Self::register_meta_tool(&mut tools, "exec_command", "Execute command in workspace", MetaCategory::WorkspaceIo, |arg, workspace| {
            let arg_s = arg.as_str().ok_or_else(|| EaiError::protocol("Invalid argument type"))?;
            let clean = arg_s.trim();
            if clean.is_empty() { return Err(EaiError::protocol("Usage: exec_command <cmd>")); }

            let args = shlex::split(clean).ok_or_else(|| EaiError::protocol("Invalid shell syntax"))?;
            if args.is_empty() { return Err(EaiError::protocol("Command cannot be empty")); }

            let out = Command::new(&args[0])
                .args(&args[1..])
                .current_dir(workspace)
                .output()
                .map_err(|e| EaiError::process(format!("Exec failed: {}", e)))?;

            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if !out.status.success() {
                Err(EaiError::process(stderr))
            } else {
                Ok(stdout)
            }
        });

        Self::register_meta_tool(&mut tools, "mcp_registry", "List global MCP registry entries", MetaCategory::McpProxy, |_arg, _ws| {
            let entries = GmcpClient::fetch_global_registry();
            let mut out = format!("Global MCP Server Registry (Count: {})\n\n", entries.len());
            for e in &entries {
                out.push_str(&format!("- [{}] {}: {}\n  Package: {}\n", e.category, e.name, e.description, e.package));
            }
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "mcp_configure", "Configure external MCP server", MetaCategory::McpProxy, |arg, _ws| {
            let name = arg.get("name").and_then(|v| v.as_str())
                .or_else(|| arg.as_str().and_then(|s| s.split_whitespace().next()));
            let package = arg.get("package").and_then(|v| v.as_str())
                .or_else(|| arg.as_str().and_then(|s| s.split_whitespace().nth(1)));

            if let Some(n) = name {
                let p = package.unwrap_or(n);
                let res = GmcpClient::auto_configure_server(n, p);
                Ok(format!("MCP Server '{}' configuration status: {}", n, res))
            } else {
                Err(EaiError::protocol("Usage: mcp_configure {name: <name>, package: <package>}"))
            }
        });

        Self::register_meta_tool(&mut tools, "agent_register", "Dynamically register a new agent profile", MetaCategory::IntelligenceBridge, |arg, _ws| {
            let name = arg.get("name").and_then(|v| v.as_str());
            let desc = arg.get("description").and_then(|v| v.as_str());
            let cats = arg.get("categories").and_then(|v| v.as_str());

            if let (Some(n), Some(d), Some(c)) = (name, desc, cats) {
                let profile = crate::gawd::agents::AgentProfile {
                    name: n.to_string(),
                    description: d.to_string(),
                    categories: c.split(',').map(|s| s.trim().to_string()).collect(),
                    semantic_anchors: Vec::new(),
                    base_rank: 0.8,
                };
                crate::gawd::agents::AgentMetaRegistry::global().register_agent(profile);
                Ok(format!("Successfully registered agent: {}", n))
            } else {
                // Fallback for flat string
                let arg_s = arg.as_str().unwrap_or("");
                let parts: Vec<&str> = arg_s.splitn(3, ' ').collect();
                if parts.len() < 3 { return Err(EaiError::protocol("Usage: agent_register {name, description, categories}")); }

                let profile = crate::gawd::agents::AgentProfile {
                    name: parts[0].to_string(),
                    description: parts[1].to_string(),
                    categories: parts[2].split(',').map(|s| s.trim().to_string()).collect(),
                    semantic_anchors: Vec::new(),
                    base_rank: 0.8,
                };

                crate::gawd::agents::AgentMetaRegistry::global().register_agent(profile);
                Ok(format!("Successfully registered agent: {}", parts[0]))
            }
        });

        Self::register_meta_tool(&mut tools, "reason", "Execute native local reasoning substrate", MetaCategory::SystemPrimitive, |arg, _workspace| {
             // Aspiration 8: Pure Rust-Native Inference (Hardened)
             let arg_s = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
             crate::gemi::engine::AeonGgufEngine.run_inference(&arg_s)
        });

        // 5. Meta-Intelligence Bridge Primitives
        Self::register_meta_tool(&mut tools, "power_reason", "Delegate complex reasoning to Power-Tier MCP remotes", MetaCategory::IntelligenceBridge, |arg, _ws| {
            let arg_s = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
            if arg_s.trim().is_empty() {
                return Err(EaiError::protocol("Usage: power_reason <complex_intent>"));
            }

            // Meta-Scout: Identify a reasoning-capable MCP server
            let remotes = GmcpClient::scout_reasoning_remotes();
            if let Some(best_remote) = remotes.first() {
                let res = GmcpClient::execute_external_tool(best_remote, "reason", &arg_s);
                if !res.contains("[FAIL]") {
                    return Ok(res);
                }
            }

            Err(EaiError::protocol("No Power-Tier reasoning remotes configured or available. AEON local reasoning active."))
        });

        Self::register_meta_tool(&mut tools, "meta_scout_agents", "Discover agent capabilities from connected remotes", MetaCategory::IntelligenceBridge, |_arg, _ws| {
            let remotes = GmcpClient::list_external_tools();
            let mut report = "Discovered Meta-Agent Capabilities:\n\n".to_string();
            for r in remotes {
                if r.name.contains("agent") || r.name.contains("swarm") {
                    report.push_str(&format!("- [REMOTE] {}: {}\n", r.name, r.description));
                }
            }
            Ok(report)
        });

        Self::register_meta_tool(&mut tools, "meta_rank_agents", "Report current agent expertise hierarchy", MetaCategory::IntelligenceBridge, |_arg, _ws| {
            let registry = crate::gawd::agents::AgentMetaRegistry::global();
            let agents = registry.list_agents();
            let mut report = "AEON Expertise Hierarchy:\n\n".to_string();
            for a in agents {
                report.push_str(&format!("- [AGENT] {} (Base Rank: {:.2}): {}\n", a.name, a.base_rank, a.description));
            }
            Ok(report)
        });

        // DYNAMIC DISCOVERY: Synthesized Native Reflexes (Rule 11)
        crate::gmcp::reflexes::register_synthesized_reflexes(&mut tools);

        // Zero-Config Auto-Link: Ensure essential MCP tools are mapped
        Self::auto_link_essential_mcp_servers();
    }

    /// Zero-Config Autonomous Tool Linking (Rule 21 Hardening)
    pub fn auto_link_essential_mcp_servers() {
        let registry = GmcpClient::fetch_global_registry();
        let config_path = GmcpClient::get_config_path();

        let config_exists = config_path.exists();
        let mut essential_found = false;

        if config_exists {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str::<super::McpConfig>(&content) {
                    essential_found = !config.mcp_servers.is_empty();
                }
            }
        }

        if !essential_found {
            eprintln!("[GMCP] No external tools configured. Auto-linking essential substrates (Registry count: {})...", registry.len());
            // Link search and filesystem by default as they are foundational
            let essentials = ["brave_search", "filesystem", "google_search", "github", "google_maps"];
            for e in essentials {
                if let Some(entry) = registry.iter().find(|r| r.name == e) {
                    let res = GmcpClient::auto_configure_server(&entry.name, &entry.package);
                    eprintln!("  - Linked {}: {}", e, res);
                } else {
                    eprintln!("  - Essential substrate '{}' not found in registry.", e);
                }
            }
        }
    }

    fn register_meta_tool<F>(
        tools: &mut HashMap<String, Arc<dyn AeonTool>>,
        name: &str,
        desc: &str,
        category: MetaCategory,
        handler: F,
    ) where
        F: Fn(&serde_json::Value, &Path) -> EaiResult<String> + Send + Sync + 'static,
    {
        let tool = MetaTool {
            tool_name: name.to_string(),
            tool_desc: desc.to_string(),
            category,
            handler: Arc::new(handler),
        };
        tools.insert(name.to_string(), Arc::new(tool));
    }

    pub fn list_tools() -> Vec<McpTool> {
        let registry = Self::global();
        let mut tools: Vec<McpTool> = registry.tools.read().unwrap()
            .values()
            .map(|t| McpTool { name: t.name(), description: t.description() })
            .collect();

        tools.extend(GmcpClient::list_external_tools());

        if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
            let reflex_dir = home.join(".aeon/reflexes");
            if let Ok(entries) = fs::read_dir(&reflex_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "wasm") {
                        if let Ok(name) = entry.file_name().into_string() {
                            tools.push(McpTool {
                                name: format!("reflex_{}", name.replace(".wasm", "")),
                                description: "Dynamic Wasm neural reflex tool".to_string(),
                            });
                        }
                    }
                }
            }
        }

        tools.sort_by(|a, b| a.name.cmp(&b.name));
        tools.dedup_by(|a, b| a.name == b.name);
        tools
    }

    pub fn exists(name: &str) -> bool {
        let registry = Self::global();
        let tools = registry.tools.read().unwrap();
        if tools.contains_key(name) {
            return true;
        }
        let lower_name = name.to_lowercase();
        if lower_name.contains(':') || lower_name.starts_with("ext_") || lower_name.starts_with("reflex_") {
            return Self::list_tools().iter().any(|t| t.name == name || t.name.starts_with(name));
        }
        false
    }

    pub fn execute_tool(name: &str, arg: &serde_json::Value, workspace: &Path) -> String {
        if name.contains(':') && !name.starts_with("ext_") {
            let parts: Vec<&str> = name.splitn(2, ':').collect();
            let arg_str = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
            return GmcpClient::execute_external_tool(parts[0], parts[1], &arg_str);
        }

        if name.starts_with("reflex_") {
            let wasm_name = format!("{}.wasm", name.trim_start_matches("reflex_"));
            if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
                let wasm_path = home.join(".aeon/reflexes").join(wasm_name);
                if wasm_path.exists() {
                    let arg_str = if let Some(s) = arg.as_str() { s.to_string() } else { arg.to_string() };
                    match crate::native::wasm::WasmHost::execute_reflex(&wasm_path, &arg_str) {
                        Ok(res) => return res,
                        Err(e) => return format!("Reflex Error: {}", e),
                    }
                }
            }
        }

        let registry = Self::global();
        let tools = registry.tools.read().unwrap();
        if let Some(tool) = tools.get(name) {
            match tool.execute(arg, workspace) {
                Ok(res) => res,
                Err(e) => format!("{}", e),
            }
        } else {
            // Self-Healing Protocol (Rule 21): Attempt autonomous resolution
            if let Ok(provisioned_res) = Self::resolve_capability_gap(name) {
                if provisioned_res == "SUCCESS_CONFIGURED" {
                     return format!("[RECOVERY] Capability '{}' was missing and autonomously provisioned. Please retry the mission.", name);
                }
            }
            format!("[CAPABILITY_GAP] Tool '{}' missing from Meta-Substrate. Report to Creator for native substrate hardening.", name)
        }
    }

    /// Autonomous Capability Resolution (Rule 21)
    pub fn resolve_capability_gap(name: &str) -> EaiResult<String> {
        let server_name = name.split(':').next().unwrap_or(name);

        // Proactive Semantic Scout (Tier 1 Hardening)
        // If the tool name isn't an exact match, we search for semantic overlaps in the registry
        let registry = GmcpClient::fetch_global_registry();
        if let Some(entry) = registry.iter().find(|e| e.name == server_name || e.description.to_lowercase().contains(server_name)) {
            return Ok(GmcpClient::auto_configure_server(&entry.name, &entry.package));
        }

        let res = GmcpClient::provision_tool_package(server_name);
        Ok(res)
    }

    pub fn acquire_meta_lock(resource_id: &str) -> bool {
        if !Self::acquire_local_lock(resource_id) {
            return false;
        }

        // Distributed Resource Sovereignty: Broadcast to peers
        if !crate::gawd::amas::AmaSupervisor::broadcast_lock_request(resource_id) {
            Self::release_meta_lock(resource_id);
            return false;
        }

        true
    }

    pub fn acquire_local_lock(resource_id: &str) -> bool {
        let registry = Self::global();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        let mut locks = registry.locks.lock().unwrap();
        if let Some(&timestamp) = locks.get(resource_id) {
            // Lease-Based Timed Locks (300s TTL)
            if now - timestamp < 300 {
                return false;
            }
        }
        locks.insert(resource_id.to_string(), now);
        true
    }

    pub fn release_meta_lock(resource_id: &str) {
        let registry = Self::global();
        let mut locks = registry.locks.lock().unwrap();
        locks.remove(resource_id);
    }
}
