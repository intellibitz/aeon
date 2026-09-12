# Project Instructions & Architecture

## System Information

* **Project Name**: `aeon`
* **Current Engine Version**: `v0.1.2022727`

## Architectural Aspirations

aeon is a local-first, native Rust AI execution engine designed for high-throughput, low-latency agent orchestration. Its structural roadmap is driven entirely by clear, isolated architectural goals.

---

### [Aspiration 0] Substrate Purity & Meta-Only Mandate
* **Status**: `REALIZED`
* **Core Paradigm**: The entire compiled core codebase contains zero static domain-specific logic or rules. 100% meta and dynamic; all operational capabilities, specialist agent mappings, and semantic intent matrices are discovered and bound dynamically at runtime via local safetensors, system environment discovery, and Model Context Protocol (MCP) registries. 0 vendor bias and 0 platform bias—the substrate remains purely a protocol-driven execution layer.

---

### [Aspiration 1] Alpha-Self Core Awareness
* **Status**: `REALIZED`
* **Core Paradigm**: Immutable design governance rules (`AGENTS.md`) and structural layouts (`PROJECTS.md`) are swallowed whole and hard-compiled directly into strongly typed Rust structures (`AlphaSelf`). This completely eliminates expensive runtime string-parsing overhead and ensures the core protocol machinery maintains absolute, instantaneous alignment with the system genome.

---

### [Aspiration 2] Hardware-Saturated Inference Routing
* **Status**: `REALIZED`
* **Core Paradigm**: Active interrogation of CPU topologies, native system RAM, and GPU vRAM structures using a unified local Candle tensor framework. The engine dynamically maps its reasoning layer to a progressive model ladder based on the hosts live hardware profile, ensuring 100% GPU kernel offloading whenever compatible acceleration substrates are detected.

---

### [Aspiration 3] Industry-Standard MCP Interop Bus
* **Status**: `REALIZED`
* **Core Paradigm**: Native JSON-RPC 2.0 transport multiplexing active across both stdio and TCP network sockets over port 9090. The substrate acts as a fully compliant Model Context Protocol server and client proxy, enabling external AI applications, tool registries, and IDE layers to interoperate with local agent fleets instantly without configuration.

---

### [Aspiration 4] Autonomous Test-Driven Evolution (Motion Loop)
* **Status**: `REALIZED`
* **Core Paradigm**: The engine possesses self-healing substrate capabilities. When the core evolution managers encounter an authorized architectural mismatch or execution pathology, they autonomously trigger inner sub-processes that run cargo test, isolate compilation errors or logic failure signatures, and automatically synthesize production-ready Rust traits alongside companion unit test modules to close the gap.

---

### [Aspiration 5] High-Density Distributed Context Mapping
* **Status**: `REALIZED`
* **Core Paradigm**: Scale the meta-substrate to sustain high-density context tracking across massive asynchronous swarms, multi-agent blackboard states, and cluster-wide peer-to-peer network nodes. Implemented via lease-capped, memory-safe HighDensityContextStore primitives that prevent resource starvation and latency spikes during deep reasoning saturation.

---

### [Aspiration 6] Ultra-Latency Competitive Inference Racing
* **Status**: `REALIZED`
* **Core Paradigm**: Transformation of GEMI into a multi-path competitive substrate. Implements speculative parallel execution across local GPU kernels and cloud providers (via Power MCP), using a winner-takes-all protocol to deliver results at sub-10ms logic latency while maintaining a recursive "Chain of Verification" for absolute epistemic truth. Implemented in src/gemi/engine.rs via asynchronous thread racing and axiomatic alignment audits.

---

### [Aspiration 7] Universal Model Substrate Ingestion
* **Status**: `REALIZED`
* **Core Paradigm**: Transformation of AEON into a world-scale universal model ingestion engine. Enables absolute hardware-agnostic execution of any model from any web-based hub (Hugging Face, ModelScope, AEON CDN) regardless of weight, size, or native format (GGUF, Safetensors, ONNX, PyTorch). Implemented via hardware-aware speculative offloading and multi-format scanner substrates.

## Project Mechanics & Deployment Workflow

0. **Clean Build Auto-Push**: Once aeon cleanly compiles (build success, 0 warnings, 0 errors, no functionality broken), automatically push to GitHub.
1. **Version Increment on Push**: Every push to GitHub must automatically increment the project version.
2. **Lightning Fast Compilation**: aeon must compile lightning fast through optimized build configurations, aggressive caching, and minimal overhead.
3. **Maximum Resource Utilization**: aeon is configured to utilize maximum available hardware resources (all available CPU threads, RAM, and parallel compilation jobs).
4. **Terminology & Component Sync on Push**: Every GitHub push must update .agents/PROJECTS.md and README.md with the latest terminology, architecture components, and current version.
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
