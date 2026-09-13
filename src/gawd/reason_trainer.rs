// Reasoning Trainer: Autonomous Tier 2 Substrate Distillation
// Implements the "Substrate Ingestion Motion" (Aspiration 12)

use std::path::Path;
use crate::error::EaiResult;
use crate::gemi::reasoning::AeonReasoningModel;
use crate::gawd::genome_distiller::GenomeDistiller;

pub struct ReasoningTrainer;

impl ReasoningTrainer {

    pub fn audit_reasoning_substrate(workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let experience_file = global_dir.join("reasoning_experience.jsonl");

        // Step 1: Ensure Genome is distilled into experience if buffer is low
        let existing_count = if experience_file.exists() {
            std::fs::read_to_string(&experience_file).unwrap_or_default().lines().count()
        } else { 0 };

        if existing_count == 0 {
            if std::env::var("AEON_VERBOSE").is_ok() {
                eprintln!("[Reasoning Trainer] Experience buffer empty. Distilling Genome into synthetic wisdom...");
            }
            let distilled = GenomeDistiller::distill_genome_to_experience(workspace)?;
            if std::env::var("AEON_VERBOSE").is_ok() {
                eprintln!("[Reasoning Trainer] Added {} genome-anchored samples.", distilled);
            }
        }

        Ok("Tier 2 Reasoning Substrate Optimal.".into())
    }

    pub fn force_distillation(_workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        AeonReasoningModel::train_from_experience(&global_dir).map_err(|e| crate::error::EaiError::inference(e.to_string()))
    }
}
