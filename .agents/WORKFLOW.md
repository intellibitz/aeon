# AEON Operational Workflow

* **Current Engine Version**: `v0.1.2022755`

This document defines the Meta-Workflow for substrate evolution and the Federated Parallelism logic of the active `aeon` engine.

## 1. The Creator Evolution Pipeline (Meta-Workflow)

1. **Vision Ingestion (`TESTS.md`)**: Creator vision is formalized strictly as a validation protocol (Test-Driven Evolution).
2. **Architectural Mapping (`ASPIRATIONS.md`)**: The test protocol dictates a new or refined architectural goal.
3. **Structural Definition (`TOPOLOGY.md`)**: The aspiration materializes as a specific component or pillar within the substrate topology.
4. **Operational Logic (`WORKFLOW.md`)**: The component's interaction mechanics and parallel behaviors are defined.
5. **Alpha-Self Fork (`BUILD.md`)**: If the operational logic mutates the immutable core (compiled Rust traits, engine mechanics), the workflow triggers the build sequence directly to evolve the `alpha-self`.
6. **Alpha-User Fork (`RUNTIME.md`)**: If the logic mutates the dynamic environment (weights, tools, sandboxes), the workflow dictates runtime mandates before triggering the build sequence to evolve the `alpha-user`.

## 2. Phase A: Foundational Readiness (Continuous)

7. **Hardware Interrogation**: The `AeonRuntimeAdmin` continuously audits host CPU/GPU/RAM topologies to ensure the engine is primed for 100% compute saturation.
8. **Substrate Optimization**: The `AeonRuntimeAdmin` provisions the optimal model ladder step and locks the engine to the peak performing local weights.
9. **Daemon Persistence**: The `AmaDaemon` sustains the GMCP/GEMI/UDP server fleet, maintaining a stateful protocol bridge for all internal and external requests.

## 3. Phase B: Swarm Synthesis (On Intent)

10. **Genome Interrogation**: Upon receiving a natural language intent, the `GAWD / AMA` orchestrator interrogates the hard-compiled binary genome for recruitment rules.
11. **Semantic Recruitment**: The substrate recruits a mission-specific fleet (Safety, Context, Specialists) using Tier 0 semantic centroid projections.
12. **Axiomatic Auditing**: The `AeonAdmin` audits the synthesized swarm to ensure it adheres to the **Epistemic Integrity Mandates** before execution begins.

## 4. Phase C: Execution & Distillation (Mission Cycle)

13. **Explosive Swarm Dispatch**: Parallel execution of agents across isolated threads, coordinating via a shared, high-density **Mission Blackboard**.
14. **Chain of Truth Convergence**: Swarm participants converge on a verified outcome, grounding all results in empirical filesystem state and tool results.
15. **Substrate Ingestion**: Successful reasoning is staged and distilled into the **Native Tier 2 Reasoning Model** to close the loop between experience and memory.
16. **Autonomous Drift Correction**: The `EvolutionManager` audits the mission logs for capability gaps and triggers autonomous synthesis to heal the substrate.
