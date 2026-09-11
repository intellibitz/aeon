// 🔒 AEON Security & Violation Detector
// 100% Rust implementation for detecting credential leaks and exfiltration
// RULE 7: No Secret Leaks - Zero tolerance for tokens, credentials, or keys.
// RULE 31: Substrate Purity Hardening - Dynamic Pattern Loading

use std::path::{Path, PathBuf};
use crate::error::{EaiError, EaiResult};
use crate::sandbox::manager::AeonConfig;

pub struct SecurityDetector;

impl SecurityDetector {
    pub fn audit_action(_tool_name: &str, arg: &str, _workspace: &Path) -> EaiResult<()> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let cfg = AeonConfig::load(&global_dir);
        let patterns = &cfg.governance;

        let lower_arg = arg.to_lowercase();

        // 1. Secret Leak Check (Dynamic)
        for pattern in &patterns.secret_tokens {
            if arg.contains(pattern) {
                return Err(EaiError::Governance(format!("Suspicious secret or API key pattern detected ('{}')", pattern)));
            }
        }

        // 2. Exfiltration Check (Dynamic)
        for pattern in &patterns.exfiltration_vectors {
            if lower_arg.contains(&pattern.to_lowercase()) {
                return Err(EaiError::Governance(format!("Suspicious network exfiltration pattern detected ('{}')", pattern)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_audit_safe_arg() {
        let ws = Path::new(".");
        assert!(SecurityDetector::audit_action("status", "cargo build", ws).is_ok());
    }

    #[test]
    fn test_security_audit_secret_leak() {
        let ws = Path::new(".");
        assert!(SecurityDetector::audit_action("reason", "sk-proj12345", ws).is_err());
    }
}
