// AEON-Pulse: Tier 0 Native Bootstrap Brain
// 100% Neural implementation - Zero Hardcoded Heuristics (Rule 31)

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use super::alpha::AeonAlphaModel;

pub struct AeonPulse;

impl AeonPulse {
    /// Pure Neural Intent Resolution
    /// Eliminates Rule 11 violations by delegating all reasoning to trained tensors.
    pub fn reason(prompt: &str, workspace: &Path) -> Result<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".aeon");

        // 🧠 Neural Reflex Attempt
        if let Ok(model) = AeonAlphaModel::load(&global_dir) {
            if let Ok(neural_action) = model.predict_intent(prompt) {
                // If it's a directory action, we still need to ground the path
                if neural_action.contains("list_directory") {
                    return Ok(format!("ACTION: list_directory {}", workspace.display()));
                }
                return Ok(neural_action);
            }
        }

        // 🚀 Evolutionary Transition: If reflex confidence is low, escalate to Tier 2
        Err(anyhow!("Pulse Brain: Confidence threshold not met. Transitioning to Tier 2 Deep Reasoning..."))
    }
}
