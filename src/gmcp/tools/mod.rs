// 🔌 GMCP Universal Meta MCP Tool Registry
// 100% Pure Rust implementation for Dynamic MCP Server Proxying, Meta Tool Routing & Wasm Reflexes

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, RwLock, OnceLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::gmcp::client::GmcpClient;
use crate::gemi::hardware::HardwareProfiler;
use crate::gemi::models::ModelManager;
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
    fn execute(&self, arg: &str, workspace: &Path) -> EaiResult<String>;
}

/// Enum representing Meta-Tool Category in AEON Substrate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaCategory {
    SystemPrimitive,
    WorkspaceIo,
    McpProxy,
    WasmReflex,
}

/// Generic Meta-Tool Struct
pub struct MetaTool {
    pub tool_name: String,
    pub tool_desc: String,
    pub category: MetaCategory,
    pub handler: Arc<dyn Fn(&str, &Path) -> EaiResult<String> + Send + Sync>,
}

impl AeonTool for MetaTool {
    fn name(&self) -> String { self.tool_name.clone() }
    fn description(&self) -> String { self.tool_desc.clone() }
    fn execute(&self, arg: &str, workspace: &Path) -> EaiResult<String> {
        (self.handler)(arg, workspace)
    }
}

pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Arc<dyn AeonTool>>>,
}

impl ToolRegistry {
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<ToolRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let registry = ToolRegistry {
                tools: RwLock::new(HashMap::new()),
            };
            registry.bootstrap();
            registry
        })
    }

    fn bootstrap(&self) {
        let mut tools = self.tools.write().unwrap();

        // 🛡️ INTERNAL META-CAPABILITIES (Tier 0 & 1 Primitives)

        Self::register_meta_tool(&mut tools, "status", "AEON Substrate status report", MetaCategory::SystemPrimitive, |_arg, _ws| {
            let hardware = HardwareProfiler::get_profile();
            let mut out = format!("AEON Engine Version: {}\\n", crate::AEON_VERSION);
            out.push_str(&format!("System Environment: {} CPUs | RAM: {}GB | {}\\n", hardware.cpus, hardware.ram_gb, hardware.gpu_info));
            out.push_str("Status: Operational & Self-Aware.\\n");
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "identity", "AEON Alpha Brain identity report", MetaCategory::SystemPrimitive, |_arg, workspace| {
            let brain = crate::gawd::brain::AlphaBrainContext::initialize(workspace);
            let mut report = String::new();
            report.push_str("# aeon Alpha Brain - Self-Awareness Report\\n\\n");
            report.push_str("## 1. SELF (Compiled Binary Axiomatic Core)\\n");
            report.push_str(&format!("- Version: {}\\n", crate::gawd::self_core::AlphaSelf::VERSION));
            report.push_str(&format!("- Core Paradigm: {}\\n", crate::gawd::self_core::AlphaSelf::CORE_PARADIGM));
            report.push_str(&format!("- Axiom Rules: {}\\n", crate::gawd::self_core::AlphaSelf::RULES.len()));
            report.push_str(&format!("- Baked Components: {}\\n\\n", crate::gawd::self_core::AlphaSelf::COMPONENTS.len()));
            report.push_str("## 2. SYSTEM ENVIRONMENT\\n");
            report.push_str(&format!("- CPUs: {}\\n- RAM: {}GB\\n- Workspace: {}\\n", brain.system_cpus, brain.system_ram_gb, brain.workspace_path.display()));
            Ok(report)
        });

        Self::register_meta_tool(&mut tools, "list_models", "List available model substrates", MetaCategory::SystemPrimitive, |_arg, workspace| {
            let models = ModelManager::list_models(workspace);
            let mut out = format!("Active Model Substrates (Count: {})\\n\\n", models.len());
            for m in &models {
                out.push_str(&format!("- [{}] {} ({})\\n", if m.is_local { "LOCAL" } else { "CLOUD" }, m.name, m.model_id));
            }
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "scout_model", "Scout or install model substrate", MetaCategory::SystemPrimitive, |arg, _workspace| {
            if arg.trim().is_empty() {
                return Ok("Usage: scout_model <model_name_or_url>".to_string());
            }
            let res = ModelManager::install_model(arg.trim());
            Ok(res)
        });

        Self::register_meta_tool(&mut tools, "read_file", "Read file content in workspace", MetaCategory::WorkspaceIo, |arg, workspace| {
            let clean = arg.trim().trim_matches('"').trim_matches('\'');
            if clean.is_empty() { return Err(EaiError::Protocol("Usage: read_file <file_path>".into())); }
            let path = workspace.join(clean);
            if !path.is_file() {
                return Err(EaiError::Sandbox(format!("File not found: {}", path.display())));
            }
            let content = fs::read_to_string(&path).map_err(|e| EaiError::Sandbox(e.to_string()))?;
            Ok(content)
        });

        Self::register_meta_tool(&mut tools, "write_file", "Write content to workspace file", MetaCategory::WorkspaceIo, |arg, workspace| {
            let parts: Vec<&str> = arg.splitn(2, ' ').collect();
            if parts.len() < 2 { return Err(EaiError::Protocol("Usage: write_file <path> <content>".into())); }
            let path = workspace.join(parts[0].trim());
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(&path, parts[1]).map_err(|e| EaiError::Sandbox(e.to_string()))?;
            Ok(format!("Wrote to {}", parts[0].trim()))
        });

        Self::register_meta_tool(&mut tools, "exec_command", "Execute command in workspace", MetaCategory::WorkspaceIo, |arg, workspace| {
            let clean = arg.trim();
            if clean.is_empty() { return Err(EaiError::Protocol("Usage: exec_command <cmd>".into())); }
            let out = Command::new("sh").args(["-c", clean]).current_dir(workspace).output().map_err(|e| EaiError::Sandbox(e.to_string()))?;
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if !stderr.is_empty() && stdout.is_empty() {
                Ok(stderr)
            } else {
                Ok(stdout)
            }
        });

        Self::register_meta_tool(&mut tools, "mcp_registry", "List global MCP registry entries", MetaCategory::McpProxy, |_arg, _ws| {
            let entries = GmcpClient::fetch_global_registry();
            let mut out = format!("Global MCP Server Registry (Count: {})\\n\\n", entries.len());
            for e in &entries {
                out.push_str(&format!("- [{}] {}: {}\\n  Package: {}\\n", e.category, e.name, e.description, e.package));
            }
            Ok(out)
        });

        Self::register_meta_tool(&mut tools, "mcp_configure", "Configure external MCP server", MetaCategory::McpProxy, |arg, _ws| {
            let parts: Vec<&str> = arg.splitn(2, ' ').collect();
            if parts.is_empty() { return Err(EaiError::Protocol("Usage: mcp_configure <name> [package]".into())); }
            let name = parts[0];
            let package = parts.get(1).unwrap_or(&name);
            let res = GmcpClient::auto_configure_server(name, package);
            Ok(format!("MCP Server '{}' configuration status: {}", name, res))
        });

        // 🚢 DYNAMIC DISCOVERY: Synthesized Native Reflexes (Rule 11)
        crate::gmcp::reflexes::register_synthesized_reflexes(&mut tools);
    }

    fn register_meta_tool<F>(
        tools: &mut HashMap<String, Arc<dyn AeonTool>>,
        name: &str,
        desc: &str,
        category: MetaCategory,
        handler: F,
    ) where
        F: Fn(&str, &Path) -> EaiResult<String> + Send + Sync + 'static,
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

    pub fn execute_tool(name: &str, arg: &str, workspace: &Path) -> String {
        if name.contains(':') && !name.starts_with("ext_") {
            let parts: Vec<&str> = name.splitn(2, ':').collect();
            return GmcpClient::execute_external_tool(parts[0], parts[1], arg);
        }

        if name.starts_with("reflex_") {
            let wasm_name = format!("{}.wasm", name.trim_start_matches("reflex_"));
            if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
                let wasm_path = home.join(".aeon/reflexes").join(wasm_name);
                if wasm_path.exists() {
                    match crate::native::wasm::WasmHost::execute_reflex(&wasm_path, arg) {
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
            format!("[CAPABILITY_GAP] Tool '{}' missing from Meta-Substrate. Synthesizing reflex.", name)
        }
    }
}
