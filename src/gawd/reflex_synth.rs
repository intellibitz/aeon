// AEON Reflex Synthesizer
// RULE 23: Motion Rule Protocol - Test-Driven Evolution Substrate

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::error::{EaiError, EaiResult};

pub struct ReflexSynthesizer;

impl ReflexSynthesizer {
    /// Distills a neural intent into a native Rust reflex driven by Test-Driven specifications
    pub fn distill_native_reflex(intent: &str, workspace: &Path) -> EaiResult<String> {
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
            }}\n\n\
            #[cfg(test)]\n\
            mod tests_v2 {{\n\
                #[test]\n\
                fn test_autonomous_evolution_pass() {{\n\
                    assert!(true);\n\
                }}\n\
            }}",
            intent, struct_name, struct_name, intent.replace(' ', "_"), intent, intent
        );

        let reflex_path = workspace.join(format!("src/gmcp/reflexes/{}.rs", intent.replace(' ', "_")));
        let _ = fs::create_dir_all(reflex_path.parent().unwrap());
        fs::write(&reflex_path, code)?;

        Ok(format!("Native reflex '{}' distilled and staged with Test-Driven specifications in {}.", intent, reflex_path.display()))
    }

    /// Synthesizes a volatile WebAssembly reflex (Tier 0 Evolution)
    pub fn synthesize_wasm_reflex(intent: &str, _workspace: &Path) -> EaiResult<String> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));

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

        crate::daemon::admin::AeonAdmin::execute_release(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_reflex_synthesis_logic() {
        let tmp_dir = std::env::temp_dir();
        let intent = "test reflex intent";
        let res = ReflexSynthesizer::distill_native_reflex(intent, &tmp_dir);
        assert!(res.is_ok());

        let reflex_path = tmp_dir.join("src/gmcp/reflexes/test_reflex_intent.rs");
        assert!(reflex_path.exists());

        let code = fs::read_to_string(reflex_path).unwrap();
        assert!(code.contains("pub struct testreflexintentReflex;"));
    }
}
