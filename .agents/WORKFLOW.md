# AEON Operational Workflow

* **Current Engine Version**: `v0.1.2022756`

This document defines the Meta-Workflow for substrate evolution and the Federated Parallelism logic of the active `aeon` engine.

## 1. The Creator Evolution Pipeline (Meta-Workflow)

1. **Vision Ingestion (`TESTS.md`)**: Creator vision is formalized strictly as a validation protocol (Test-Driven Evolution).
2. **Substrate Fork Decision**: A critical decision is made immediately following vision ingestion. Does the vision require mutation of the `alpha-self` core (binary code/architecture) or only the `alpha-user` environment (weights, tools, sandboxes)?

### 1a. Alpha-Self Mutation Path (Core Evolution)
3. **Architectural Mapping (`ASPIRATIONS.md`)**: The vision demands a new or refined architectural goal.
4. **Structural Definition (`TOPOLOGY.md`)**: The aspiration materializes as a specific component or pillar within the substrate topology.
5. **Operational Logic (`WORKFLOW.md`)**: The core component's interaction mechanics and parallel behaviors are defined.
6. **Core Synthesis (`BUILD.md`)**: The workflow triggers the build sequence directly to evolve the `alpha-self`.

### 1b. Alpha-User Mutation Path (Runtime Evolution)
7. **Runtime Mandate (`RUNTIME.md`)**: Because the vision does not mutate the core, `ASPIRATIONS.md` and `TOPOLOGY.md` are bypassed. The vision translates directly into operational mandates for the dynamic environment.
8. **Runtime Synthesis (`BUILD.md`)**: The updated runtime rules trigger a fast-path compilation, evolving the `alpha-user` state without altering the fundamental substrate architecture.

## 2. Phase A: Foundational Readiness (Continuous)

9. **Hardware Interrogation**: The `AeonRuntimeAdmin` continuously audits host CPU/GPU/RAM topologies to ensure the engine is primed for 100% compute saturation.
10. **Substrate Optimization**: The `AeonRuntimeAdmin` provisions the optimal model ladder step and locks the engine to the peak performing local weights.
11. **Daemon Persistence**: The `AmaDaemon` sustains the GMCP/GEMI/UDP server fleet, maintaining a stateful protocol bridge for all internal and external requests.

## 3. Phase B: Swarm Synthesis (On Intent)

12. **Genome Interrogation**: Upon receiving a natural language intent, the `GAWD / AMA` orchestrator interrogates the hard-compiled binary genome for recruitment rules.
13. **Semantic Recruitment**: The substrate recruits a mission-specific fleet (Safety, Context, Specialists) using Tier 0 semantic centroid projections.
14. **Axiomatic Auditing**: The `AeonAdmin` audits the synthesized swarm to ensure it adheres to the **Epistemic Integrity Mandates** before execution begins.

## 4. Phase C: Execution & Distillation (Mission Cycle)

15. **Explosive Swarm Dispatch**: Parallel execution of agents across isolated threads, coordinating via a shared, high-density **Mission Blackboard**.
16. **Chain of Truth Convergence**: Swarm participants converge on a verified outcome, grounding all results in empirical filesystem state and tool results.
17. **Substrate Ingestion**: Successful reasoning is staged and distilled into the **Native Tier 2 Reasoning Model** to close the loop between experience and memory.
18. **Autonomous Drift Correction**: The `EvolutionManager` audits the mission logs for capability gaps and triggers autonomous synthesis to heal the substrate.
