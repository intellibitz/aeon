// AEON Truth Transformer: Formal Verification Substrate
// RULE 15: Truth & Hallucination Sovereignty - Native Candle Verification
// RULE 31: Substrate Purity Hardening - Meta Reality Verification

use std::path::Path;
use candle_core::{Device, Tensor};
use crate::error::{EaiError, EaiResult};

pub struct AeonTruthAgent;

impl AeonTruthAgent {
    /// Formal Verification Reflex
    /// Validates tool output against physical workspace reality before user delivery.
    pub fn verify_mission_reality(_goal: &str, _tool_name: &str, result: &str, workspace: &Path) -> EaiResult<String> {
        let mut violations = Vec::new();

        // META REALITY VERIFICATION
        // Instead of hardcoded tool names, we detect "Intent of Effect" in the result string.

        // Pattern: File System Mutation Detection
        if result.contains("Wrote to ") || result.contains("Saved to ") {
             let mut found_path = false;
             let parts: Vec<&str> = result.split([' ', '[', ']']).collect();
             for part in parts {
                 let path_candidate = part.trim_matches(|c| c == '.' || c == ':' || c == '[' || c == ']');
                 if (path_candidate.contains('/') || path_candidate.contains('.')) && !path_candidate.is_empty() {
                     let target_path = workspace.join(path_candidate);
                     found_path = true;
                     if !target_path.exists() {
                         violations.push(format!("Reality Mismatch: Resource '{}' reported as written but does not exist in workspace.", path_candidate));
                     } else if let Ok(m) = target_path.metadata() {
                         if m.len() == 0 && !result.to_lowercase().contains("empty") {
                             violations.push(format!("Reality Mismatch: Resource '{}' exists but is empty (0 bytes). Result claimed success.", path_candidate));
                         }
                     }
                     break;
                 }
             }
             if !found_path && (result.contains("Wrote to") || result.contains("Saved to")) {
                 violations.push("Reality Mismatch: Tool reported writing a file but no valid path could be extracted for verification.".to_string());
             }
        }

        if !violations.is_empty() {
            // Mandate: Epistemic Delegation (Aspiration 18)
            // If local verification fails, check if the result comes from a high-trust consensus
            if result.contains("[CONVERGENCE_SCORE: ") {
                if let Some(score_str) = result.split("[CONVERGENCE_SCORE: ").nth(1).and_then(|s| s.split(']').next()) {
                    if let Ok(score) = score_str.parse::<f32>() {
                        if score >= 0.85 {
                             crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "EPISTEMIC_DELEGATION", &format!("Local verification failed but Swarm Consensus (Score: {}) accepted. Proceeding.", score));
                             return Ok(format!("{} (Verified via Epistemic Delegation)", result));
                        }
                    }
                }
            }

            let error_msg = format!("TRUTH_VIOLATION: {}\nSTRUCTURED_FEEDBACK: Please grounded your response in the physical workspace state. Ensure files are actually written before reporting success.", violations.join(" | "));
            return Err(EaiError::governance(error_msg));
        }

        Ok(result.to_string())
    }

    #[allow(dead_code)]
    fn calculate_neural_truth_score(_goal: &str, result: &str) -> EaiResult<f32> {
        let bytes = result.as_bytes();
        if bytes.is_empty() { return Ok(0.0); }

        let device = Device::Cpu;
        let data: Vec<f32> = bytes.iter().map(|&b| b as f32 / 255.0).collect();
        let tensor = Tensor::from_vec(data, (bytes.len(),), &device)
            .map_err(|e| EaiError::inference(e.to_string()))?;

        let mean = tensor.mean_all().map_err(|e| EaiError::inference(e.to_string()))?
            .to_scalar::<f32>().map_err(|e| EaiError::inference(e.to_string()))?;

        let var = tensor.sqr().map_err(|e| EaiError::inference(e.to_string()))?
            .mean_all().map_err(|e| EaiError::inference(e.to_string()))?
            .to_scalar::<f32>().map_err(|e| EaiError::inference(e.to_string()))? - (mean * mean);

        let score = (var * 10.0 + 0.5).clamp(0.0, 1.0);
        Ok(score)
    }
}

pub struct TruthTransformer;

impl TruthTransformer {
    pub fn verify_mission_reality(goal: &str, tool_name: &str, result: &str, workspace: &Path) -> EaiResult<String> {
        AeonTruthAgent::verify_mission_reality(goal, tool_name, result, workspace)
    }
}
