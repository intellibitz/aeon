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
    intents: Vec<String>,
}

impl AeonAlphaModel {
    pub const DIM: usize = 128;

    pub fn load(global_dir: &Path) -> Result<Self> {
        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        if !weights_path.exists() {
            return Err(anyhow!("AEON-Alpha weights not found at {}", weights_path.display()));
        }

        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? };

        let fc1 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex"))?;
        let fc2 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex_out"))
            .unwrap_or(candle_nn::linear(Self::DIM, Self::DIM, vb.pp("reflex"))?);

        // Standard Intent Mapping for v0.1
        let intents = vec![
            "status".into(), "version".into(), "self_heal_build".into(),
            "run_test_harness".into(), "write_file".into(), "read_file".into(),
            "list_directory".into(), "scout".into(), "reason".into()
        ];

        Ok(Self { fc1, fc2, intents })
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

        // Load and Parse Data
        let content = std::fs::read_to_string(&staged_file)?;
        let mut samples = Vec::new();
        let mut labels = Vec::new();

        let intents = vec![
            "status", "version", "self_heal_build", "run_test_harness",
            "write_file", "read_file", "list_directory", "scout", "reason"
        ];

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<DistillationStaged>(line) {
                let vec = Self::semantic_centroid_projection(&entry.intent)?;
                samples.push(Tensor::from_vec(vec, (1, Self::DIM), &device)?);

                let action_clean = entry.action.to_lowercase();
                let label_idx = intents.iter().position(|&i| action_clean.contains(i)).unwrap_or(8) as u32;
                labels.push(label_idx);
            }
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

        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        varmap.save(weights_path)?;

        Ok(format!("Autonomous Distillation Complete. Retrained on {} samples with Semantic Projections.", samples.len()))
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

        if let Some(intent) = self.intents.get(max_idx) {
            return Ok((format!("ACTION: {}", intent), max_val));
        }

        Err(anyhow!("Low confidence in neural reflex."))
    }

    /// 🧪 Deterministic Semantic Embedding Substrate (Phase 4 Evolution)
    /// Replaces brittle hash-based vectorization with AEON-specific semantic centroids.
    fn semantic_centroid_projection(prompt: &str) -> Result<Vec<f32>> {
        let mut vec = vec![0.0f32; Self::DIM];
        let prompt_lower = prompt.to_lowercase();
        let words: Vec<&str> = prompt_lower.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).collect();

        if words.is_empty() { return Ok(vec); }

        for (i, word) in words.iter().enumerate() {
            let word_vec = Self::get_semantic_anchor(word);
            for (j, &val) in word_vec.iter().enumerate() {
                // Centroid pooling: Average of anchors weighted by position
                let weight = 1.0 / (i as f32 + 1.0);
                vec[j] += val * weight;
            }
        }

        // L2 Normalization to stabilize the projection
        let sum_sq = vec.iter().map(|x| x * x).sum::<f32>();
        if sum_sq > 0.0 {
            let norm = sum_sq.sqrt();
            for x in vec.iter_mut() {
                *x /= norm;
            }
        }

        Ok(vec)
    }

    /// ⚓ Semantic Anchor Dictionary
    /// Tiny pre-baked dictionary for core AEON primitives.
    fn get_semantic_anchor(word: &str) -> Vec<f32> {
        let mut anchor = vec![0.0f32; Self::DIM];

        // 🧪 Zero-Config Semantic Mapping
        let category = match word {
            "status" | "health" | "state" | "check" | "hardware" | "system" | "report" => 0,
            "version" | "ver" | "build" | "engine" | "revision" => 1,
            "write" | "save" | "create" | "file" | "update" | "put" => 2,
            "read" | "get" | "fetch" | "cat" | "show" | "content" => 3,
            "list" | "ls" | "dir" | "directory" | "folder" | "files" => 4,
            "scout" | "search" | "find" | "look" | "discover" | "mcp" => 5,
            "reason" | "think" | "solve" | "complex" | "calculate" => 6,
            "fix" | "heal" | "repair" | "audit" | "compliance" => 7,
            _ => 99, // Unknown / Noise
        };

        if category < 10 {
            // Project into category-specific DIM segments
            let start = category * 10;
            for j in start..start+10 {
                anchor[j] = 1.0;
            }
        } else {
            // Deterministic Noise (Fall back to hash for unknown words)
            let mut h = 0u32;
            for b in word.as_bytes() { h = h.wrapping_add(*b as u32); }
            anchor[(h as usize) % Self::DIM] = 0.5;
        }

        anchor
    }
}
