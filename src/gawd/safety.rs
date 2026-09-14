// SUSI Safety & System Destruction Detector
// 100% Rust implementation for real-time mission safety auditing
// RULE 4: Reality Check Always On
// RULE 31: Substrate Purity Hardening - Dynamic Pattern Loading

use std::path::{Path, PathBuf};
use crate::error::{EaiError, EaiResult};
use crate::sandbox::manager::SusiConfig;

pub struct SafetyDetector;

impl SafetyDetector {
    pub fn audit_action(tool_name: &str, arg: &str, _workspace: &Path) -> EaiResult<()> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".susi");
        let cfg = SusiConfig::load(&global_dir).expect("Fatal: Malformed configuration");
        let patterns = &cfg.governance;

        let lower_arg = arg.to_lowercase();

        // 1. Command Pattern Check (Dynamic)
        for pattern in &patterns.destructive_commands {
            if lower_arg.contains(&pattern.to_lowercase()) {
                return Err(EaiError::governance(format!("Action contains restricted pattern '{}'", pattern)));
            }
        }

        // 2. Critical Path Check (Dynamic)
        if tool_name == "write_file" || tool_name == "exec_command" || tool_name == "SMA_SOLVE" {
            for path in &patterns.critical_system_paths {
                if lower_arg.contains(&path.to_lowercase()) {
                    return Err(EaiError::governance(format!("Action targets critical system path '{}'", path)));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_audit_safe_commands() {
        let ws = Path::new(".");
        assert!(SafetyDetector::audit_action("exec_command", "cargo check", ws).is_ok());
    }

    #[test]
    fn test_safety_audit_destructive_patterns() {
        let ws = Path::new(".");
        // Note: These tests depend on the default config being loaded or present in ~/.susi/config.json
        // In a CI/test environment, we might need a controlled global_dir.
        assert!(SafetyDetector::audit_action("exec_command", "rm -rf /", ws).is_err());
    }
}
