# AEON Build & Deployment Substrate

* **Current Engine Version**: `v0.1.2022813`

This document defines the mechanics of the `aeon` binary lifecycle, release orchestration, and deployment protocols.

## 1. Core Build Mechanics

1. **Lightning-Fast Compilation**: Optimization of build configurations and aggressive caching to minimize overhead and accelerate iteration.
2. **Maximum Resource Utilization**: Optimal saturation of hardware resources (CPU threads, RAM, parallel jobs) during the compilation cycle, with an absolute mandate to never exceed physical memory limits.
3. **Workspace Purity Enforcement**: Absolute isolation of build artifacts and test pollutants. All ephemeral state must be contained within git-ignored `.aeon/` directories.
4. **Dynamic Context Enforcement**: Zero hardcoded static configurations in source code. All engine and network parameters must be discoverable at runtime.

## 2. Substrate Evolution Release Sequence (The Motion Rule)

5. **Clean Build Verification**: Mandatory pass of `cargo check` with zero errors or warnings before any deployment.
6. **Native Test & Mission Verification**: Mandatory 100% pass rate across the native unit tests (`cargo test`) AND successful execution of ephemeral mission protocols (`aeon identity`, `aeon status`, `aeon models`).
7. **Release Gatekeeper**: Absolute mandate to execute `aeon admin release` to automatically enforce tests, mission verification, and the compliance audit prior to pushing.
8. **Compliance Audit**: Embedded within the release gatekeeper to verify security patterns and genome alignment.
9. **Genome Synchronization**: Atomic version increment in `Cargo.toml` followed by a sync update to all `.agents/*.md` and `README.md` files via `aeon admin sync`.
10. **Conventional Commit Protocol**: Git commit messages must use plain text conventional prefixes (e.g., `feat:`, `fix:`, `refactor:`) without emojis.
11. **Workspace De-pollution**: Absolute mandate to remove all non-essential temporary files, mission logs, and architectural scratch files from the root directory before any remote push.
12. **Automated Release Push**: Atomic push to the remote repository once all verification tiers and de-pollution mandates are satisfied.

## 3. Diagnostic & Evolution Tools

13. **Linting**: Mandatory use of `cargo clippy --all-targets --all-features` to ensure zero technical debt.
14. **Security Auditing**: Mandatory use of `cargo audit` to identify and mitigate dependency vulnerabilities.
15. **Fuzz Testing**: Use of `cargo fuzz run <target>` for deep stateful analytics and edge-case discovery.
16. **Async Debugging**: Use of `RUSTFLAGS="--cfg tokio_unstable" cargo run` with `tokio-console` for high-density asynchronous orchestration tracking.

## 4. Universal Deployment Protocols

17. **One-Line Installation**: The only authorized installation method for all platforms is: `curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh`.
18. **Binary Download vs. Build Fallback**: The installer must prioritize pre-compiled binary deployment for microsecond onboarding, with a transparent fallback to local compilation.
19. **Auto-Path Initialization**: Mandatory injection of `.aeon/bin` into the host's shell path environment (`.bashrc`, `.zshrc`, etc.) during installation.
