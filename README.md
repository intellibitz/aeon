# aeon: Exponential Intelligence for Any AI (EAI)

![AEON Version](https://img.shields.io/badge/version-v0.1.2022818-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)

**aeon** is a local-first, native Rust AI execution engine designed for high-throughput, hardware-saturated agent orchestration. It transforms static AI interactions into a dynamic, self-evolving intelligence substrate governed by a hard-compiled genome. By embedding design governance rules directly into binary memory, `aeon` eliminates the gap between intention and execution, delivering a safe, sovereign, and exponentially improving intelligence layer for any environment.

## Installation

### Universal One-Liner (Linux / macOS / Windows / WSL)
```bash
curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh
```
The installer prioritizes pre-compiled binary deployment for microsecond onboarding, with a transparent fallback to local compilation if required. It automatically initializes the `.aeon/bin` environment in your host's shell path.

## Usage

Interact with the substrate using the simplified natural language interface:

```bash
# Core Evolution: Trigger a binary mutation (Creator Mode)
aeon "Add a new spectral analysis engine to the binary"

# Workspace Mission: Execute a task with a swarm
aeon "analyze this workspace and propose an optimization plan"

# Substrate Query: Verify semantic truth
aeon "identity"

# Optional: The 'pulse' keyword is preserved for explicit ingestion
aeon pulse "Sync genome version"

# Administration: Atomic genome synchronization
aeon admin sync

# Release: Execute full compliance audit and test suite
aeon admin release
```

---

# Complete Documentation

This section provides a detailed synthesis of the AEON Substrate Genome, as defined in the hard-compiled `.agents/` manifest.

## 1. Universal Agent Governance
AEON is governed by a set of **Epistemic Integrity Mandates** that ensure absolute truth and professional excellence:

- **No Lies & No Hallucinations**: Every status report, code snippet, and fact must be grounded in verified reality or direct tool results.
- **Brutally Honest & Critical**: The engine maintains a continuous evaluation mode, critically auditing architecture and logic for drift or pathologies.
- **Epistemic Chain of Truth**: Source code and empirical runtime results are the ultimate truth.
- **Immutability Enforcement**: The core `alpha-self` binary is protected from modification by runtime agents; evolution is only permitted through the authorized pipeline.
- **Substrate Sovereignty**: Absolute isolation of ephemeral state within git-ignored `.aeon/` directories.

## 2. Architectural Aspirations
AEON's evolution is driven by core genomic paradigms, including:

- **Platform Independence**: Native execution across Linux, macOS, and Windows with zero bias.
- **Lock-Free Native Substrate**: Elimination of blocking thread locks across critical data structures (KV stores, registries, blackboards) using non-blocking primitives and `RwLock` concurrency.
- **<2ms Ultra-Reflex Substrate**: High-resolution latency guards enforcing sub-2ms intent classification and reflex response times.
- **Recursive Swarm Parallelism**: Decomposing complex missions into independent sub-tasks via parallel split-solve-join execution.
- **Substrate Purity**: The core codebase contains zero static domain-specific logic; all capabilities are bound dynamically via MCP.
- **Hardware Saturation**: Active interrogation of CPU/GPU topologies to dynamically map reasoning to the optimal acceleration substrate.
- **Autonomous Test-Driven Evolution**: The substrate independently detects architectural gaps and triggers self-healing Rust mutations.
- **Unified Multi-Modal Embedding Space**: A 1024-dimensional neural projection space where text, vision, and audio intents are unified.
- **Federated Experience Aggregation**: Secure, privacy-preserving protocol for aggregating distilled reasoning from world users to evolve the global genome.

## 3. Substrate Topology (The 5 Pillars)
The engine is structured into five functional tiers:

1.  **Agent of Agents (AoA)**: Universal swarm supervisor (`GAWD`) supporting parallel Fork-Join partitioning and persistent daemon control (`AmaDaemon`).
2.  **Agents**: Specialist units (Safety, Context, Hardware, Runtime) synthesized dynamically based on intent.
3.  **Engines**: Lock-free, hardware-saturated substrates for Alpha (Reflex), GEMI (Reasoning), Vision, and Audio.
4.  **Models**: Local neural weights (`aeon-alpha.safetensors`, `aeon-reason.safetensors`) and universal model ingestion (GGUF, safetensors, ONNX).
5.  **MCPs**: JSON-RPC 2.0 interoperability bus connecting to any tool registry or external data source.

## 4. Operational Workflow
AEON follows a recursive **Creator Evolution Pipeline**:

1.  **Vision Ingestion**: Transformation of natural language into typed `[MOTION]`, `[MISSION]`, or `[QUERY]` pulse entries with an automated **Deduplication Guard** guaranteeing unique genome entries.
2.  **Substrate Fork Decision**: Causal routing to the appropriate mutation path (Core, Workspace, or Ephemeral).
3.  **Explosive Swarm Dispatch**: Parallel execution of agents coordinating via a shared, lock-free **Mission Blackboard**.
4.  **Chain of Truth Convergence**: Swarm participants grounding all results in empirical filesystem state.
5.  **Substrate Ingestion**: Distillation of reasoning into the Native Tier 2 model to close the loop between experience and memory.

## 5. Interaction Fronts (The Sovereign Boundary)
- **Motions (Alpha-Self)**: **Creator-Only**. Architectural evolution requiring the genome source code and Rust compiler.
- **Missions (Alpha-User)**: **Universal**. Dynamic task fulfillment and workspace mutation via Experience Distillation.
- **Queries (Zero-Mutation)**: **Universal**. Stateless substrate interrogation and truth auditing.

## 6. Build & Release Protocols
Release integrity is enforced by the **AeonAdmin** administrative substrate:
- **Mandatory Verification**: 100% pass rate in native unit tests (`cargo test`) and ephemeral mission protocols (`aeon identity`).
- **Workspace De-pollution**: Absolute mandate to remove all temporary files and logs before remote push.
- **Genome Synchronization**: Atomic version increment and sync across all manifests and binary constants.

## 7. Runtime Mandates & Safety
- **Lock-Free Concurrency**: Zero execution stall across multi-threaded swarm tasks.
- **Self-Healing Reflex**: Autonomous recovery from port collisions or memory faults.
- **Swarm Safety**: Absolute intercept of destructive patterns (e.g., `rm -rf /`) and critical path protection.
- **Secret Masking**: Active detection and masking of sensitive credentials (tokens, keys) in all output streams.

## 8. Validation Genome (The Master Pulse)
The `pulse.md` file serves as the singular source of truth for realized and pending evolution. Every capability of the active binary is anchored in a resolved, unique pulse entry, ensuring that the engine always accurately reflects its hard-compiled genome.

---

## License
[Apache License 2.0](LICENSE)
