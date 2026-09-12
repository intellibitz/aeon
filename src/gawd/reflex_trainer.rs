// Reflex Trainer: Autonomous Neural Substrate Evolution
// Monitors learning staged buffer and triggers native distillation.

use std::path::Path;
use crate::error::EaiResult;
use crate::gemi::alpha::AeonAlphaModel;

pub struct ReflexTrainer;

impl ReflexTrainer {
    const TRAINING_THRESHOLD: usize = 50;

    /// Checks if the substrate needs a retraining cycle based on learned wisdom volume.
    pub fn audit_distillation_state(_workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let staged_file = global_dir.join("distillation_staged.jsonl");

        if staged_file.exists() {
            let content = std::fs::read_to_string(&staged_file).unwrap_or_default();
            let count = content.lines().count();

            if count >= Self::TRAINING_THRESHOLD {
                eprintln!("[Reflex Trainer] Wisdom buffer saturated ({} samples). Triggering native distillation...", count);
                match AeonAlphaModel::train_on_staged_data(&global_dir) {
                    Ok(report) => {
                        // Clear the buffer after successful evolution
                        let _ = std::fs::remove_file(&staged_file);
                        return Ok(report);
                    },
                    Err(e) => return Ok(format!("Distillation Failure: {}", e)),
                }
            }
        }

        Ok("Reflex substrate optimal.".into())
    }

    pub fn force_train(_workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        AeonAlphaModel::train_on_staged_data(&global_dir).map_err(|e| crate::error::EaiError::inference(e.to_string()))
    }
}
