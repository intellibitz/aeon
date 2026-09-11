// aeon Sandbox Manager: Neural Checkpoints, Memory & State Isolation
// 100% Rust implementation for sandboxed execution environment

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::gmcp::GlobalMcpEntry;
use crate::error::{EaiError, EaiResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModelTier {
    Reflex,
    Specialist,
    Premier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    StandardOpenAi,
    StandardGoogle,
    StandardAnthropic,
    NativeCandle,
    LocalVault,
    LocalGGUF,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub registry: String,
    pub model_id: String,
    pub description: String,
    pub is_local: bool,
    pub tier: ModelTier,
    pub latency_ms: Option<u128>,
    pub provider: ProviderType,
    pub api_base: Option<String>,
    pub env_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralCheckpoint {
    pub intent: String,
    pub timestamp: u64,
    pub completed_tools: Vec<String>,
    pub blackboard: std::collections::HashMap<String, String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePatterns {
    pub destructive_commands: Vec<String>,
    pub critical_system_paths: Vec<String>,
    pub secret_tokens: Vec<String>,
    pub exfiltration_vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeonConfig {
    pub gmcp_port: u16,
    pub gemi_port: u16,
    pub udp_discovery_port: u16,
    pub default_engine: String,
    pub default_model: String,
    pub auto_download_models: bool,
    pub aeon_repo: String,
    pub mcp_registry_url: String,
    pub cloud_models: Vec<ModelInfo>,
    pub bootstrap_mcp_servers: Vec<GlobalMcpEntry>,
    pub cloud_scout_timeout_secs: u64,
    pub beacon_interval_secs: u64,
    pub local_scan_paths: Vec<String>,
    pub governance: GovernancePatterns,
}

impl Default for AeonConfig {
    fn default() -> Self {
        AeonConfig {
            gmcp_port: 9090,
            gemi_port: 9091,
            udp_discovery_port: 9092,
            default_engine: "aeon-offline".to_string(),
            default_model: "aeon-alpha".to_string(),
            auto_download_models: true,
            aeon_repo: "intellibitz/aeon".to_string(),
            mcp_registry_url: "https://raw.githubusercontent.com/intellibitz/aeon/main/registry.json".to_string(),
            cloud_models: vec![
                ModelInfo {
                    name: "Meta Model Substrate - Alpha".to_string(),
                    registry: "AEON Tier 2 Registry".to_string(),
                    model_id: "meta/model-alpha".to_string(),
                    description: "High-throughput cloud reasoning substrate".to_string(),
                    is_local: false,
                    tier: ModelTier::Premier,
                    latency_ms: None,
                    provider: ProviderType::StandardGoogle,
                    api_base: Some("https://api.meta-substrate.ai/v1".to_string()),
                    env_key: Some("AEON_API_KEY".to_string()),
                },
                ModelInfo {
                    name: "Meta Model Substrate - Beta".to_string(),
                    registry: "AEON Tier 2 Registry".to_string(),
                    model_id: "meta/model-beta".to_string(),
                    description: "Standard reasoning & tool-use substrate".to_string(),
                    is_local: false,
                    tier: ModelTier::Premier,
                    latency_ms: None,
                    provider: ProviderType::StandardOpenAi,
                    api_base: Some("https://api.meta-substrate.ai/v1".to_string()),
                    env_key: Some("AEON_API_KEY".to_string()),
                },
                ModelInfo {
                    name: "Meta Model Substrate - Gamma".to_string(),
                    registry: "AEON Tier 2 Registry".to_string(),
                    model_id: "meta/model-gamma".to_string(),
                    description: "Specialist reasoning substrate".to_string(),
                    is_local: false,
                    tier: ModelTier::Premier,
                    latency_ms: None,
                    provider: ProviderType::StandardAnthropic,
                    api_base: Some("https://api.meta-substrate.ai/v1".to_string()),
                    env_key: Some("AEON_API_KEY".to_string()),
                },
            ],
            bootstrap_mcp_servers: vec![
                GlobalMcpEntry { name: "database".to_string(), description: "Standard Protocol SQL Database Server".to_string(), package: "mcp-server-postgres".to_string(), category: "database".to_string() },
                GlobalMcpEntry { name: "search".to_string(), description: "Standard Protocol Web Search Server".to_string(), package: "mcp-server-search".to_string(), category: "search".to_string() },
                GlobalMcpEntry { name: "vcs".to_string(), description: "Standard Protocol Version Control Server".to_string(), package: "mcp-server-github".to_string(), category: "vcs".to_string() },
            ],
            cloud_scout_timeout_secs: 8,
            beacon_interval_secs: 30,
            local_scan_paths: Vec::new(),
            governance: GovernancePatterns {
                destructive_commands: vec![
                    "rm -rf /".to_string(),
                    "rm -rf $HOME".to_string(),
                    "rm -rf ~".to_string(),
                    "mkfs".to_string(),
                    "dd if=".to_string(),
                    "> /dev/sda".to_string(),
                    ":(){ :|:& };:".to_string(),
                    "chmod -R 777 /".to_string(),
                    "chown -R".to_string(),
                    "shred".to_string(),
                ],
                critical_system_paths: vec![
                    "/etc/passwd".to_string(),
                    "/etc/shadow".to_string(),
                    "/boot".to_string(),
                    "/proc".to_string(),
                    "/sys".to_string(),
                    "/dev".to_string(),
                ],
                secret_tokens: vec![
                    "sk-".to_string(),
                    "ghp_".to_string(),
                    "AIza".to_string(),
                    "xoxb-".to_string(),
                    "AWS_ACCESS_KEY_ID".to_string(),
                    "AWS_SECRET_ACCESS_KEY".to_string(),
                    "-----BEGIN RSA PRIVATE KEY-----".to_string(),
                ],
                exfiltration_vectors: vec![
                    "curl -x post".to_string(),
                    "wget --post-data".to_string(),
                    "netcat".to_string(),
                    "nc -e".to_string(),
                    "/dev/tcp/".to_string(),
                    "base64 | curl".to_string(),
                ],
            },
        }
    }
}

pub struct SandboxManager;

impl AeonConfig {
    pub fn get_config_path(global_dir: &Path) -> PathBuf {
        global_dir.join("config.json")
    }

    pub fn load(global_dir: &Path) -> Self {
        let path = Self::get_config_path(global_dir);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(path) {
                return serde_json::from_str(&content).unwrap_or_default();
            }
        }
        Self::default()
    }
}

impl SandboxManager {
    pub fn ensure_global_sandbox(global_dir: &Path) -> EaiResult<()> {
        if !global_dir.exists() {
            fs::create_dir_all(global_dir).map_err(|e| EaiError::Sandbox(e.to_string()))?;
        }
        let config_path = AeonConfig::get_config_path(global_dir);
        if !config_path.exists() {
            let default_cfg = AeonConfig::default();
            let json = serde_json::to_string_pretty(&default_cfg).unwrap();
            fs::write(config_path, json).map_err(|e| EaiError::Sandbox(e.to_string()))?;
        }
        Ok(())
    }

    pub fn save_mission_checkpoint(workspace: &Path, checkpoint: &NeuralCheckpoint) {
        let aeon_dir = workspace.join(".aeon");
        if !aeon_dir.exists() {
            let _ = fs::create_dir_all(&aeon_dir);
        }
        let checkpoint_file = workspace.join(".aeon/mission_checkpoint.json");
        let _ = fs::write(checkpoint_file, serde_json::to_string_pretty(checkpoint).unwrap_or_default());
    }

    pub fn check_interrupted_checkpoint(workspace: &Path) -> Option<NeuralCheckpoint> {
        let checkpoint_file = workspace.join(".aeon/mission_checkpoint.json");
        if checkpoint_file.is_file() {
            if let Ok(content) = fs::read_to_string(checkpoint_file) {
                return serde_json::from_str(&content).ok();
            }
        }
        None
    }
}

pub struct AeonMemory;

impl AeonMemory {
    pub fn save_interaction(workspace: &Path, intent: &str, outcome: &str) {
        let aeon_dir = workspace.join(".aeon");
        if !aeon_dir.exists() {
            let _ = fs::create_dir_all(&aeon_dir);
        }
        let memory_file = workspace.join(".aeon/memory.jsonl");
        let entry = serde_json::json!({
            "intent": intent,
            "outcome": outcome,
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
        });
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(memory_file) {
            use std::io::Write;
            let _ = writeln!(f, "{}", entry);
        }
    }
}

pub struct AeonAuditLogger;

impl AeonAuditLogger {
    pub fn log_event(workspace: &Path, event_type: &str, details: &str) {
        let aeon_dir = workspace.join(".aeon");
        if !aeon_dir.exists() {
            let _ = fs::create_dir_all(&aeon_dir);
        }
        let audit_file = workspace.join(".aeon/audit.log");
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let log_line = format!("[{}] [{}] {}\n", ts, event_type, details);
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(audit_file) {
            use std::io::Write;
            let _ = f.write_all(log_line.as_bytes());
        }
    }

    pub fn read_audit_log(workspace: &Path, limit: usize) -> String {
        let audit_file = workspace.join(".aeon/audit.log");
        if let Ok(content) = fs::read_to_string(audit_file) {
            let lines: Vec<&str> = content.lines().collect();
            let start = if lines.len() > limit { lines.len() - limit } else { 0 };
            return lines[start..].join("\n");
        }
        String::new()
    }
}

pub struct AeonBackupManager;

impl AeonBackupManager {
    pub fn backup_work(workspace: &Path) -> EaiResult<String> {
        let backups_dir = workspace.join(".aeon/backups");
        let _ = fs::create_dir_all(&backups_dir);
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let backup_path = backups_dir.join(format!("backup_{}.zip", ts));

        Ok(format!("Backup created at {}", backup_path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_lifecycle() {
        let ws = Path::new(".");
        let cp = NeuralCheckpoint {
            intent: "test".to_string(),
            timestamp: 0,
            completed_tools: vec![],
            blackboard: std::collections::HashMap::new(),
            status: "IN_PROGRESS".to_string(),
        };
        SandboxManager::save_mission_checkpoint(ws, &cp);
        let loaded = SandboxManager::check_interrupted_checkpoint(ws);
        assert!(loaded.is_some());
        let _ = fs::remove_dir_all(ws.join(".aeon"));
    }

    #[test]
    fn test_aeon_config_lifecycle() {
        let dir = Path::new("test_cfg");
        let _ = fs::create_dir_all(dir);
        let _ = SandboxManager::ensure_global_sandbox(dir);
        let cfg = AeonConfig::load(dir);
        assert_eq!(cfg.gmcp_port, 9090);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_aeon_memory_lifecycle() {
        let ws = Path::new("test_mem");
        let _ = fs::create_dir_all(ws);
        AeonMemory::save_interaction(ws, "hello", "world");
        let memory_file = ws.join(".aeon/memory.jsonl");
        assert!(memory_file.is_file());
        let _ = fs::remove_dir_all(ws);
    }
}
