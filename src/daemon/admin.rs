// SUSI Native Administrative Substrate
// 100% Rust implementation for Full Compliance Enforcement, Version Synchronization & Release Orchestration

use std::fs;
use std::path::Path;
use std::process::Command;
use crate::error::{EaiError, EaiResult};

pub struct SusiAdmin;

impl SusiAdmin {
    /// Full Compliance Audit (Rule 15)
    pub fn audit_compliance(workspace: &Path, target: Option<&str>) -> EaiResult<String> {
        let mut report = "# SUSI Compliance Audit\n\n".to_string();
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
            if content.contains(".susi") || content.contains(".susi/") {
                report.push_str("- [PASS] Workspace Purity: .susi is correctly git-ignored.\n");
            } else {
                report.push_str("- [FAIL] Workspace Purity: .susi is NOT git-ignored.\n");
                overall_success = false;
            }
        }

        // 3. Model Integrity & Provenance (Rule 31)
        let model_verifications = crate::gemi::models::ModelManager::verify_local_models(workspace);
        if model_verifications.is_empty() {
            report.push_str("- [WARNING] Models: No local model substrates found.\n");
        } else {
            for v in model_verifications {
                let status = if v.checksum_verified { "PASS" } else { "FAIL" };
                report.push_str(&format!("- [{}] Model Integrity: {} (Verified: {})\n", status, v.model_id, v.checksum_verified));
                if !v.checksum_verified { overall_success = false; }
            }
        }

        // 4. Binary Integrity Check (Aspiration 4)
        if let Ok(current_exe) = std::env::current_exe() {
            let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
            let global_dir = home.join(".susi");
            match crate::daemon::server::SusiDaemon::verify_binary_integrity(&current_exe, &global_dir) {
                Ok(true) => report.push_str("- [PASS] Binary Integrity: Executable hash matches trusted genome.\n"),
                Ok(false) => {
                    report.push_str("- [FAIL] Binary Integrity: Executable hash MISMATCH. Potential tampering or build drift.\n");
                    overall_success = false;
                }
                Err(e) => report.push_str(&format!("- [WARNING] Binary Integrity: Could not verify ({})\n", e)),
            }
        }

        // 5. Version Consistency (Rule 1)
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
            Err(EaiError::governance(format!("Compliance Audit Failed:\n{}", report)))
        }
    }

    /// Enforce Version Consistency across all files using Cargo.toml as the source of truth.
    pub fn enforce_version_consistency(workspace: &Path) -> EaiResult<String> {
        let cargo_toml_path = workspace.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_toml_path)?;

        let version = content.lines()
            .find(|l| l.trim().starts_with("version = \""))
            .and_then(|l| l.split('"').nth(1))
            .ok_or_else(|| EaiError::config("Could not find version in Cargo.toml"))?;

        // 1. Sync Native Launcher Cargo.toml
        let launcher_cargo = workspace.join("src/native/susi/Cargo.toml");
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
                    let updated_line = format!("![SUSI Version](https://img.shields.io/badge/version-v{}-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)", version);
                    updated.push(updated_line);
                } else {
                    updated.push(line.to_string());
                }
            }
            fs::write(&readme_path, updated.join("\n") + "\n")?;
        }

        // 3. Sync Governance Files (.agents/*.md)
        let governance_files = ["AGENTS.md", "ASPIRATIONS.md", "BUILD.md", "CREATORS.md", "MISSIONS.md", "QUERIES.md", "RUNTIME.md", "MOTIONS.md", "TOPOLOGY.md", "WORKFLOW.md"];
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

        // 4. Update Binary Integrity Hash
        if let Ok(current_exe) = std::env::current_exe() {
            let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
            let global_dir = home.join(".susi");
            let _ = fs::create_dir_all(&global_dir);
            let hash_file = global_dir.join("binary.hash");

            use sha2::{Sha256, Digest};
            if let Ok(mut file) = fs::File::open(&current_exe) {
                let mut hasher = Sha256::new();
                let mut buffer = [0u8; 65536];
                while let Ok(n) = std::io::Read::read(&mut file, &mut buffer) {
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                let hash = format!("{:x}", hasher.finalize());
                let _ = fs::write(&hash_file, hash);
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
            .ok_or_else(|| EaiError::config("Could not find version in Cargo.toml"))?;

        // Check README
        let readme_path = workspace.join("README.md");
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            let expected_badge = format!("version-v{}-blue.svg", version);
            if !readme_content.contains(&expected_badge) {
                return Err(EaiError::config(format!("README.md version badge is out of sync with Cargo.toml (v{}). Run 'susi admin sync'.", version)));
            }
        }

        // Check Governance Files (.agents/*.md)
        let governance_files = ["AGENTS.md", "ASPIRATIONS.md", "BUILD.md", "CREATORS.md", "MISSIONS.md", "QUERIES.md", "RUNTIME.md", "MOTIONS.md", "TOPOLOGY.md", "WORKFLOW.md"];
        for file_name in governance_files {
            let path = workspace.join(".agents").join(file_name);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let expected_line = format!("* **Current Engine Version**: `v{}`", version);
                if !content.contains(&expected_line) {
                    return Err(EaiError::config(format!("{}: version is out of sync with Cargo.toml (v{}). Run 'susi admin sync'.", file_name, version)));
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
            return Err(EaiError::process(format!("Release aborted: Native tests failed.\n{}", stderr)));
        }

        eprintln!("[Release Gatekeeper] 3. Executing Static Analysis (Clippy)...");
        let clippy = Command::new("cargo")
            .args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
            .current_dir(workspace)
            .output()?;
        if !clippy.status.success() {
            let stderr = String::from_utf8_lossy(&clippy.stderr);
            return Err(EaiError::process(format!("Release aborted: Linting failed.\n{}", stderr)));
        }

        eprintln!("[Release Gatekeeper] 4. Verifying Ephemeral Mission Protocols...");
        let missions = ["identity", "status", "models"];
        for mission in missions {
            let mission_out = Command::new("cargo")
                .args(["run", "--quiet", "--", mission])
                .current_dir(workspace)
                .output()?;

            if !mission_out.status.success() {
                let stderr = String::from_utf8_lossy(&mission_out.stderr);
                return Err(EaiError::process(format!("Release aborted: Ephemeral mission '{}' failed.\n{}", mission, stderr)));
            }
        }

        Ok("Release sequence verified. Tests, Audits, and Lints passed. Substrate is ready for deployment.".into())
    }

    /// Dynamic Neural Cascade Classifier (Tier 0 Reflex -> Tier 2 GEMI -> Motion)
    pub fn classify_natural_intent(workspace: &Path, intent: &str) -> (&'static str, &'static str) {
        let trimmed = intent.trim();
        let lower = trimmed.to_lowercase();

        // Fast-Path Reflex for Standard Queries/Motions
        if lower == "identity" || lower == "status" || lower == "models" || lower == "version" ||
           lower.starts_with("susi status") || lower.starts_with("susi identity") || lower.starts_with("susi models") {
            return ("[QUERY]", "Zero-Mutation Interrogation");
        }

        if lower.contains("motion") || lower.contains("architecture") || lower.contains("hardcode") {
            return ("[MOTION]", "Architectural Evolution");
        }

        // Tier 0 Neural Reflex Attempt via Local Alpha Model
        if let Ok(reflex_action) = crate::gemi::pulse::SusiPulse::reason(trimmed, workspace) {
            let reflex_lower = reflex_action.to_lowercase();
            if reflex_lower.contains("query") || reflex_lower.contains("status") || reflex_lower.contains("identity") {
                return ("[QUERY]", "Zero-Mutation Interrogation");
            } else if reflex_lower.contains("motion") || reflex_lower.contains("recompile") {
                return ("[MOTION]", "Architectural Evolution");
            } else if reflex_lower.contains("mission") || reflex_lower.contains("solve") {
                return ("[MISSION]", "Dynamic Task Fulfillment");
            }
        }

        // Default to Dynamic Task Fulfillment Mission
        ("[MISSION]", "Dynamic Task Fulfillment")
    }

    /// Ingest a natural language intent and automatically inject it into pulse.md
    /// Supports both Creator mode (.agents/pulse.md) and World User mode (.susi/pulse.md).
    pub fn ingest_natural_intent(workspace: &Path, intent: &str) -> EaiResult<String> {
        let mut pulse_path = workspace.join(".agents/pulse.md");

        // World User Fallback: If .agents/ is missing, use .susi/ sandbox
        if !pulse_path.exists() {
            pulse_path = workspace.join(".susi/pulse.md");
            if !pulse_path.exists() {
                 // Synthesize a new local pulse from hard-compiled genome if missing
                 let _ = fs::create_dir_all(workspace.join(".susi"));
                 fs::write(&pulse_path, crate::gawd::self_core::AlphaSelf::PULSE_MD)?;
            }
        }

        // 1. Dynamic Neural Cascade Classifier (Tier 0 Reflex -> Tier 2 GEMI -> Motion)
        let (prefix, _category) = Self::classify_natural_intent(workspace, intent);

        // 2. Read pulse.md and find the last index in section 2
        let content = fs::read_to_string(&pulse_path)?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // 2.5 Prevent Duplicate Intent Ingestion (Rule: pulse contains only unique entries)
        let intent_trimmed = intent.trim().to_lowercase();
        let is_duplicate = lines.iter().any(|l| {
            let l_lower = l.to_lowercase();
            if let Some(idx) = l_lower.find("**: ") {
                let existing = l_lower[idx + 4..].trim_end_matches('`').trim();
                existing == intent_trimmed
            } else if let Some(idx) = l_lower.find("**: ") {
                let existing = l_lower[idx + 4..].trim_end_matches('`').trim();
                existing == intent_trimmed
            } else if let Some(idx) = l_lower.find("**:") {
                let existing = l_lower[idx + 3..].trim_end_matches('`').trim();
                existing == intent_trimmed
            } else {
                false
            }
        });

        if is_duplicate {
            return Ok(format!("Intent '{}' is already present in pulse.md ({})", intent, prefix));
        }

        let last_index = lines.iter()
            .filter_map(|l| {
                let trimmed = l.trim();
                if trimmed.is_empty() || !trimmed.chars().next().unwrap().is_ascii_digit() { return None; }
                trimmed.split('.').next()?.parse::<usize>().ok()
            })
            .max()
            .unwrap_or(0);

        let new_index = last_index + 1;
        let entry = format!("{}. [ ] **{}**: {}", new_index, prefix, intent);

        // 3. Inject into Section 1 (Pending)
        let mut section1_start = None;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("## 1. Pending Failing Pulse") {
                section1_start = Some(i);
                break;
            }
        }

        if let Some(start) = section1_start {
             // Find insertion point (after header, before next section)
             let mut insert_pos = start + 1;
             while insert_pos < lines.len() && (lines[insert_pos].trim().is_empty() || lines[insert_pos].trim().starts_with("(No pending")) {
                 if lines[insert_pos].trim().starts_with("(No pending") {
                     lines.remove(insert_pos);
                     continue;
                 }
                 insert_pos += 1;
             }
             lines.insert(insert_pos, format!("* `{}`", entry));
        }

        fs::write(&pulse_path, lines.join("\n") + "\n")?;

        Ok(format!("Intent ingested successfully as {} into pulse.md", prefix))
    }

    pub fn execute_autonomous_evolution_cycle(workspace: &Path) -> EaiResult<String> {
        let res = crate::daemon::evolution::EvolutionManager::evolve_substrate(workspace)?;
        let _ = Self::execute_release(workspace)?;
        Ok(res)
    }

    pub fn run_lint(workspace: &Path) -> EaiResult<String> {
        let out = Command::new("cargo")
            .args(["clippy", "--all-targets", "--all-features"])
            .current_dir(workspace)
            .output()?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    pub fn run_audit(workspace: &Path) -> EaiResult<String> {
        let out = Command::new("cargo")
            .arg("audit")
            .current_dir(workspace)
            .output()?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_alignment_enforcement() {
        let workspace = Path::new(".");
        // This test ensures that the build fails if developer forgot to run 'susi admin sync'
        let result = SusiAdmin::verify_version_alignment(workspace);
        assert!(result.is_ok(), "Version mismatch detected between Cargo.toml and documentation. Run 'cargo run -- admin sync' to fix.");
    }
}
