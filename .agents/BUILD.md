# Project Instructions & Architecture

## System Information

* **Project Name**: `aeon`
* **Current Engine Version**: `v0.1.2022730`

## Project Mechanics & Deployment Workflow

0. **Clean Build Auto-Push**: Once aeon cleanly compiles (build success, 0 warnings, 0 errors, no functionality broken), automatically push to GitHub.
1. **Version Increment on Push**: Every push to GitHub must automatically increment the project version.
2. **Lightning Fast Compilation**: aeon must compile lightning fast through optimized build configurations, aggressive caching, and minimal overhead.
3. **Maximum Resource Utilization**: aeon is configured to utilize maximum available hardware resources (all available CPU threads, RAM, and parallel compilation jobs).
4. **Terminology & Component Sync on Push**: Every GitHub push must update .agents/BUILD.md, .agents/ASPIRATIONS.md, .agents/RUNTIME.md, .agents/TESTS.md, .agents/TOPOLOGY.md, and README.md with the latest terminology, architecture components, and current version.
5. **Natural Language Only**: aeon interactions with users are natural language only.
6. **100% Platform Independent**: aeon is 100% platform independent, self-contained, and cross-platform across Linux, macOS, Windows, WSL, and mobile architectures. 0 platform bias is the mandated execution state.
7. **Zero Configuration, Self-Tuning & Self-Healing**: aeon is 100% zero configuration, self-tuning, and self-healing. It automatically adapts, discovers local hardware and models, and self-heals runtime errors without requiring manual user setup. 0 config for the aeon user is the absolute mandate.
8. **Workspace Boundaries**: Project root (.) is the target workspace. Temporary runtime state is isolated inside local git-ignored directories (.aeon/ / ~/.aeon/).
9. **Automated Build & Test Harness**: Every GitHub push verifies clean compilation (cargo check) and unit/integration test suite pass (cargo test).
10. **Conventional Commit Format**: Git commit messages must use plain text conventional commit prefixes (e.g., feat:, fix:, refactor:, chore:, release:) without emojis.
11. **Dynamic Configuration Enforcement**: Zero hardcoded static configurations in code. All engine, server, port, model, and network parameters must be dynamic and loaded from configuration files (~/.aeon/config.json, ~/.aeon/env, ~/.aeon/mcp_config.json, ~/.aeon/global_mcp_registry.json) with automated dynamic defaults.
12. **Workspace Purity Enforcement**: The main workspace must remain free of temporary artifacts and test pollutants. All runtime tests must use isolated ephemeral directories or .aeon/.
13. **Full Compliance Enforcement on Push**: Before every GitHub push, the agent MUST apply all Agent Instructions and AEON Execution Rules to the entire codebase. This includes verifying version synchronization, auditing security patterns, enforcing workspace purity, and ensuring that no hardcoded simulations remain.
14. **Substrate Evolution Deployment**: Fulfilling a creator directive or architectural goal through the Motion Rule triggers the automated release sequence: (1) Clean Build; (2) Integration Test Pass; (3) Version Increment; (4) Compliance Audit; (5) GitHub Push.
15. **Runtime Substrate Preparation Rule**: The AeonRuntimeAgent autonomously prepares the optimal runtime environment. It links tools, provisions models, and tunes hardware for user intents while maintaining the immutability of the alpha-self core.
