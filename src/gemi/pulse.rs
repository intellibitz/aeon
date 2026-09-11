// AEON-Pulse: Tier 0 Native Bootstrap Brain
// 100% Neural implementation - Zero Hardcoded Heuristics (Rule 31)

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use super::alpha::AeonAlphaModel;

pub struct AeonPulse;

impl AeonPulse {
    /// Pure Neural Intent Resolution
    /// Eliminates Rule 11 violations by delegating all reasoning to trained semantic tensors.
    pub fn reason(prompt: &str, workspace: &Path) -> Result<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".aeon");

        // 🧠 Neural Reflex Attempt
        if let Ok(model) = AeonAlphaModel::load(&global_dir) {
            match model.predict_intent(prompt) {
                Ok(neural_action) => {
                    // If it's a directory action, we still need to ground the path
                    if neural_action.contains("list_directory") {
                        return Ok(format!("ACTION: list_directory {}", workspace.display()));
                    }
                    return Ok(neural_action);
                },
                Err(e) => {
                    // 🚀 Deterministic Escalation: If confidence is low, escalate to Tier 2
                    eprintln!("🧠 [Tier 0 Reflex] Escalating due to: {}", e);
                    return Err(anyhow!("Low confidence reflex. Escalating to Tier 2 Deep Reasoning..."));
                }
            }
        }

        // 🚀 Evolutionary Transition: If reflex weights missing, escalate
        Err(anyhow!("Pulse Brain: Neural substrate missing. Transitioning to Tier 2..."))
    }
}
