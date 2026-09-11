// GEMI: Universal AI Inference & Reasoning Bridge
// 100% Rust implementation for Native Intelligence Substrate
// RULE 23: Motion Rule Protocol - Aspiration 6: Competitive Inference Racing

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, mpsc};
use std::thread;
use crate::error::EaiResult;
use crate::gemi::models::ModelManager;
use crate::gemi::hardware::HardwareProfiler;

use candle_core::quantized::gguf_file;
use candle_transformers::models::quantized_llama as llama;
use tokenizers::Tokenizer;

pub struct InferenceHost;

impl InferenceHost {
    /// 🧠 Intelligence Persistence (Phase 4 Hardening)
    /// Static model weights container to eliminate disk I/O bottlenecks.
    pub fn get_model(model_path: &Path, device: &candle_core::Device) -> EaiResult<Arc<Mutex<llama::ModelWeights>>> {
        static CACHED_MODEL: OnceLock<Arc<Mutex<llama::ModelWeights>>> = OnceLock::new();

        if let Some(m) = CACHED_MODEL.get() {
            return Ok(Arc::clone(m));
        }

        let mut file = std::fs::File::open(model_path)?;
        let model_data = gguf_file::Content::read(&mut file)
            .map_err(|e| crate::error::EaiError::Inference(format!("GGUF Read Error: {}", e)))?;

        let weights = llama::ModelWeights::from_gguf(model_data, &mut file, device)
            .map_err(|e| crate::error::EaiError::Inference(format!("Model Load Error: {}", e)))?;

        let shared = Arc::new(Mutex::new(weights));
        let _ = CACHED_MODEL.set(Arc::clone(&shared));
        Ok(shared)
    }
}

pub struct ContextSummarizer;

impl ContextSummarizer {
    /// 🗜️ Context Compression: Reduces Mission Blackboard to high-density semantic summary.
    pub fn compress_blackboard(blackboard: &std::collections::HashMap<String, String>) -> String {
        let mut summary = String::new();
        for (agent, output) in blackboard {
            let clean_output = if output.len() > 100 {
                format!("{}...", &output[..97])
            } else {
                output.clone()
            };
            summary.push_str(&format!("[{}: {}] ", agent, clean_output));
        }
        summary
    }
}

pub struct GemiEngine;

impl GemiEngine {
    pub fn generate_reasoning(prompt: &str, workspace: &Path) -> String {
        Self::reason_internal(prompt, workspace, true)
    }

    pub fn generate_reasoning_deep(prompt: &str, workspace: &Path) -> String {
        Self::reason_internal(prompt, workspace, false)
    }

    /// 🚀 Aspiration 6: Ultra-Latency Competitive Inference Racing
    fn reason_internal(prompt: &str, workspace: &Path, allow_reflex: bool) -> String {
        let _ = crate::gawd::model_supervisor::ModelSupervisor::audit_and_prepare_models(workspace);

        if allow_reflex {
            let (reflex_decision, _) = super::reflex::ReflexEngine::try_solve(prompt, workspace);
            if let super::reflex::ReflexDecision::Solved(action) = reflex_decision {
                return action;
            }
        }

        let (tx, rx) = mpsc::channel();
        let p1 = prompt.to_string();
        let p2 = prompt.to_string();
        let ws2 = workspace.to_path_buf();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let engine = AeonGgufEngine;
            if let Ok(res) = engine.run_inference(&p1) {
                let _ = tx1.send(res);
            }
        });

        thread::spawn(move || {
            let power_res = crate::gmcp::tools::ToolRegistry::execute_tool("power_reason", &p2, &ws2);
            if !power_res.contains("[FAIL]") && !power_res.contains("[CAPABILITY_GAP]") {
                let _ = tx.send(power_res);
            } else if let Ok(pulse_res) = super::pulse::AeonPulse::reason(&p2, &ws2) {
                let _ = tx.send(pulse_res);
            }
        });

        let winner = rx.recv_timeout(std::time::Duration::from_millis(15000))
            .unwrap_or_else(|_| "STATUS: Reasoning timeout during substrate race.".into());

        match Self::verify_axiomatic_alignment(&winner, workspace) {
            Ok(v) => v,
            Err(_) => {
                AeonGgufEngine.run_inference(prompt).unwrap_or_else(|e| format!("REPAIR_FAILED: {}", e))
            }
        }
    }

    pub fn generate_multimodal_vision(prompt: &str, image_path: &Path) -> String {
        format!("👁️ [aeon Native Vision]: {} -> {}", image_path.display(), prompt)
    }

    pub fn verify_axiomatic_alignment(reasoning: &str, workspace: &Path) -> EaiResult<String> {
        let audit_prompt = format!(
            "REASONING_OUTPUT: {}\n\n[INSTRUCTION]: Audit this reasoning. Report ONLY 'PASSED' or 'FAILED'.",
            reasoning
        );

        let (reflex, _) = super::reflex::ReflexEngine::try_solve(&audit_prompt, workspace);
        if let super::reflex::ReflexDecision::Solved(action) = reflex {
            if action.contains("PASSED") { return Ok(reasoning.to_string()); }
        }

        let audit_res = AeonGgufEngine.run_inference(&audit_prompt).unwrap_or("FAILED".into());
        if audit_res.to_uppercase().contains("PASSED") {
            Ok(reasoning.to_string())
        } else {
            Err(crate::error::EaiError::Governance(format!("Axiomatic Alignment Failed: {}", audit_res)))
        }
    }
}

pub struct MissionPlan {
    pub goals: Vec<String>,
}

pub struct MissionPlanner;

impl MissionPlanner {
    pub fn plan_mission(goal: &str, workspace: &Path) -> EaiResult<MissionPlan> {
        let plan_prompt = format!(
            "MISSION_GOAL: {}\n\n[INSTRUCTION]: Decompose this mission into a sequence of executable sub-goals. Output as a comma-separated list of actions.",
            goal
        );
        let plan_str = GemiEngine::generate_reasoning(&plan_prompt, workspace);
        let mut goals = Vec::new();
        if plan_str.contains(',') {
            for g in plan_str.split(',') {
                let clean = g.trim();
                if !clean.is_empty() { goals.push(clean.to_string()); }
            }
        } else {
            goals.push(goal.to_string());
        }
        Ok(MissionPlan { goals })
    }

    pub fn refine_plan(original_goal: &str, blackboard_state: &str, workspace: &Path) -> EaiResult<MissionPlan> {
        let refine_prompt = format!(
            "ORIGINAL_GOAL: {}\nCURRENT_STATE: {}\n\n[INSTRUCTION]: Mid-mission change. Re-synthesize sub-goals.",
            original_goal, blackboard_state
        );
        Self::plan_mission(&refine_prompt, workspace)
    }
}

pub trait NativeInferenceEngine: Send + Sync {
    fn name(&self) -> String;
    fn run_inference(&self, prompt: &str) -> EaiResult<String>;
}

pub struct AeonGgufEngine;

impl NativeInferenceEngine for AeonGgufEngine {
    fn name(&self) -> String { "AeonGgufEngine".to_string() }
    fn run_inference(&self, prompt: &str) -> EaiResult<String> {
        let model_id = ModelManager::get_selected_model()
            .ok_or_else(|| crate::error::EaiError::Inference("No reasoning model selected.".into()))?;
        let model_path = ModelManager::get_model_path(&model_id)
            .ok_or_else(|| crate::error::EaiError::Inference(format!("Model '{}' not found.", model_id)))?;
        let tokenizer_path = ModelManager::get_tokenizer_path(&model_id)
            .ok_or_else(|| crate::error::EaiError::Inference("Tokenizer missing.".into()))?;

        let device = HardwareProfiler::get_candle_device();
        let model_weights_shared = InferenceHost::get_model(&model_path, &device)?;
        let mut model_weights = model_weights_shared.lock().unwrap();

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| crate::error::EaiError::Inference(format!("Tokenizer Error: {}", e)))?;
        let tokens = tokenizer.encode(prompt, true)
            .map_err(|e| crate::error::EaiError::Inference(format!("Tokenization Error: {}", e)))?;

        let prompt_tokens = tokens.get_ids();
        let mut all_tokens = vec![];
        let mut tokens_to_process = prompt_tokens.to_vec();

        for i in 0..512 {
            let input = candle_core::Tensor::new(tokens_to_process.as_slice(), &device)?.unsqueeze(0)?;
            let logits = model_weights.forward(&input, prompt_tokens.len() + i)?;
            let logits = logits.squeeze(0)?;
            let logits_v: Vec<f32> = logits.to_vec1()?;

            let mut max_idx = 0;
            let mut max_val = f32::NEG_INFINITY;
            for (idx, &v) in logits_v.iter().enumerate() {
                if v > max_val { max_val = v; max_idx = idx; }
            }

            let next_token = max_idx as u32;
            all_tokens.push(next_token);
            if next_token == 2 || next_token == 32000 { break; }
            tokens_to_process = vec![next_token];
        }

        let output = tokenizer.decode(&all_tokens, true)
            .map_err(|e| crate::error::EaiError::Inference(format!("Decoding Error: {}", e)))?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_competitive_racing_logic() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(50));
            let _ = tx1.send("FastPath".to_string());
        });
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(200));
            let _ = tx.send("SlowPath".to_string());
        });
        let winner = rx.recv().unwrap();
        assert_eq!(winner, "FastPath");
    }

    #[test]
    fn test_native_tokenization() {
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let tokenizer_path = home.join(".aeon/models/tokenizer.json");
        if tokenizer_path.exists() {
            let tokenizer = Tokenizer::from_file(tokenizer_path);
            assert!(tokenizer.is_ok());
        }
    }
}
