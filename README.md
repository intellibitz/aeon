# aeon: Exponential Intelligence for Any AI (EAI)

![AEON Version](https://img.shields.io/badge/version-v0.1.2022764-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)

**aeon** is a local-first, native Rust AI execution engine designed for high-throughput, hardware-saturated agent orchestration. It transforms static AI interactions into a dynamic, self-evolving intelligence substrate.

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

### 2. Substrate Ingestion Motion
`aeon` is a closed-loop intelligence system. It autonomously distills successful reasoning from its mutable interaction history into its own **Native Tier 2 Reasoning Model**, effectively migrating experience into hard-compiled logic.

### 3. Federated Parallelism
Agents run in parallel, coordinating via a shared, high-density **Mission Blackboard**. This allows for "Explosive Swarm Dispatch" where multiple specialists converge on a "Chain of Truth" simultaneously.

### 4. Autonomous Drift Detection
The substrate independently audits its own capability surface. When architectural gaps are detected, the **Motion Rule** triggers autonomous synthesis and testing of native Rust traits to heal and expand the engine.

## Installation

### Universal One-Liner (Linux / macOS / Windows / WSL)
```bash
curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh
```

## Usage

Interact with the substrate using natural language or system commands:

```bash
# Identity Report: Inspect the hard-compiled genome
aeon identity

# Native Logic: Execute a mission with an explosive swarm
aeon "analyze this workspace and propose an optimization plan"

# Multimodal: Unified vision/text analysis
aeon "analyze visual/image.png and describe its intent"

# Administration: Atomic genome synchronization
aeon admin sync
```

## License

[Apache License 2.0](LICENSE)
