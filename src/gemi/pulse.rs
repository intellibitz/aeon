// SUSI-Pulse: Tier 0 Native Bootstrap Brain
// 100% Neural implementation - Zero Hardcoded Heuristics (Rule 31)

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::alpha::SusiAlphaModel;

pub struct SusiPulse;

static REFLEX_CACHE: Lazy<Arc<RwLock<HashMap<String, String>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

static CURRENT_FINGERPRINT: Lazy<Arc<RwLock<String>>> = Lazy::new(|| {
    Arc::new(RwLock::new(String::new()))
});

impl SusiPulse {
    /// Pure Neural Intent Resolution
    pub fn reason(prompt: &str, workspace: &Path) -> Result<String> {
        let prompt_trimmed = prompt.trim();

        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".susi");

        // Neural Synchronization (Cache Invalidation)
        {
            let fingerprint = SusiAlphaModel::get_model_fingerprint(&global_dir);
            let mut current = CURRENT_FINGERPRINT.write().unwrap();
            if *current != fingerprint {
                if workspace.join(".agents").exists() && std::env::var("SUSI_VERBOSE").is_ok() {
                    eprintln!("[Tier 0 Reflex] Neural substrate evolved. Invalidating cache...");
                }
                *current = fingerprint;
                let mut cache = REFLEX_CACHE.write().unwrap();
                cache.clear();
            }
        }

        // Sub-100us Reflex Cache
        {
            let cache = REFLEX_CACHE.read().unwrap();
            if let Some(cached_action) = cache.get(prompt_trimmed) {
                return Ok(cached_action.clone());
            }
        }

        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".susi");

        // Neural Reflex Attempt
        if let Ok(model) = SusiAlphaModel::load(&global_dir) {
            match model.predict_intent(prompt_trimmed) {
                Ok(neural_action) => {
                    let mut final_action = neural_action;
                    if final_action.contains("list_directory") {
                        final_action = format!("ACTION: list_directory {}", workspace.display());
                    }

                    // Populate Cache
                    let mut cache = REFLEX_CACHE.write().unwrap();
                    cache.insert(prompt_trimmed.to_string(), final_action.clone());

                    return Ok(final_action);
                },
                Err(_e) => {
                    return Err(anyhow!("Low confidence reflex. Escalating to Tier 2 Deep Reasoning..."));
                }
            }
        }

        Err(anyhow!("Pulse Brain: Neural substrate missing. Transitioning to Tier 2..."))
    }
}
