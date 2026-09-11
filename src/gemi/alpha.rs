// 🌌 AEON-Alpha: Native Neural Intelligence Substrate
// 100% Rust implementation using Candle for Tier 0 Reflex Distillation

use anyhow::{Result, anyhow};
use candle_core::{Tensor, DType};
use candle_nn::{Linear, Module, VarBuilder, VarMap, Optimizer, AdamW, ParamsAdamW};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistillationStaged {
    pub intent: String,
    pub action: String,
    pub timestamp: u64,
}

/// AEON-Alpha Intent Classifier (Neural Reflex)
pub struct AeonAlphaModel {
    fc1: Linear,
    fc2: Linear,
}

impl AeonAlphaModel {
    pub const DIM: usize = 128;

    pub fn load(global_dir: &Path) -> Result<Self> {
        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        if !weights_path.exists() {
            return Err(anyhow!("AEON-Alpha weights not found"));
        }

        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? };

        let fc1 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex"))?;
        let fc2 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex_out"))
            .unwrap_or(candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex"))?);

        Ok(Self { fc1, fc2 })
    }

    /// 🧪 Dynamic Intent Surface Discovery (Rule 31 Hardening)
    pub fn list_dynamic_intents() -> Vec<String> {
        let mut intents = vec![
            "status".into(), "version".into(), "self_heal_build".into(),
            "run_test_harness".into(), "write_file".into(), "read_file".into(),
            "list_directory".into(), "scout".into(), "reason".into()
        ];

        // 🚀 Add Registered Agents
        let registry = crate::gawd::agents::AgentMetaRegistry::global();
        for agent in registry.list_agents() {
            if !intents.contains(&agent.name) {
                intents.push(agent.name);
            }
        }

        // 🚀 Add Installed Tools
        let tools = crate::gmcp::tools::ToolRegistry::list_tools();
        for tool in tools {
            if !intents.contains(&tool.name) {
                intents.push(tool.name);
            }
        }

        intents.sort();
        intents.truncate(Self::DIM); // Cap at output dimension
        intents
    }

    pub fn train_on_staged_data(global_dir: &Path) -> Result<String> {
        let staged_file = global_dir.join("distillation_staged.jsonl");
        if !staged_file.exists() {
            return Err(anyhow!("No staged distillation data found."));
        }

        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let fc1 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex"))?;
        let fc2 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex_out"))?;

        let mut opt = AdamW::new(varmap.all_vars(), ParamsAdamW::default())?;

        // Load Data and Map to Dynamic Surface
        let content = std::fs::read_to_string(&staged_file)?;
        let mut samples = Vec::new();
        let mut labels = Vec::new();

        let dynamic_intents = Self::list_dynamic_intents();

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<DistillationStaged>(line) {
                let vec = Self::semantic_centroid_projection(&entry.intent)?;
                samples.push(Tensor::from_vec(vec, (1, Self::DIM), &device)?);

                let action_clean = entry.action.to_lowercase();
                let label_idx = dynamic_intents.iter()
                    .position(|i| action_clean.contains(&i.to_lowercase()))
                    .unwrap_or(dynamic_intents.len() - 1) as u32;
                labels.push(label_idx);
            }
        }

        // 🚀 Neural Seeding (Synthetic Priming): Ensure new tools have at least one sample
        for (idx, intent) in dynamic_intents.iter().enumerate() {
            let vec = Self::semantic_centroid_projection(intent)?;
            samples.push(Tensor::from_vec(vec, (1, Self::DIM), &device)?);
            labels.push(idx as u32);
        }

        if samples.is_empty() { return Err(anyhow!("Empty distillation dataset.")); }

        let x = Tensor::cat(&samples, 0)?;
        let y = Tensor::from_vec(labels, samples.len(), &device)?;

        // Training Loop
        for epoch in 1..=100 {
            let logits = fc1.forward(&x)?.relu()?;
            let logits = fc2.forward(&logits)?;
            let log_sm = candle_nn::ops::log_softmax(&logits, 1)?;
            let loss = candle_nn::loss::nll(&log_sm, &y)?;
            opt.backward_step(&loss)?;

            if epoch % 20 == 0 {
                eprintln!("Epoch {}: Loss: {:?}", epoch, loss);
            }
        }

        // 🚀 Atomic Model Save (Rule 13 Hardening)
        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        let tmp_path = weights_path.with_extension("tmp");
        varmap.save(&tmp_path)?;
        std::fs::rename(tmp_path, weights_path)?;

        Ok(format!("Autonomous Distillation Complete. Retrained on {} samples with Dynamic Intent Surface.", samples.len()))
    }

    pub fn get_model_fingerprint(global_dir: &Path) -> String {
        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        if let Ok(meta) = std::fs::metadata(weights_path) {
            return format!("{:?}", meta.modified().unwrap());
        }
        "missing".to_string()
    }

    pub fn predict_intent(&self, prompt: &str) -> Result<String> {
        let (action, confidence) = self.predict_intent_with_confidence(prompt)?;
        if confidence > 0.5 {
            return Ok(action);
        }
        Err(anyhow!("Low confidence ({:.2}) in neural reflex.", confidence))
    }

    pub fn predict_intent_with_confidence(&self, prompt: &str) -> Result<(String, f32)> {
        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let input_vec = Self::semantic_centroid_projection(prompt)?;
        let input_tensor = Tensor::from_vec(input_vec, (1, Self::DIM), &device)?;

        let output = self.fc1.forward(&input_tensor)?;
        let output = output.relu()?;
        let output = self.fc2.forward(&output)?;

        let probs = candle_nn::ops::softmax(&output, 1)?;
        let results = probs.to_vec2::<f32>()?[0].clone();

        let mut max_idx = 0;
        let mut max_val = 0.0;
        for (i, &val) in results.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        let dynamic_intents = Self::list_dynamic_intents();
        if let Some(intent) = dynamic_intents.get(max_idx) {
            return Ok((format!("ACTION: {}", intent), max_val));
        }

        Err(anyhow!("Low confidence in neural reflex."))
    }

    /// 🧪 Deterministic Semantic Embedding Substrate
    fn semantic_centroid_projection(prompt: &str) -> Result<Vec<f32>> {
        let mut vec = vec![0.0f32; Self::DIM];
        let prompt_lower = prompt.to_lowercase();
        let words: Vec<&str> = prompt_lower.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).collect();

        if words.is_empty() { return Ok(vec); }

        for (i, word) in words.iter().enumerate() {
            let word_vec = Self::get_semantic_anchor(word);
            for (j, &val) in word_vec.iter().enumerate() {
                let weight = 1.0 / (i as f32 + 1.0);
                vec[j] += val * weight;
            }
        }

        let sum_sq = vec.iter().map(|x| x * x).sum::<f32>();
        if sum_sq > 0.0 {
            let norm = sum_sq.sqrt();
            for x in vec.iter_mut() {
                *x /= norm;
            }
        }

        Ok(vec)
    }

    fn get_semantic_anchor(word: &str) -> Vec<f32> {
        let mut anchor = vec![0.0f32; Self::DIM];

        // 🚀 Adaptive Semantic Anchors: Query registry for domain specialist keywords
        let registry = crate::gawd::agents::AgentMetaRegistry::global();
        let agents = registry.list_agents();
        for agent in agents {
            if agent.semantic_anchors.iter().any(|a| a == word) {
                 // Map to agent-specific segment (starting from DIM 80+)
                 let offset = 80 + (agent.name.len() % 40);
                 anchor[offset] = 1.0;
                 return anchor;
            }
        }

        let category = match word {
            "status" | "health" | "state" | "check" | "hardware" | "system" | "report" => 0,
            "version" | "ver" | "build" | "engine" | "revision" => 1,
            "write" | "save" | "create" | "file" | "update" | "put" => 2,
            "read" | "get" | "fetch" | "cat" | "show" | "content" => 3,
            "list" | "ls" | "dir" | "directory" | "folder" | "files" => 4,
            "scout" | "search" | "find" | "look" | "discover" | "mcp" => 5,
            "reason" | "think" | "solve" | "complex" | "calculate" => 6,
            "fix" | "heal" | "repair" | "audit" | "compliance" => 7,
            _ => 99,
        };

        if category < 10 {
            let start = category * 10;
            for j in start..start+10 { anchor[j] = 1.0; }
        } else {
            let mut h = 0u32;
            for b in word.as_bytes() { h = h.wrapping_add(*b as u32); }
            anchor[(h as usize) % Self::DIM] = 0.5;
        }
        anchor
    }
}
