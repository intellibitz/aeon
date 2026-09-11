// aeon Launcher: Micro-binary for Exponential Intelligence Substrate Onboarding

use std::process::Command;
use std::env;
use std::path::PathBuf;

const AEON_VERSION: &str = "0.1.2022684";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let home = env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let global_dir = home.join(".aeon");
    let engine_path = global_dir.join("bin").join("aeon-engine");

    if args.first().map(|s| s.as_str()) == Some("install") {
        println!("aeon Launcher v{}", AEON_VERSION);
    }

    if engine_path.exists() {
        let status = Command::new(engine_path)
            .args(&args)
            .status();

        if let Err(e) = status {
            eprintln!("Error executing aeon-engine: {}", e);
        }
    } else {
        // If engine not found, try searching in path or current dir
        let fallback = if cfg!(target_os = "windows") { "aeon-engine.exe" } else { "aeon-engine" };
        let status = Command::new(fallback)
            .args(&args)
            .status();

        if let Err(_) = status {
            eprintln!("aeon substrate engine not found. Please run 'aeon install'.");
        }
    }
}
