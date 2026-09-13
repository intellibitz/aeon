// 100% Rust implementation for autonomous hardware profiling

use std::sync::{RwLock, OnceLock};
use serde::{Deserialize, Serialize};
use candle_core::Device;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpus: usize,
    pub cpu_brand: String,
    pub gpu_info: String,
    pub ram_gb: usize,
    pub available_ram_gb: usize,
    pub gpu_vram_gb: usize,
    pub swap_gb: usize,
    pub nvme_active: bool,
    pub acceleration_active: bool,
    pub native_acceleration: String,
    pub os_info: String,
    pub arch: String,
    pub disk_gb: usize,
    pub disk_usage_pct: u8,
    pub load_avg: String,
    pub uptime: String,
    pub hostname: String,
}

pub struct HardwareProfiler;

impl HardwareProfiler {
    pub fn get_profile() -> HardwareProfile {
        static CACHED_PROFILE: OnceLock<HardwareProfile> = OnceLock::new();
        CACHED_PROFILE.get_or_init(|| {
            let (cpus, _) = Self::profile();
            let ram_gb = Self::determine_total_ram_gb();
            let available_ram_gb = Self::determine_available_ram_gb();
            let gpu_vram_gb = Self::determine_gpu_vram_gb();
            let swap_gb = Self::determine_swap_gb();
            let nvme_active = Self::is_nvme_active();

            // 1. Direct Interrogation via Candle Substrate
            let (native_accel, gpu_name) = Self::interrogate_native_acceleration();

            let acceleration_active = !native_accel.contains("None") && !native_accel.contains("Cpu");
            let gpu_display = if acceleration_active {
                format!("{} ({} | {}GB VRAM)", native_accel, gpu_name, gpu_vram_gb)
            } else {
                // 2. Fallback to Meta-Parsing for diagnostics if native probe is inactive
                let (_, shell_gpu) = Self::profile();
                shell_gpu
            };

            HardwareProfile {
                cpus,
                cpu_brand: Self::get_cpu_brand(),
                gpu_info: gpu_display,
                ram_gb,
                available_ram_gb,
                gpu_vram_gb,
                swap_gb,
                nvme_active,
                acceleration_active,
                native_acceleration: native_accel,
                os_info: Self::get_os_info(),
                arch: std::env::consts::ARCH.to_string(),
                disk_gb: Self::determine_disk_gb(),
                disk_usage_pct: Self::determine_disk_usage_pct(),
                load_avg: Self::get_load_avg(),
                uptime: Self::get_uptime(),
                hostname: Self::get_hostname(),
            }
        }).clone()
    }

    pub fn check_oom_critical() -> bool {
        let profile = Self::get_profile();
        if profile.ram_gb == 0 { return false; }
        let usage_pct = ((profile.ram_gb - profile.available_ram_gb) as f32 / profile.ram_gb as f32) * 100.0;
        usage_pct > 90.0
    }

    fn get_cpu_brand() -> String {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("model name") {
                        return line.split(':').nth(1).unwrap_or("Unknown CPU").trim().to_string();
                    }
                }
            }
        }
        "Generic Hardware Substrate".to_string()
    }

    fn get_load_avg() -> String {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 3 {
                    return format!("{}, {}, {}", parts[0], parts[1], parts[2]);
                }
            }
        }
        "N/A".to_string()
    }

    fn get_uptime() -> String {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
                if let Some(secs_str) = content.split_whitespace().next() {
                    if let Ok(secs) = secs_str.parse::<f64>() {
                        let hours = (secs / 3600.0) as u64;
                        let mins = ((secs % 3600.0) / 60.0) as u64;
                        return format!("up {} hours, {} minutes", hours, mins);
                    }
                }
            }
        }
        "N/A".to_string()
    }

    fn get_hostname() -> String {
        std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| {
                std::fs::read_to_string("/etc/hostname")
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| "localhost".to_string())
            })
    }

    fn determine_disk_usage_pct() -> u8 {
        0
    }

    pub fn get_caps_string() -> String {
        let profile = Self::get_profile();
        let mut caps = Vec::new();
        if profile.acceleration_active {
            caps.push("GPU".to_string());
        } else {
            caps.push("CPU".to_string());
        }
        caps.push(format!("{}GB", profile.ram_gb));
        caps.push(format!("{}V", crate::AEON_VERSION));
        caps.join(",")
    }

    pub fn get_candle_device() -> Device {
        // Dynamic Device Refresh: Re-scan for acceleration if previously CPU-bound
        static DEVICE_CACHE: OnceLock<RwLock<Device>> = OnceLock::new();
        let cache = DEVICE_CACHE.get_or_init(|| RwLock::new(Device::Cpu));

        let mut dev = cache.write().unwrap();
        if dev.is_cpu() {
            // Attempt CUDA initialization with panic safety
            let cuda_attempt = std::panic::catch_unwind(|| {
                Device::new_cuda(0)
            });
            if let Ok(Ok(cuda_dev)) = cuda_attempt {
                *dev = cuda_dev.clone();
                return cuda_dev;
            }

            // Attempt Metal initialization with panic safety
            #[cfg(feature = "metal")]
            {
                let metal_attempt = std::panic::catch_unwind(|| {
                    Device::new_metal(0)
                });
                if let Ok(Ok(metal_dev)) = metal_attempt {
                    *dev = metal_dev.clone();
                    return metal_dev;
                }
            }
        }
        // Absolute Fallback: CPU
        Device::Cpu
    }

    fn get_os_info() -> String {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        return line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string();
                    }
                }
            }
            return "Linux".to_string();
        } else if cfg!(target_os = "macos") {
            return "macOS".to_string();
        } else if cfg!(target_os = "windows") {
             return "Windows".to_string();
        }
        "Unknown OS".to_string()
    }

    fn determine_disk_gb() -> usize {
        256 // Fallback - Meta Interrogation Required
    }

    fn interrogate_native_acceleration() -> (String, String) {
        if candle_core::utils::cuda_is_available() {
             return ("CUDA (Detected)".to_string(), "NVIDIA Driver found".to_string());
        }

        if candle_core::utils::metal_is_available() {
             return ("Metal (Detected)".to_string(), "Apple Silicon / macOS".to_string());
        }

        ("None".to_string(), "Cpu".to_string())
    }

    fn determine_gpu_vram_gb() -> usize {
        0
    }

    fn determine_swap_gb() -> usize {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("SwapTotal:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(kb_str) = parts.get(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb / (1024 * 1024);
                            }
                        }
                    }
                }
            }
        }
        0
    }

    fn is_nvme_active() -> bool {
        if cfg!(target_os = "linux") {
            if let Ok(entries) = std::fs::read_dir("/sys/block/") {
                for entry in entries.flatten() {
                    if entry.file_name().to_string_lossy().starts_with("nvme") {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn profile() -> (usize, String) {
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        let gpu_info = if candle_core::utils::cuda_is_available() {
            "CUDA Acceleration Substrate Active".to_string()
        } else if candle_core::utils::metal_is_available() {
            "Metal Acceleration Substrate Active".to_string()
        } else {
            format!("CPU Parallel Execution Substrate Active ({} Threads)", cpus)
        };

        (cpus, gpu_info)
    }

    pub fn determine_total_ram_gb() -> usize {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(kb_str) = parts.get(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb / (1024 * 1024);
                            }
                        }
                    }
                }
            }
        } else if cfg!(target_os = "macos") {
            return 16; // macOS Meta Interrogation Required
        } else if cfg!(target_os = "windows") {
            return 16; // Windows Meta Interrogation Required
        }
        8
    }

    pub fn determine_available_ram_gb() -> usize {
        if cfg!(target_os = "linux") {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("MemAvailable:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(kb_str) = parts.get(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb / (1024 * 1024);
                            }
                        }
                    }
                }
            }
        }
        Self::determine_total_ram_gb() // Fallback
    }

    pub fn get_progressive_model_ladder() -> Vec<ModelLadderStep> {
        let ram_gb = Self::determine_total_ram_gb();
        let mut ladder = vec![
            ModelLadderStep {
                step: 1,
                label: "1.5B Parameters (Fast Local Edge)",
                hf_repo: "aeon-alpha/aeon-alpha-1.5b-instruct-v0.1-GGUF",
                hf_file: "aeon-alpha-1.5b-instruct-q4_k_m.gguf",
            },
        ];

        if ram_gb >= 8 {
            ladder.push(ModelLadderStep {
                step: 2,
                label: "7B Parameters (Mid-Range Desktop)",
                hf_repo: "aeon-alpha/aeon-alpha-7b-instruct-v0.1-GGUF",
                hf_file: "aeon-alpha-7b-instruct-q4_k_m.gguf",
            });
        }
        if ram_gb >= 16 {
            ladder.push(ModelLadderStep {
                step: 3,
                label: "14B Parameters (High-Accuracy Workstation)",
                hf_repo: "aeon-alpha/aeon-alpha-14b-instruct-v0.1-GGUF",
                hf_file: "aeon-alpha-14b-instruct-q4_k_m.gguf",
            });
        }
        if ram_gb >= 32 {
            ladder.push(ModelLadderStep {
                step: 4,
                label: "32B Parameters (High-End Workstation)",
                hf_repo: "aeon-alpha/aeon-alpha-32b-instruct-v0.1-GGUF",
                hf_file: "aeon-alpha-32b-instruct-q4_k_m.gguf",
            });
        }
        if ram_gb >= 64 {
            ladder.push(ModelLadderStep {
                step: 5,
                label: "72B Parameters (Ultra-Capacity Workstation)",
                hf_repo: "aeon-alpha/aeon-alpha-72b-instruct-v0.1-GGUF",
                hf_file: "aeon-alpha-72b-instruct-q4_k_m.gguf",
            });
        }

        ladder
    }
}

#[derive(Debug, Clone)]
pub struct ModelLadderStep {
    pub step: usize,
    pub label: &'static str,
    pub hf_repo: &'static str,
    pub hf_file: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_profile_generation() {
        let profile = HardwareProfiler::get_profile();
        assert!(profile.cpus > 0);
        assert!(profile.ram_gb > 0);
    }

    #[test]
    fn test_progressive_model_ladder_ordering() {
        let ladder = HardwareProfiler::get_progressive_model_ladder();
        assert!(!ladder.is_empty());
        let mut last_step = 0;
        for step in ladder {
            assert!(step.step > last_step);
            last_step = step.step;
            assert!(!step.label.is_empty());
            assert!(!step.hf_repo.is_empty());
            assert!(!step.hf_file.is_empty());
        }
    }

    #[test]
    fn test_candle_device_retrieval() {
        let device = HardwareProfiler::get_candle_device();
        // Simply ensure it doesn't panic and returns a valid variant
        match device {
            candle_core::Device::Cpu => {}
            candle_core::Device::Cuda(_) => {}
            candle_core::Device::Metal(_) => {}
        }
    }
}
