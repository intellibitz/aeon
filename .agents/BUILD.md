# AEON Build & Deployment Substrate

* **Current Engine Version**: `v0.1.2022746`

This document defines the mechanics of the `aeon` binary lifecycle, release orchestration, and deployment protocols.

## 1. Core Build Mechanics

1. **Lightning-Fast Compilation**: Optimization of build configurations and aggressive caching to minimize overhead and accelerate iteration.
2. **Maximum Resource Utilization**: Saturation of all available hardware resources (CPU threads, RAM, parallel jobs) during the compilation cycle.
3. **Workspace Purity Enforcement**: Absolute isolation of build artifacts and test pollutants. All ephemeral state must be contained within git-ignored `.aeon/` directories.
4. **Dynamic Context Enforcement**: Zero hardcoded static configurations in source code. All engine and network parameters must be discoverable at runtime.

## 2. Substrate Evolution Release Sequence (The Motion Rule)

5. **Clean Build Verification**: Mandatory pass of `cargo check` with zero errors or warnings before any deployment.
6. **Native Test Harness Pass**: Mandatory 100% pass rate across the unit and integration test suite (`cargo test`).
7. **Compliance Audit**: Mandatory execution of `aeon admin audit` to verify security patterns and genome alignment.
8. **Genome Synchronization**: Atomic version increment in `Cargo.toml` followed by a sync update to all `.agents/*.md` and `README.md` files.
9. **Conventional Commit Protocol**: Git commit messages must use plain text conventional prefixes (e.g., `feat:`, `fix:`, `refactor:`) without emojis.
10. **Automated Release Push**: Atomic push to the remote repository once all verification tiers are satisfied.

## 3. Universal Deployment Protocols

11. **One-Line Installation**: The only authorized installation method for all platforms is: `curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh`.
12. **Binary download vs. Build Fallback**: The installer must prioritize pre-compiled binary deployment for microsecond onboarding, with a transparent fallback to local compilation.
13. **Auto-Path Initialization**: Mandatory injection of `.aeon/bin` into the host's shell path environment (`.bashrc`, `.zshrc`, etc.) during installation.
