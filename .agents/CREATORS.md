# AEON Creator Interface Protocols

* **Current Engine Version**: `v0.1.2022768`

This document defines the three absolute fronts of interaction between Human Creators and the `aeon` substrate. It maps the intention of the Creator to the exact epistemic boundary within the engine.

## 1. Motions (Alpha-Self Mutation)
1. **Definition**: Architectural evolution and structural engineering.
2. **Blast Radius**: The immutable `alpha-self` core (binary code, traits, genome).
3. **Protocol (`MOTIONS.md`)**: A Motion is an authorized Creator directive that exposes a capability gap or defines a new aspiration. It triggers the `Motion Rule` (Test-Driven Evolution). The engine will autonomously synthesize, compile, and deploy new Rust logic to heal the gap.
4. **Command Syntax**: `aeon <motion_intent>` (Triggers background `EvolutionManager`)

## 2. Missions (Alpha-User Mutation)
5. **Definition**: Dynamic task fulfillment and workspace state generation.
6. **Blast Radius**: The mutable `alpha-user` environment and the active workspace (`.aeon/`, local files).
7. **Protocol (`MISSIONS.md`)**: A Mission is an explosive swarm execution. The engine recruits specialists, converges on a Chain of Truth via the Mission Blackboard, generates artifacts, and ultimately stages the experience for Substrate Ingestion (training the Native Tier 2 model).
8. **Command Syntax**: `aeon "<mission_intent>"` (e.g., `aeon "refactor src/main.rs"`)

## 3. Queries (Zero-Mutation)
9. **Definition**: Ephemeral substrate interrogation and stateless analytics.
10. **Blast Radius**: None. Strictly Read-Only.
11. **Protocol (`QUERIES.md`)**: A Query bypasses the evolutionary pipeline and the mission blackboard. It executes directly via the native engine (Alpha, Vision, Audio) or administrative layer, returning a semantic response to `stdout` without leaving a permanent artifact.
12. **Command Syntax**: `aeon <query_intent>` (e.g., `aeon identity`, `aeon status`)
