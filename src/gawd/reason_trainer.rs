// Reasoning Trainer: Autonomous Tier 2 Substrate Distillation
// Implements the "Substrate Ingestion Motion" (Aspiration 12)

use std::path::Path;
use crate::error::EaiResult;
use crate::gemi::reasoning::AeonReasoningModel;

pub struct ReasoningTrainer;

impl ReasoningTrainer {
    const DISTILLATION_THRESHOLD: usize = 25;

    pub fn audit_reasoning_substrate(_workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let experience_file = global_dir.join("reasoning_experience.jsonl");

        if experience_file.exists() {
            let content = std::fs::read_to_string(&experience_file).unwrap_or_default();
            let count = content.lines().count();

            if count >= Self::DISTILLATION_THRESHOLD {
                eprintln!("[Reasoning Trainer] Experience threshold reached ({} samples). Initializing Substrate Ingestion Motion...", count);
                match AeonReasoningModel::train_from_experience(&global_dir) {
                    Ok(report) => {
                        // Archive experience to avoid redundant distillation
                        let archive_path = global_dir.join(format!("reasoning_archive_{}.jsonl", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
                        let _ = std::fs::rename(&experience_file, archive_path);
                        return Ok(report);
                    },
                    Err(e) => return Ok(format!("Ingestion Failure: {}", e)),
                }
            }
        }

        Ok("Tier 2 Reasoning Substrate Optimal.".into())
    }

    pub fn force_distillation(_workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        AeonReasoningModel::train_from_experience(&global_dir).map_err(|e| crate::error::EaiError::Inference(e.to_string()))
    }
}
