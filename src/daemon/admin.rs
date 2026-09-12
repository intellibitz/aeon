// AEON Native Administrative Substrate
// 100% Rust implementation for Full Compliance Enforcement, Version Synchronization & Release Orchestration

use std::fs;
use std::path::Path;
use std::process::Command;
use crate::error::{EaiError, EaiResult};

pub struct AeonAdmin;

impl AeonAdmin {
    /// Full Compliance Audit (Rule 15)
    pub fn audit_compliance(workspace: &Path, target: Option<&str>) -> EaiResult<String> {
        let mut report = "# AEON Compliance Audit\n\n".to_string();
        if let Some(t) = target {
             report.push_str(&format!("Target: {}\n\n", t));
        }
        let mut overall_success = true;

        // 1. Audit Security Patterns (No hardcoded keys)
        let mut secret_found = false;
        let patterns = ["sk-", "ghp_", "AIza"];
        let src_dir = workspace.join("src");
        if let Ok(entries) = fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                if path.is_file() && !file_name.contains("security.rs") && !file_name.contains("admin.rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for p in patterns {
                            if content.contains(p) {
                                secret_found = true;
                                report.push_str(&format!("- [FAIL] Security: Potential secret matching '{}' detected in {}.\n", p, path.display()));
                            }
                        }
                    }
                }
            }
        }
        if !secret_found {
            report.push_str("- [PASS] Security: No hardcoded secrets detected.\n");
        } else {
            overall_success = false;
        }

        // 2. Enforce Workspace Purity (Rule 12)
        let gitignore = workspace.join(".gitignore");
        if gitignore.exists() {
            let content = fs::read_to_string(&gitignore)?;
            if content.contains(".aeon") || content.contains(".aeon/") {
                report.push_str("- [PASS] Workspace Purity: .aeon is correctly git-ignored.\n");
            } else {
                report.push_str("- [FAIL] Workspace Purity: .aeon is NOT git-ignored.\n");
                overall_success = false;
            }
        }

        // 3. Version Consistency (Rule 1)
        match Self::enforce_version_consistency(workspace) {
            Ok(v) => report.push_str(&format!("- [PASS] Version Consistency: All manifests synchronized to v{}.\n", v)),
            Err(e) => {
                report.push_str(&format!("- [FAIL] Version Consistency: {}\n", e));
                overall_success = false;
            }
        }

        if overall_success {
            Ok(report)
        } else {
            Err(EaiError::Protocol(format!("Compliance Audit Failed:\n{}", report)))
        }
    }

    /// Enforce Version Consistency across all files using Cargo.toml as the source of truth.
    pub fn enforce_version_consistency(workspace: &Path) -> EaiResult<String> {
        let cargo_toml_path = workspace.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_toml_path)?;

        let version = content.lines()
            .find(|l| l.trim().starts_with("version = \""))
            .and_then(|l| l.split('"').nth(1))
            .ok_or_else(|| EaiError::Config("Could not find version in Cargo.toml".into()))?;

        // 1. Sync Native Launcher Cargo.toml
        let launcher_cargo = workspace.join("src/native/aeon/Cargo.toml");
        if launcher_cargo.exists() {
            let launcher_content = fs::read_to_string(&launcher_cargo)?;
            let mut updated = Vec::new();
            for line in launcher_content.lines() {
                if line.trim().starts_with("version = \"") {
                    updated.push(format!("version = \"{}\"", version));
                } else {
                    updated.push(line.to_string());
                }
            }
            fs::write(&launcher_cargo, updated.join("\n") + "\n")?;
        }

        // 2. Sync README.md Badge
        let readme_path = workspace.join("README.md");
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            let mut updated = Vec::new();
            for line in readme_content.lines() {
                if line.contains("https://img.shields.io/badge/version-v") {
                    let updated_line = format!("![AEON Version](https://img.shields.io/badge/version-v{}-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)", version);
                    updated.push(updated_line);
                } else {
                    updated.push(line.to_string());
                }
            }
            fs::write(&readme_path, updated.join("\n") + "\n")?;
        }

        // 3. Sync Governance Files (.agents/*.md)
        let governance_files = ["AGENTS.md", "ASPIRATIONS.md", "BUILD.md", "MISSIONS.md", "RUNTIME.md", "MOTIONS.md", "TOPOLOGY.md", "WORKFLOW.md"];
        for file_name in governance_files {
            let path = workspace.join(".agents").join(file_name);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let mut updated = Vec::new();
                for line in content.lines() {
                    if line.trim().starts_with("* **Current Engine Version**: `v") {
                        updated.push(format!("* **Current Engine Version**: `v{}`", version));
                    } else {
                        updated.push(line.to_string());
                    }
                }
                fs::write(&path, updated.join("\n") + "\n")?;
            }
        }

        Ok(version.to_string())
    }

    /// Checks if all files are in sync with the current Cargo.toml version.
    /// Does not modify files; returns an error if a mismatch is detected.
    pub fn verify_version_alignment(workspace: &Path) -> EaiResult<()> {
        let cargo_toml_path = workspace.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_toml_path)?;

        let version = content.lines()
            .find(|l| l.trim().starts_with("version = \""))
            .and_then(|l| l.split('"').nth(1))
            .ok_or_else(|| EaiError::Config("Could not find version in Cargo.toml".into()))?;

        // Check README
        let readme_path = workspace.join("README.md");
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            let expected_badge = format!("version-v{}-blue.svg", version);
            if !readme_content.contains(&expected_badge) {
                return Err(EaiError::Config(format!("README.md version badge is out of sync with Cargo.toml (v{}). Run 'aeon admin sync'.", version)));
            }
        }

        // Check Governance Files (.agents/*.md)
        let governance_files = ["AGENTS.md", "ASPIRATIONS.md", "BUILD.md", "MISSIONS.md", "RUNTIME.md", "MOTIONS.md", "TOPOLOGY.md", "WORKFLOW.md"];
        for file_name in governance_files {
            let path = workspace.join(".agents").join(file_name);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let expected_line = format!("* **Current Engine Version**: `v{}`", version);
                if !content.contains(&expected_line) {
                    return Err(EaiError::Config(format!("{}: version is out of sync with Cargo.toml (v{}). Run 'aeon admin sync'.", file_name, version)));
                }
            }
        }

        Ok(())
    }

    pub fn execute_release(workspace: &Path) -> EaiResult<String> {
        eprintln!("[Release Gatekeeper] 1. Executing Compliance Audit...");
        let _ = Self::audit_compliance(workspace, Some("release"))?;

        eprintln!("[Release Gatekeeper] 2. Executing Native Test Harness...");
        let output = Command::new("cargo")
            .arg("test")
            .current_dir(workspace)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EaiError::Protocol(format!("Release aborted: Native tests failed.\n{}", stderr)));
        }

        eprintln!("[Release Gatekeeper] 3. Verifying Ephemeral Mission Protocols...");
        let missions = ["identity", "status", "models"];
        for mission in missions {
            let mission_out = Command::new("cargo")
                .args(&["run", "--quiet", "--", mission])
                .current_dir(workspace)
                .output()?;

            if !mission_out.status.success() {
                let stderr = String::from_utf8_lossy(&mission_out.stderr);
                return Err(EaiError::Protocol(format!("Release aborted: Ephemeral mission '{}' failed.\n{}", mission, stderr)));
            }
        }

        Ok("Release sequence verified. Tests, Missions, and Audits passed. Substrate is ready for deployment.".into())
    }

    pub fn execute_autonomous_evolution_cycle(workspace: &Path) -> EaiResult<String> {
        let res = crate::daemon::evolution::EvolutionManager::evolve_substrate(workspace)?;
        let _ = Self::execute_release(workspace)?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_alignment_enforcement() {
        let workspace = Path::new(".");
        // This test ensures that the build fails if developer forgot to run 'aeon admin sync'
        let result = AeonAdmin::verify_version_alignment(workspace);
        assert!(result.is_ok(), "Version mismatch detected between Cargo.toml and documentation. Run 'cargo run -- admin sync' to fix.");
    }
}
