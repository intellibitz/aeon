// Reasoning Trainer: Autonomous Tier 2 Substrate Distillation
// Implements the "Substrate Ingestion Motion" (Aspiration 12)

use std::path::Path;
use crate::error::EaiResult;
use crate::gemi::reasoning::AeonReasoningModel;
use crate::gawd::genome_distiller::GenomeDistiller;

pub struct ReasoningTrainer;

impl ReasoningTrainer {
    const DISTILLATION_THRESHOLD: usize = 25;

    pub fn audit_reasoning_substrate(workspace: &Path) -> EaiResult<String> {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let experience_file = global_dir.join("reasoning_experience.jsonl");

        // Step 1: Ensure Genome is distilled into experience if buffer is low
        let existing_count = if experience_file.exists() {
            std::fs::read_to_string(&experience_file).unwrap_or_default().lines().count()
        } else { 0 };

        if existing_count < Self::DISTILLATION_THRESHOLD {
            eprintln!("[Reasoning Trainer] Experience buffer low. Distilling Genome into synthetic wisdom...");
            let distilled = GenomeDistiller::distill_genome_to_experience(workspace)?;
            eprintln!("[Reasoning Trainer] Added {} genome-anchored samples.", distilled);
        }

        if experience_file.exists() {
            let content = std::fs::read_to_string(&experience_file).unwrap_or_default();
            let count = content.lines().count();

            if count >= Self::DISTILLATION_THRESHOLD {
                eprintln!("[Reasoning Trainer] Experience threshold reached ({} samples). Initializing Substrate Ingestion Motion for 'aeon-reason' Tier 2 model...", count);
                let g_dir = global_dir.clone();
                let exp_f = experience_file.clone();
                std::thread::spawn(move || {
                    if let Ok(report) = AeonReasoningModel::train_from_experience(&g_dir) {
                        let archive_path = g_dir.join(format!("reasoning_archive_{}.jsonl", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
                        let _ = std::fs::rename(&exp_f, archive_path);
                        eprintln!("[Reasoning Trainer] Background Distillation Complete: {}", report);
                    }
                });
                return Ok(format!("Substrate Ingestion Motion initialized in background ({} samples).", count));
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
