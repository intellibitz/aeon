// susi Launcher: Micro-binary for Exponential Intelligence Substrate Onboarding

use std::process::Command;
use std::env;
use std::path::PathBuf;

const SUSI_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let home = env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let global_dir = home.join(".susi");
    let engine_path = global_dir.join("bin").join("susi-engine");

    if args.first().map(|s| s.as_str()) == Some("install") {
        println!("susi Launcher v{}", SUSI_VERSION);
    }

    if engine_path.exists() {
        let status = Command::new(engine_path)
            .args(&args)
            .status();

        if let Err(e) = status {
            eprintln!("Error executing susi-engine: {}", e);
        }
    } else {
        // If engine not found, try searching in path or current dir
        let fallback = if cfg!(target_os = "windows") { "susi-engine.exe" } else { "susi-engine" };
        let status = Command::new(fallback)
            .args(&args)
            .status();

        if let Err(_) = status {
            eprintln!("susi substrate engine not found. Please run 'susi install'.");
        }
    }
}
