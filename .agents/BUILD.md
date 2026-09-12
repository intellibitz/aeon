# AEON Build & Deployment Substrate

* **Current Engine Version**: `v0.1.2022734`

This document defines the mechanics of the `aeon` binary lifecycle, release orchestration, and deployment mandates.

## 1. Core Project Mechanics

1. **100% Platform Independence**: Mandated native execution across Linux, macOS, Windows, and mobile architectures with zero platform bias.
2. **Lightning-Fast Compilation**: Optimization of build configurations and aggressive caching to minimize overhead.
3. **Maximum Resource Utilization**: Saturation of all available hardware resources (CPU threads, RAM, parallel jobs) during build and execution.
4. **Dynamic Configuration Mandate**: Zero hardcoded static configurations. All parameters must be loaded dynamically from `~/.aeon/` configuration files.
5. **Workspace Purity**: Absolute isolation of temporary artifacts and test pollutants within local git-ignored `.aeon/` directories.
6. **Natural Language Interface**: End-user interactions with the substrate are strictly natural language only.

## 2. Substrate Evolution Release Sequence

Fulfilling a creator directive or architectural goal triggers the automated **Motion Rule** sequence:

7. **Clean Build & Test Pass**: Mandatory verification of `cargo check` (0 errors/warnings) and 100% pass rate in the native test suite (`cargo test`).
8. **Compliance Audit**: Execution of `aeon admin audit` to verify security patterns, workspace purity, and genome alignment.
9. **Genome Synchronization**: Atomic version increment in `Cargo.toml` followed by a sync update to all `.agents/*.md` and `README.md` files.
10. **Conventional Commit Standard**: Git commit messages must use plain text conventional prefixes (e.g., `feat:`, `fix:`, `refactor:`, `chore:`, `release:`) without emojis.
11. **Automated GitHub Push**: Atomic push to the remote repository once all verification tiers are satisfied.

## 3. Universal Deployment Mandates

12. **One-Line Installation**: The only authorized installation method for all platforms is: `curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh`.
13. **Zero-Configuration Guarantee**: The engine must automatically adapt, discover local hardware/models, and self-heal without manual user setup.
