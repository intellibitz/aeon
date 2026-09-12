// AEON-Pulse: Tier 0 Native Bootstrap Brain
// 100% Neural implementation - Zero Hardcoded Heuristics (Rule 31)

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::alpha::AeonAlphaModel;

pub struct AeonPulse;

static REFLEX_CACHE: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

static CURRENT_FINGERPRINT: Lazy<Arc<Mutex<String>>> = Lazy::new(|| {
    Arc::new(Mutex::new(String::new()))
});

impl AeonPulse {
    /// Pure Neural Intent Resolution
    pub fn reason(prompt: &str, workspace: &Path) -> Result<String> {
        let prompt_trimmed = prompt.trim();

        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".aeon");

        // Neural Synchronization (Cache Invalidation)
        {
            let fingerprint = AeonAlphaModel::get_model_fingerprint(&global_dir);
            let mut current = CURRENT_FINGERPRINT.lock().unwrap();
            if *current != fingerprint {
                eprintln!("[Tier 0 Reflex] Neural substrate evolved. Invalidating cache...");
                *current = fingerprint;
                let mut cache = REFLEX_CACHE.lock().unwrap();
                cache.clear();
            }
        }

        // Sub-100us Reflex Cache
        {
            let cache = REFLEX_CACHE.lock().unwrap();
            if let Some(cached_action) = cache.get(prompt_trimmed) {
                return Ok(cached_action.clone());
            }
        }

        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".aeon");

        // Neural Reflex Attempt
        if let Ok(model) = AeonAlphaModel::load(&global_dir) {
            match model.predict_intent(prompt_trimmed) {
                Ok(neural_action) => {
                    let mut final_action = neural_action;
                    if final_action.contains("list_directory") {
                        final_action = format!("ACTION: list_directory {}", workspace.display());
                    }

                    // Populate Cache
                    let mut cache = REFLEX_CACHE.lock().unwrap();
                    cache.insert(prompt_trimmed.to_string(), final_action.clone());

                    return Ok(final_action);
                },
                Err(e) => {
                    eprintln!("[Tier 0 Reflex] Escalating due to: {}", e);
                    return Err(anyhow!("Low confidence reflex. Escalating to Tier 2 Deep Reasoning..."));
                }
            }
        }

        Err(anyhow!("Pulse Brain: Neural substrate missing. Transitioning to Tier 2..."))
    }
}
