// Model Supervisor: Pre-Execution Model Inspection & Autonomous Provisioning
// 100% Rust implementation for autonomous model governance under GAWD

use std::path::Path;
use crate::error::EaiResult;
use crate::gemi::models::ModelManager;

pub struct ModelSupervisor;

impl ModelSupervisor {
    /// Inspects cloud and local models during pre-execution governance.
    /// - Checks cloud model API keys. If missing, registers notice and proceeds to local inspection.
    /// - Inspects and verifies local models (GGUF validation & test benchmarks).
    /// - If no valid local models and no cloud keys are found, automatically provisions or downloads a compatible local model based on hardware profile.
    pub fn audit_and_prepare_models(workspace: &Path) -> EaiResult<String> {
        let mut report = String::new();
        report.push_str("[ModelSupervisor] Initiating pre-execution model governance inspection...\\n");

        // 1. Cloud Model Inspection
        let cloud_env_keys = vec![
            "AEON_API_KEY",
            "MODEL_API_KEY",
            "EAI_API_KEY",
            "API_KEY",
        ];

        let mut cloud_available = false;
        let mut active_cloud_providers = Vec::new();
        for key in cloud_env_keys {
            if std::env::var(key).is_ok() {
                cloud_available = true;
                active_cloud_providers.push(key.to_string());
            }
        }

        if cloud_available {
            report.push_str(&format!(" [INFO] Cloud models active via API keys: {:?}\\n", active_cloud_providers));
        } else {
            report.push_str(" [INFO] No cloud model API keys detected. Operating in local / air-gapped mode.\\n");
        }

        // 2. Local Model Inspection & Verification
        let _local_models = ModelManager::list_models(workspace);
        let verifications = ModelManager::verify_local_models(workspace);

        let mut valid_local_found = false;
        for v in &verifications {
            if v.is_valid_gguf || v.model_id.contains("native") {
                valid_local_found = true;
                report.push_str(&format!(" [VERIFIED] Local model ready: {} (Size: {}, Status: {})\\n", v.model_id, v.file_size_formatted, v.test_inference_status));
            }
        }

        // 3. Autonomous Provisioning if Neither Cloud Nor Valid Local Models Exist
        if !cloud_available && !valid_local_found {
            report.push_str(" [WARN] No active cloud models and no verified local models found. Triggering hardware profile inspection for autonomous model bootstrapping...\\n");

            let res = ModelManager::ensure_hardware_optimal_models(workspace)?;
            report.push_str(&format!(" [SUCCESS] {}\\n", res));
        } else {
            report.push_str(" [SUCCESS] Model governance audit passed successfully.\\n");
        }

        Ok(report)
    }
}
