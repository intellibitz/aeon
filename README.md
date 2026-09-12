# aeon: Exponential Intelligence for Any AI (EAI)

![AEON Version](https://img.shields.io/badge/version-v0.1.2022772-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)

**aeon** is a local-first, native Rust AI execution engine designed for high-throughput, hardware-saturated agent orchestration. It transforms static AI interactions into a dynamic, self-evolving intelligence substrate governed by a hard-compiled genome.

## The 5 Pillars of aeon

The substrate is organized into five functional pillars that ensure substrate purity and absolute model-agnosticism:

1.  **Agent of Agents (AoA)**: The coordination and governance layer (`GAWD / AMA`) that recruits and supervises parallel agent swarms.
2.  **Agents**: Functional execution units (Safety, Context, Specialist) synthesized dynamically to fulfill mission goals.
3.  **Engines**: Hardware-saturated inference substrates (`GEMI`, `Reflex`, `Vision`, `Audio`) providing low-latency compute logic.
4.  **Models**: Absolute model-agnosticism. Ingests any weights (GGUF, Safetensors, ONNX) from any global repository.
5.  **MCPs**: The interoperability bus (Model Context Protocol) connecting aeon to any external tool or data source.

## Architectural Innovations

### 1. Hard-Compiled Genome (Alpha-Self)
Unlike traditional AI frameworks, `aeon` hard-compiles its entire governance genome (`.agents/*.md`) into its binary. This ensures that the engine's core ethics, aspirations, and structural topology are immutable binary reflexes, not mutable prompts.

### 2. The Master Pulse (`pulse.md`)
The substrate operates via a singular, natural language verification loop. Every human instruction is automatically mapped to a verifiable `[MOTION]`, `[MISSION]`, or `[QUERY]` entry in [pulse.md](.agents/pulse.md). This anchors the substrate's entire lifecycle in a "Test-First" self-healing loop.

### 3. Axiomatic Pulse Ingestion
`aeon` possesses a native neural reflex to autonomously classify instructions and inject them into the genome. This eliminates the friction between human intent and machine execution, allowing the substrate to evolve at the speed of thought.

### 4. Substrate Ingestion Motion
`aeon` is a closed-loop intelligence system. It autonomously distills successful reasoning from its mutable interaction history into its own **Native Tier 2 Reasoning Model**, effectively migrating experience into hard-compiled logic.

### 5. Federated Parallelism
Agents run in parallel, coordinating via a shared, high-density **Mission Blackboard**. This allows for "Explosive Swarm Dispatch" where multiple specialists converge on a "Chain of Truth" simultaneously.

## Installation

### Universal One-Liner (Linux / macOS / Windows / WSL)
```bash
curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh
```

## Usage

Interact with the substrate using the unified `pulse` command:

```bash
# Core Evolution: Trigger a binary mutation
aeon pulse "Add a new spectral analysis engine to the binary"

# Workspace Mission: Execute a task with a swarm
aeon pulse "analyze this workspace and propose an optimization plan"

# Substrate Query: Verify semantic truth
aeon pulse "identity"

# Administration: Atomic genome synchronization
aeon admin sync

# Release: Execute full compliance audit and test suite
aeon admin release
```

## License

[Apache License 2.0](LICENSE)
