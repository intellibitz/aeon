// AEON Protocol Knowledge Base (PKB)
// Tier 0: Reflex Data Synthesis for AEON-Alpha Training

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::error::EaiResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolReflex {
    pub intent: String,
    pub action: String,
    pub context: String,
    pub verified: bool,
}

pub struct ProtocolKnowledgeBase;

impl ProtocolKnowledgeBase {
    /// Ingests audit logs to synthesize new neural reflex training data
    pub fn synthesize_training_data(workspace: &Path) -> EaiResult<Vec<ProtocolReflex>> {
        let mut reflexes = Vec::new();
        let log_content = crate::sandbox::manager::AeonAuditLogger::read_audit_log(workspace, 500);

        for line in log_content.lines() {
            if line.contains("[MISSION_START]") {
                let intent = line.split("[MISSION_START]").nth(1).unwrap_or("").trim().to_string();
                if intent.len() > 5 {
                    reflexes.push(ProtocolReflex {
                        intent,
                        action: "PENDING_DISTILLATION".to_string(),
                        context: "SYNTHETIC_AUDIT_DERIVED".to_string(),
                        verified: false,
                    });
                }
            }
        }
        Ok(reflexes)
    }

    pub fn bootstrap_alpha_reflexes() -> Vec<ProtocolReflex> {
        vec![
            ProtocolReflex {
                intent: "install".to_string(),
                action: "SandboxManager::ensure_global_sandbox".to_string(),
                context: "CORE_INITIALIZATION".to_string(),
                verified: true,
            },
            ProtocolReflex {
                intent: "audit compliance".to_string(),
                action: "AeonAdmin::audit_compliance".to_string(),
                context: "GOVERNANCE_ENFORCEMENT".to_string(),
                verified: true,
            }
        ]
    }

    pub fn export_reflex_dataset(workspace: &Path) -> EaiResult<String> {
        let mut reflexes = Self::bootstrap_alpha_reflexes();
        let synthetic = Self::synthesize_training_data(workspace)?;
        reflexes.extend(synthetic);

        let data = serde_json::to_string_pretty(&reflexes).map_err(|e| crate::error::EaiError::Internal(e.to_string()))?;
        let export_path = workspace.join(".aeon/reflex_dataset.json");
        std::fs::write(&export_path, data)?;

        Ok(format!("Exported {} neural reflexes to {}", reflexes.len(), export_path.display()))
    }

    pub fn generate_synthetic_intent_pair(intent: &str, workspace: &Path) -> EaiResult<String> {
        // High-fidelity synthetic generation for Tier 0 reflex training
        let mut pair = format!("INTENT: {}\n", intent);

        let agents = super::agents::GawdAgentFleet::synthesize_fleet(intent);
        for agent in agents {
            if agent.name() == "SafetyAgent" || agent.name() == "ContextAgent" {
                let bb = std::sync::Arc::new(std::sync::Mutex::new(super::agents::HighDensityContextStore::new(10)));
                let res = agent.execute(intent, workspace, &bb)?;
                pair.push_str(&format!("REFLEX_GUARD ({}): {}\n", agent.name(), res));
            }
        }

        let action = crate::gemi::pulse::AeonPulse::reason(intent, workspace).unwrap_or_else(|_| "ACTION: status".into());
        pair.push_str(&format!("FINAL_ACTION: {}\n", action));

        Ok(pair)
    }

    pub fn list_reflex_weights(workspace: &Path) -> Vec<String> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
        let models_dir = PathBuf::from(home).join(".aeon").join("models");

        let mut weights = Vec::new();
        if let Ok(entries) = std::fs::read_dir(models_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".safetensors") || name.ends_with(".gguf") {
                    weights.push(name);
                }
            }
        }

        let local_weights = workspace.join("target/release/aeon-alpha.safetensors");
        if local_weights.exists() {
            weights.push("target/release/aeon-alpha.safetensors".into());
        }

        weights
    }

    pub fn verify_alpha_substrate() -> EaiResult<String> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
        let models_dir = PathBuf::from(home).join(".aeon").join("models");
        let weights_file = models_dir.join("aeon-alpha.safetensors");

        if weights_file.exists() {
            let meta = std::fs::metadata(&weights_file)?;
            Ok(format!("AEON-Alpha Substrate Verified: {} ({} bytes)", weights_file.display(), meta.len()))
        } else {
            Err(crate::error::EaiError::Inference("AEON-Alpha weights missing. Run 'aeon install'.".into()))
        }
    }

    #[allow(dead_code)]
    pub fn distill_reflex_to_binary(intent: &str, workspace: &Path) -> EaiResult<PathBuf> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
        let models_dir = PathBuf::from(home).join(".aeon").join("models");
        let weights_file = models_dir.join("aeon-alpha.safetensors");

        if !weights_file.exists() {
             return Err(crate::error::EaiError::Inference("AEON-Alpha substrate missing.".into()));
        }

        // Tier 0 Distillation Logic (Mock for now, will call candle-nn in next evolution)
        let distilled_path = workspace.join(format!(".aeon/reflexes/{}.bin", intent.replace(' ', "_")));
        let _ = std::fs::create_dir_all(distilled_path.parent().unwrap());
        std::fs::write(&distilled_path, b"DISTILLED_AEON_REFLEX_V1")?;

        Ok(distilled_path)
    }

    /// Stages a reasoning pair for autonomous distillation into local reflexes
    pub fn stage_distillation_pair(intent: &str, action: &str, workspace: &Path, metadata: Option<serde_json::Value>) -> EaiResult<()> {
        let aeon_dir = workspace.join(".aeon");
        if !aeon_dir.exists() {
            let _ = std::fs::create_dir_all(&aeon_dir);
        }
        let distillation_file = aeon_dir.join("distillation_staged.jsonl");
        let entry = serde_json::json!({
            "intent": intent,
            "action": action,
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
            "performance_metadata": metadata,
        });

        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(distillation_file) {
            use std::io::Write;
            let _ = writeln!(f, "{}", entry);
        }
        Ok(())
    }
}
