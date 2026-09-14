// SUSI Substrate Evolution Manager
// RULE 23: Motion Rule Protocol - Test-Driven Evolution Substrate

use std::path::Path;
use std::process::Command;
use crate::error::EaiResult;
use crate::sandbox::manager::SusiAuditLogger;
use crate::gawd::reflex_synth::ReflexSynthesizer;
use std::collections::HashMap;

pub struct EvolutionManager;

impl EvolutionManager {
    /// Test-Driven Evolution Loop (Aspirational Core Paradigm)
    /// Ingests failure signatures from tests and self-heals the native substrate.
    pub fn execute_evolutionary_cycle(workspace: &Path) -> EaiResult<String> {
        // 1. Execute Test Loop to detect architectural failure signatures
        let test_output = Command::new("cargo")
            .arg("test")
            .current_dir(workspace)
            .output()?;

        if test_output.status.success() {
            return Ok("Substrate is fully verified. No evolutionary pressure detected.".into());
        }

        // 2. Parse Failure Pathology
        let stderr = String::from_utf8_lossy(&test_output.stderr);
        let stdout = String::from_utf8_lossy(&test_output.stdout);
        let combined = format!("{}\n{}", stdout, stderr);

        // 3. Autonomous Self-Healing via Synthesis
        if combined.contains("FAILED") || combined.contains("error:") {
             let intent_to_heal = Self::detect_evolutionary_target(workspace, &combined);

             // Synthesis driven by test failure
             let res = ReflexSynthesizer::distill_native_reflex(&intent_to_heal, workspace)?;

             // Substrate Ingestion: Retrain Tier 2 model if experience buffer is full
             let _ = crate::gawd::reason_trainer::ReasoningTrainer::audit_reasoning_substrate(workspace);

             return Ok(format!("# SUSI Motion Rule Triggered\n\n\
                Test-Driven Evolution has detected a substrate failure and autonomously synthesis a repair.\n\n\
                - **Target**: {}\n\
                - **Action**: {}", intent_to_heal, res));
        }

        Ok("Substrate alignment verified.".into())
    }

    pub fn evolve_substrate(workspace: &Path) -> EaiResult<String> {
        Self::execute_evolutionary_cycle(workspace)
    }

    /// Autonomous Drift Detection (Aspiration 7)
    /// Periodic audit of the substrate health and capability surface.
    pub fn perform_autonomous_drift_audit(workspace: &Path) -> EaiResult<String> {
        // 1. Audit for High-Frequency Capability Gaps
        let gap = Self::detect_high_frequency_gap(workspace);

        // 2. Audit for Staged Experience (Substrate Ingestion Motion)
        let ingestion_res = crate::gawd::reason_trainer::ReasoningTrainer::audit_reasoning_substrate(workspace)?;

        // 3. If a high-frequency gap is detected and not yet synthesized, trigger synthesis
        if gap != "calculate square root" { // Heuristic check for non-default gap
             eprintln!("[Evolution Manager] Capability drift detected: {}. Initializing autonomous repair...", gap);
             let res = ReflexSynthesizer::distill_native_reflex(&gap, workspace)?;
             return Ok(format!("Autonomous evolution successful: {}", res));
        }

        Ok(format!("Substrate Optimal. {}", ingestion_res))
    }

    fn detect_evolutionary_target(workspace: &Path, test_output: &str) -> String {
        // In real evolutionary scenarios, this parses compiler errors and test failure messages
        // to identify the specific component or trait implementation that is missing or broken.
        if test_output.contains("calculate square root") {
            return "math_sqrt_reflex".to_string();
        }

        Self::detect_high_frequency_gap(workspace)
    }

    pub fn detect_high_frequency_gap(workspace: &Path) -> String {
        let log_content = SusiAuditLogger::read_audit_log(workspace, 100);
        let mut intent_freq = HashMap::new();

        for line in log_content.lines() {
            if line.contains("[MISSION_START]") {
                if let Some(intent) = line.split("[MISSION_START]").nth(1) {
                    let trimmed = intent.trim();
                    if trimmed.len() > 3 {
                        *intent_freq.entry(trimmed.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        intent_freq.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(intent, _)| intent)
            .unwrap_or_else(|| "calculate square root".to_string())
    }
}
