// AEON-Reason: Native Neural Reasoning Substrate
// 100% Rust implementation using Candle for Tier 2 Logic Distillation

use anyhow::{Result, anyhow};
use candle_core::{Tensor, DType};
use candle_nn::{Linear, Module, VarBuilder, VarMap, Optimizer, AdamW, ParamsAdamW};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningSample {
    pub intent: String,
    pub blackboard_context: String,
    pub successful_outcome: String,
    pub timestamp: u64,
}

/// AEON-Reason Native Tier 2 Model (Distilled Logic)
/// 4-Layer Deep Logic Substrate for high-fidelity reasoning emulation.
pub struct AeonReasoningModel {
    l1: Linear,
    l2: Linear,
    l3: Linear,
    l4: Linear,
}

impl AeonReasoningModel {
    pub const DIM: usize = 256;

    pub fn load(global_dir: &Path) -> Result<Self> {
        let weights_path = global_dir.join("models/aeon-reason.safetensors");
        if !weights_path.exists() {
            return Err(anyhow!("AEON-Reason weights not found. Run 'aeon train_reason'."));
        }

        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? };

        let l1 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_1"))?;
        let l2 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_2"))?;
        let l3 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_3"))?;
        let l4 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_out"))?;

        Ok(Self { l1, l2, l3, l4 })
    }

    pub fn train_from_experience(global_dir: &Path) -> Result<String> {
        let experience_file = global_dir.join("reasoning_experience.jsonl");
        if !experience_file.exists() {
             return Err(anyhow!("No reasoning experience found to distill."));
        }

        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let l1 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_1"))?;
        let l2 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_2"))?;
        let l3 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_3"))?;
        let l4 = candle_nn::linear(Self::DIM, Self::DIM, vb.pp("logic_out"))?;

        let mut opt = AdamW::new(varmap.all_vars(), ParamsAdamW::default())?;

        let content = std::fs::read_to_string(&experience_file)?;
        let mut samples = Vec::new();
        let mut targets = Vec::new();

        for line in content.lines() {
            if let Ok(sample) = serde_json::from_str::<ReasoningSample>(line) {
                // Feature Engineering: Combine intent and context into semantic vector
                let feature_vec = Self::project_features(&sample.intent, &sample.blackboard_context)?;
                samples.push(Tensor::from_vec(feature_vec, (1, Self::DIM), &device)?);

                // Target: Semantic projection of the successful outcome
                let target_vec = crate::gemi::alpha::AeonAlphaModel::semantic_centroid_projection(&sample.successful_outcome)?;
                // Up-project target to DIM if needed, or use consistent DIM for both
                // For now, we reuse alpha's projection and pad/repeat to match DIM 256
                let mut padded_target = vec![0.0f32; Self::DIM];
                for (i, &v) in target_vec.iter().enumerate() { padded_target[i] = v; }
                targets.push(Tensor::from_vec(padded_target, (1, Self::DIM), &device)?);
            }
        }

        if samples.is_empty() { return Err(anyhow!("Insufficient reasoning data for distillation.")); }

        let x = Tensor::cat(&samples, 0)?;
        let y = Tensor::cat(&targets, 0)?;

        // Deep Distillation Loop
        for _epoch in 1..=200 {
            let out = l1.forward(&x)?.relu()?;
            let out = l2.forward(&out)?.relu()?;
            let out = l3.forward(&out)?.relu()?;
            let out = l4.forward(&out)?;

            let loss = candle_nn::loss::mse(&out, &y)?;
            opt.backward_step(&loss)?;
        }

        let weights_path = global_dir.join("models/aeon-reason.safetensors");
        varmap.save(&weights_path)?;

        Ok(format!("Substrate Ingestion Motion Successful. Distilled {} experiences into Native Tier 2 Logic Model.", samples.len()))
    }

    fn project_features(intent: &str, context: &str) -> Result<Vec<f32>> {
        let mut vec = vec![0.0f32; Self::DIM];
        let i_vec = crate::gemi::alpha::AeonAlphaModel::semantic_centroid_projection(intent)?;
        let c_vec = crate::gemi::alpha::AeonAlphaModel::semantic_centroid_projection(context)?;

        // Interleave for high-density feature mapping
        for (i, &v) in i_vec.iter().enumerate() { vec[i] = v; }
        for (i, &v) in c_vec.iter().enumerate() { vec[i + 128] = v; }

        Ok(vec)
    }

    pub fn reason(&self, intent: &str, context: &str) -> Result<Vec<f32>> {
        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let features = Self::project_features(intent, context)?;
        let input = Tensor::from_vec(features, (1, Self::DIM), &device)?;

        let out = self.l1.forward(&input)?.relu()?;
        let out = self.l2.forward(&out)?.relu()?;
        let out = self.l3.forward(&out)?.relu()?;
        let out = self.l4.forward(&out)?;

        Ok(out.to_vec2::<f32>()?[0].clone())
    }
}
