// SUSI Core Runtime Substrate: Unified Operational Status Tracking
// Unifies Self (Compiled Binary Instructions), System Environment (Hardware/OS), and User (Configurations/Workspace).

use std::path::{Path, PathBuf};
use super::self_core::AlphaSelf;
use crate::gemi::hardware::HardwareProfiler;
use crate::sandbox::manager::SusiConfig;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AlphaBrainContext {
    pub self_version: &'static str,
    pub system_cpus: usize,
    pub system_gpu: String,
    pub system_ram_gb: usize,
    pub workspace_path: PathBuf,
    pub default_engine: String,
    pub default_model: String,
    pub gmcp_port: u16,
    pub gemi_port: u16,
}

impl AlphaBrainContext {
    pub fn initialize(workspace: &Path) -> Self {
        let hardware = HardwareProfiler::get_profile();
        let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let global_dir = home.join(".susi");
        let cfg = SusiConfig::load(&global_dir).expect("Fatal: Malformed configuration");

        Self {
            self_version: AlphaSelf::VERSION,
            system_cpus: hardware.cpus,
            system_gpu: hardware.gpu_info,
            system_ram_gb: hardware.ram_gb,
            workspace_path: workspace.to_path_buf(),
            default_engine: cfg.default_engine,
            default_model: cfg.default_model,
            gmcp_port: cfg.gmcp_port,
            gemi_port: cfg.gemi_port,
        }
    }

    #[allow(dead_code)]
    pub fn inspect_tri_state(&self) -> String {
        format!(
            "SUSI Core Substrate Status:\n\
             1. [CORE - Compiled System]: Version {}, {} Baked Rules, {} Baked Components\n\
             2. [HARDWARE - System Environment]: {} CPUs | {} | {}GB RAM\n\
             3. [DYNAMIC - Runtime Configuration]: Workspace: {} | Engine: {} | Model: {} | GMCP Port: {} | GEMI Port: {}",
            self.self_version,
            AlphaSelf::RULES.len(),
            AlphaSelf::COMPONENTS.len(),
            self.system_cpus,
            self.system_gpu,
            self.system_ram_gb,
            self.workspace_path.display(),
            self.default_engine,
            self.default_model,
            self.gmcp_port,
            self.gemi_port
        )
    }
}
