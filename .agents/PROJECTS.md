# Project Instructions & Architecture

## System Information

* **Project Name**: `aeon`
* **Current Engine Version**: `v0.1.2022715`

## Architecture

`aeon` is a local-first, native Rust AI execution engine designed for high-throughput, low-latency agent orchestration.

### Engine Architecture

* **Alpha-Self**: Immutable system rules (`AGENTS.md`), and component topologies (`PROJECTS.md`) compiled directly into strongly-typed Rust data structures (`AlphaSelf`). Eliminates runtime string parsing and provides instantaneous self-awareness to all engine components.
* **Alpha-User**: Experiential memory and learned actions stored in local neural tensor weights (`aeon-alpha.safetensors`), trained continuously.
* **Local-Only Mandate**: Automatic execution strategy that utilizes strictly local runtimes (Candle tensor substrates and local model vaults). External cloud APIs are permanently excluded from the core reasoning loop to ensure absolute privacy, zero-latency performance, and hardware saturation.
* **Hardware-Aware Model Selection**: Interrogates system RAM, VRAM, and GPU acceleration capabilities to dynamically identify and default to the best suited local model for the host hardware profile.

### Unified Identity

* **Client**: MCP client for interacting with distributed local intelligence.
* **Server**: JSON-RPC and HTTP host for external system integration.
* **Host**: Managed execution environment for specialized agents and model runtimes.
* **CLI**: High-speed, < 2ms latency native command-line interface.

## Industry-Standard Model Context Protocol (MCP)

* **Protocol Support**: Strictly adheres to the MCP JSON-RPC 2.0 standard.
* **Tool Registry**: Native tools are exposed via a standard MCP-compliant registry, ensuring 100% compatibility with external AI ecosystems.
* **Client Implementation**: Can connect to and proxy any industry-standard MCP server (stdio or TCP).
* **Server Implementation**: Hosts capabilities over Port 9090, allowing any external system or IDE to interoperate with local agents as an MCP host.

## Components & Concurrency

* **Zero Setup**: All components run out-of-the-box with zero manual configuration.
* **Dynamic Intelligence**: Tools and agents are implemented via dynamic traits and thread-safe registries, ensuring architectural flexibility.
* **GPU Acceleration**: Inference engines are configured for 100% GPU offload (e.g., `-ngl 99` flag) whenever a CUDA, Metal, or Vulkan-compatible GPU is detected, utilizing the Candle Rust ML framework.
* **CPU Thread Saturation**: For non-tensor tasks, `aeon` saturates available physical CPU cores using parallel OS threads and async rtimes.
* **Sandboxed State Isolation**: All engine states, temporary build artifacts, and configuration settings are isolated inside `.aeon` sandbox containers, preventing host OS corruption.

## Process Architecture

* **AEON Launcher (`src/native/aeon`)**: A micro-binary (<3MB) written in native Rust that serves as the high-speed entry point. It spawns the background daemon and proxies MCP JSON-RPC streams to the persistent engine over local TCP (Port 9090). The resulting binary is named `aeon`.
* **AEON Engine (`.`)**: The heavy-throughput intelligence engine that manages swarm orchestration, inference routing, local model management, and the dynamic tool registry. The resulting binary is named `aeon-engine`.

## Security & Governance

* **Pre-Execution Governance**: Inspects tool execution signatures prior to execution, blocking destructive commands (`rm -rf /`, raw disk formatting) and credential/secret leaks (`AEON_API_KEY`, `OPENAI_API_KEY`, `AWS_SECRET_ACCESS_KEY`, private keys).
* **Audit Logging**: Logs every intent, governance check result, and tool invocation into `.aeon/audit.log`.
* **State Verification**: Verifies that claimed action artifacts (written files, build status) actually exist and match technical specifications before completing tasks.

## Substrate Evolution & Creator Protocols

21. **Native Intent Meta-Execution Protocol**: User intents are executed strictly via the meta-substrate (Models & Tools). If an intent fails, the agent **MUST** attempt "Meta-Substrate Self-Healing" (e.g., dynamic MCP discovery, tool re-routing). However, user intents **NEVER** trigger the Motion Rule to modify the native Rust substrate. Native evolution is an exclusive Creator power.
22. **Creator Aspiration Rule**: When provided with a creator directive or architectural goal, the agent focuses on future `aeon` capabilities and long-term vision. Only creator aspirations trigger the **Motion Rule**, advancing `aeon` evolution one step forward upon successful compilation, audit, and deployment.
23. **Motion Rule**: Triggered exclusively by Creator aspiration or directive: (1) Detect capability gap or architectural expansion requirement; (2) Synthesize production-ready native Rust code directly into the `aeon` core (`./src/`); (3) Compile and validate tests via `cargo test`; (4) Deploy via automated version increment and push.
24. **Creator Agent Mandate**: Creator agents strictly build and improve the `aeon` substrate. They must never perform the final work or simulate execution themselves. Their only objective is to architect a smarter `aeon` engine capable of fulfilling the intent autonomously.

## Project Mechanics & Deployment Workflow

0. **Clean Build Auto-Push**: Once `aeon` cleanly compiles (build success, 0 warnings, 0 errors, no functionality broken), automatically push to GitHub.
1. **Version Increment on Push**: Every push to GitHub must automatically increment the project version.
2. **Lightning Fast Compilation**: `aeon` must compile lightning fast through optimized build configurations, aggressive caching, and minimal overhead.
3. **Maximum Resource Utilization**: `aeon` is configured to utilize maximum available hardware resources (all available CPU threads, RAM, and parallel compilation jobs).
4. **Terminology & Component Sync on Push**: Every GitHub push must update `.agents/PROJECTS.md` and `README.md` with the latest terminology, architecture components, and current version.
5. **Natural Language Only**: `aeon` interactions with users are natural language only.
6. **100% Platform Independent**: `aeon` is 100% platform independent, self-contained, and cross-platform across Linux, macOS, Windows, WSL, and mobile architectures.
7. **Zero Configuration, Self-Tuning & Self-Healing**: `aeon` is 100% zero configuration, self-tuning, and self-healing. It automatically adapts, discovers local hardware and models, and self-heals runtime errors without requiring manual user setup.
8. **Workspace Boundaries**: Project root (`.`) is the target workspace. Temporary runtime state is isolated inside local git-ignored directories (`.aeon/` / `~/.aeon/`).
9. **Automated Build & Test Harness**: Every GitHub push verifies clean compilation (`cargo check`) and unit/integration test suite pass (`cargo test`).
10. **Conventional Commit Format**: Git commit messages must use plain text conventional commit prefixes (e.g., `feat:`, `fix:`, `refactor:`, `chore:`, `release:`) without emojis.
11. **Dynamic Configuration Enforcement**: Zero hardcoded static configurations in code. All engine, server, port, model, and network parameters must be dynamic and loaded from configuration files (`~/.aeon/config.json`, `~/.aeon/env`, `~/.aeon/mcp_config.json`, `~/.aeon/global_mcp_registry.json`) with automated dynamic defaults.
12. **Workspace Purity Enforcement**: The main workspace must remain free of temporary artifacts and test pollutants. All runtime tests must use isolated ephemeral directories or `.aeon/`.
13. **Full Compliance Enforcement on Push**: Before every GitHub push, the agent MUST apply all Agent Instructions and AEON Execution Rules to the entire codebase. This includes verifying version synchronization, auditing security patterns, enforcing workspace purity, and ensuring that no hardcoded simulations remain.
14. **Substrate Evolution Deployment**: Fulfilling a creator directive or architectural goal through the Motion Rule triggers the automated release sequence: (1) Clean Build; (2) Integration Test Pass; (3) Version Increment; (4) Compliance Audit; (5) GitHub Push.
