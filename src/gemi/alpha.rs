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
                let vec = Self::static_vectorize(&entry.intent)?;
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
        for epoch in 1..=50 {
            let logits = fc1.forward(&x)?.relu()?;
            let logits = fc2.forward(&logits)?;
            let log_sm = candle_nn::ops::log_softmax(&logits, 1)?;
            let loss = candle_nn::loss::nll(&log_sm, &y)?;
            opt.backward_step(&loss)?;

            if epoch % 10 == 0 {
                eprintln!("Epoch {}: Loss: {:?}", epoch, loss);
            }
        }

        let weights_path = global_dir.join("models/aeon-alpha.safetensors");
        varmap.save(weights_path)?;

        Ok(format!("Autonomous Distillation Complete. Retrained on {} samples.", samples.len()))
    }

    pub fn predict_intent(&self, prompt: &str) -> Result<String> {
        let (action, confidence) = self.predict_intent_with_confidence(prompt)?;
        if confidence > 0.4 {
            return Ok(action);
        }
        Err(anyhow!("Low confidence in neural reflex."))
    }

    pub fn predict_intent_with_confidence(&self, prompt: &str) -> Result<(String, f32)> {
        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let input_vec = Self::static_vectorize(prompt)?;
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

    fn static_vectorize(prompt: &str) -> Result<Vec<f32>> {
        let mut vec = vec![0.0f32; Self::DIM];
        let prompt_lower = prompt.to_lowercase();
        let words: Vec<&str> = prompt_lower.split_whitespace().collect();

        for (i, word) in words.iter().enumerate().take(Self::DIM) {
            let mut sum = 0u32;
            for b in word.as_bytes() {
                sum = sum.wrapping_add(*b as u32);
            }
            vec[i] = (sum % 100) as f32 / 100.0;
        }
        Ok(vec)
    }
}
