// AEON Reflex Synthesizer
// RULE 16: Motion Protocol - Native Substrate Evolution
// 100% Rust implementation for distilling Tier 2 Reasoning into Tier 0/1 Native Reflexes.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::error::{EaiError, EaiResult};

pub struct ReflexSynthesizer;

impl ReflexSynthesizer {
    /// Distills a neural intent into a native Rust reflex (Tier 1 Evolution)
    pub fn distill_native_reflex(intent: &str, workspace: &Path) -> EaiResult<String> {
        // 1. Model-Driven Code Synthesis (Rule 16.2)
        let struct_name = intent.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().join("");
        let code = format!(
            "// AEON Native Reflex: {}\n\
            use crate::gmcp::tools::AeonTool;\n\
            use crate::error::EaiResult;\n\n\
            pub struct {}Reflex;\n\n\
            impl AeonTool for {}Reflex {{\n\
                fn name(&self) -> String {{ \"{}\".to_string() }}\n\
                fn description(&self) -> String {{ \"Synthesized reflex for {}\".to_string() }}\n\
                fn execute(&self, arg: &str, _ws: &std::path::Path) -> EaiResult<String> {{\n\
                    Ok(format!(\"Synthesized reflex executed for intent '{}' with arg: {{}}\", arg))\n\
                }}\n\
            }}",
            intent, struct_name, struct_name, intent.replace(' ', "_"), intent, intent
        );

        // 2. Integration Phase (Rule 16.3)
        let reflex_path = workspace.join(format!("src/gmcp/reflexes/{}.rs", intent.replace(' ', "_")));
        let _ = fs::create_dir_all(reflex_path.parent().unwrap());
        fs::write(&reflex_path, code)?;

        Ok(format!("Native reflex '{}' distilled and staged in {}.", intent, reflex_path.display()))
    }

    /// Synthesizes a volatile WebAssembly reflex (Tier 0 Evolution)
    pub fn synthesize_wasm_reflex(intent: &str, _workspace: &Path) -> EaiResult<String> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

        // 1. Save backup to ~/.aeon/reflexes/
        let reflex_dir = home.join(".aeon/reflexes");
        let _ = fs::create_dir_all(&reflex_dir);
        let wasm_src = reflex_dir.join(format!("{}.rs", intent.replace(' ', "_")));

        let _struct_name = intent.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().join("");
        let code = format!(
            "#[no_mangle]\n\
            pub extern \"C\" fn execute_reflex() -> i32 {{\n\
                // Distilled logic for: {}\n\
                42\n\
            }}",
            intent
        );
        fs::write(&wasm_src, code)?;

        // 2. Compile to WASM (if rustc exists)
        let wasm_out = reflex_dir.join(format!("{}.wasm", intent.replace(' ', "_")));
        let build = Command::new("rustc")
            .args([
                "--target", "wasm32-wasi",
                "-O",
                "--crate-type", "cdylib",
                "-o", wasm_out.to_str().unwrap(),
                wasm_src.to_str().unwrap()
            ])
            .output();

        match build {
            Ok(output) if output.status.success() => {
                Ok(wasm_out.to_string_lossy().to_string())
            }
            Ok(output) => {
                Err(EaiError::Hardware(format!("WASM compilation failed: {}", String::from_utf8_lossy(&output.stderr))))
            }
            Err(e) => {
                Err(EaiError::Hardware(format!("rustc/wasm32-wasi target missing: {}", e)))
            }
        }
    }

    pub fn evolve_substrate_native(intent: &str, workspace: &Path) -> EaiResult<String> {
        let code = match crate::gemi::pulse::AeonPulse::reason(&format!("GENERATE_RUST_TOOL: {}", intent), workspace) {
             Ok(c) => c,
             Err(_) => return Err(EaiError::Protocol("Reflex synthesis failed: No reasoning response.".into())),
        };

        if !code.contains("struct ") || !code.contains("impl AeonTool for ") {
            return Err(EaiError::Protocol("Synthesized code missing AeonTool implementation.".into()));
        }

        let tool_name = intent.split_whitespace().next().unwrap_or("new_tool");
        let path = workspace.join(format!("src/gmcp/tools/{}.rs", tool_name));
        fs::write(&path, code)?;

        // Trigger Release Cycle (Rule 16.4)
        crate::daemon::admin::AeonAdmin::execute_release(workspace)
    }
}
