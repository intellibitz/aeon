// SUSI Runtime Administrator: Substrate Maintenance & Hardware Optimization
// Sole Purpose: Interrogate hardware, provision models, and ensure peak execution state.

use std::path::Path;
use std::thread;
use std::time::Duration;
use crate::error::EaiResult;
use crate::gemi::hardware::HardwareProfiler;
use crate::gemi::models::ModelManager;
use crate::sandbox::manager::SusiConfig;

pub struct SusiRuntimeAdmin;

impl SusiRuntimeAdmin {
    /// Active Background Administration Loop
    pub fn start_administration_cycle(workspace: &Path) {
        let ws = workspace.to_path_buf();
        thread::spawn(move || {
            let ama = crate::gawd::ama::SusiMasterAgent::new();
            loop {
                // Mandate: Use Swarm for all runtime mandates
                let _ = ama.process_intent("perform full substrate audit, hardware optimization, and evolutionary maintenance", &ws);
                thread::sleep(Duration::from_secs(3600)); // Foundational Audit Every Hour
            }
        });
    }

    /// Performs hardware interrogation, model selection, and maintenance.
    pub fn execute_full_audit(workspace: &Path) -> EaiResult<String> {
        eprintln!("[Runtime Admin] Initializing Substrate Audit...");

        // 1. Hardware Interrogation
        let profile = HardwareProfiler::get_profile();
        eprintln!("[Runtime Admin] Hardware Detected: {} | {}GB RAM | Acceleration: {}",
            profile.cpu_brand, profile.ram_gb, profile.native_acceleration);

        // 2. Model Provisioning & Optimization
        let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".susi");
        let cfg = SusiConfig::load(&global_dir).expect("Fatal: Malformed configuration");

        if cfg.auto_download_models {
            eprintln!("[Runtime Admin] Auditing model substrate...");

            // Ensure hardware-optimal models are present
            let res = ModelManager::ensure_hardware_optimal_models(workspace);
            if let Ok(report) = res {
                if !report.contains("Substrate optimal") {
                    eprintln!("[Runtime Admin] Substrate Optimization: {}", report);
                }
            }

            // Select best performing model if none active
            if ModelManager::get_selected_model().is_none() {
                let ladder = HardwareProfiler::get_progressive_model_ladder();
                if let Some(best_step) = ladder.last() {
                    eprintln!("[Runtime Admin] Defaulting to peak hardware model: {}", best_step.label);
                }
            }
        }

        // 3. Autonomous Self-Validation (Aspiration 15)
        let validation_res = Self::execute_autonomous_self_validation(workspace);
        match validation_res {
            Ok(_) => eprintln!("[Runtime Admin] Substrate status: OPTIMAL."),
            Err(e) => eprintln!("[Runtime Admin] Substrate status: DEGRADED. Validation Error: {}", e),
        }

        // 4. Substrate Maintenance
        Self::perform_maintenance(workspace)?;

        Ok("Substrate optimized by Runtime Admin.".into())
    }

    /// Empirical Self-Testing on Host Hardware (Aspiration 15)
    pub fn execute_autonomous_self_validation(workspace: &Path) -> EaiResult<String> {
        use std::process::Command;

        eprintln!("[Runtime Admin] Initializing Autonomous Self-Validation...");

        let output = Command::new("cargo")
            .arg("test")
            .arg("--quiet")
            .current_dir(workspace)
            .output()?;

        if output.status.success() {
            Ok("All Validation Protocols Passed. Substrate is Optimal.".into())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(crate::error::EaiError::process(format!("Self-Validation Failed: {}", stderr)))
        }
    }

    fn perform_maintenance(workspace: &Path) -> EaiResult<()> {
        // Prune stale interactions or temporary artifacts
        let _ = crate::daemon::evolution::EvolutionManager::perform_autonomous_drift_audit(workspace);
        Ok(())
    }
}
