# AEON Operational Workflow

* **Current Engine Version**: `v0.1.2022839`

This document defines the Meta-Workflow for substrate evolution and the Federated Parallelism logic of the active `aeon` engine.

## 1. The Creator Evolution Pipeline (Meta-Workflow)

1. **Pulse Ingestion**: AEON accepts natural language intents (Pulses) via a non-blocking asynchronous listener.
2. **Pulse Queueing**: Every pulse is injected into the `SubstratePulseQueue`. If the engine is active, the pulse is staged; otherwise, it is immediately promoted to the execution context.
3. **Vision Ingestion (`pulse.md`)**: Upon promotion, the pulse is formalized as a typed test entry:
    - `[MOTION]`: Triggers `alpha-self` core mutation (Rust/Binary).
    - `[MISSION]`: Triggers `alpha-user` workspace mutation (Files/Distillation).
    - `[QUERY]`: Triggers ephemeral truth verification (Stateless Analytics).
4. **Substrate Fork Decision**: The `GAWD` orchestrator identifies the type prefix and routes the vision to the appropriate evolutionary path.

### 1a. Alpha-Self Mutation Path (Core Evolution)
3. **Architectural Mapping (`ASPIRATIONS.md`)**: The vision demands a new or refined architectural goal.
4. **Structural Definition (`TOPOLOGY.md`)**: The aspiration materializes as a specific component or pillar within the substrate topology.
5. **Operational Logic (`WORKFLOW.md`)**: The core component's interaction mechanics and parallel behaviors are defined.
6. **Core Synthesis (`BUILD.md`)**: The workflow triggers the build sequence directly to evolve the `alpha-self`.

### 1b. Alpha-User Mutation Path (Mission & Runtime Evolution)
7. **Runtime Mandate (`RUNTIME.md`)**: The vision alters the baseline environment (hardware settings, model defaults) without mutating core traits.
8. **Mission Protocol (`MISSIONS.md`)**: The vision demands new capabilities for workspace manipulation, artifact generation, or experience staging (modifying the `.aeon/` mutable state).
9. **State Synthesis**: The updated rules trigger a fast-path compilation, evolving the `alpha-user` operational boundaries.

### 1c. Ephemeral Execution Path (Zero-Mutation)
10. **Query Protocol (`QUERIES.md`)**: Defined as stateless queries.
11. **Swarm Dispatch**: The query is routed to the multi-threaded swarm for immediate fulfillment via the Mission Blackboard.

## 2. Universal Swarm Execution (Mandatory)

12. **The Swarm Mandate**: Every operation across all tiers (Tier 0 Reflex, Tier 1 Swarm, Tier 2 Reasoning)—Motions (1a), Missions (1b), Queries (1c), GEMI reasoning, and GMCP tool operations—must initialize a specialized GAWD Swarm at maximum hardware capacity. Direct serial execution of engine logic is constitutionally prohibited across all tiers.
13. **Phase A: Foundational Readiness**: The `AeonRuntimeAdmin` continuously audits host CPU/GPU/RAM topologies and provisions the optimal model ladder via the swarm.
14. **Substrate Optimization**: The `AeonRuntimeAdmin` provisions the optimal model ladder step and locks the engine to the peak performing local weights.
15. **Daemon Persistence**: The `AmaDaemon` sustains the GMCP/GEMI/UDP server fleet, maintaining a stateful protocol bridge for all internal and external requests.

## 3. Phase B: Swarm Synthesis (On Intent)

16. **Genome Interrogation**: Upon receiving a natural language intent, the `GAWD / AMA` orchestrator interrogates the hard-compiled binary genome for recruitment rules.
17. **Semantic Recruitment**: The substrate recruits a mission-specific fleet (Safety, Context, Specialists) using Tier 0 semantic centroid projections.
18. **Axiomatic Auditing**: The `AeonAdmin` audits the synthesized swarm to ensure it adheres to the **Epistemic Integrity Mandates** before execution begins.

## 4. Phase C: Execution & Distillation (Mission Cycle)

19. **Explosive Swarm Dispatch**: Parallel execution of agents across isolated threads, coordinating via a shared, high-density **Mission Blackboard**.
20. **Recursive Fork-Join**: For complex missions, the orchestrator triggers a recursive "Split-Parallel-Join" cycle. The goal is partitioned into independent sub-missions, processed by sub-swarms, and re-joined upon semantic convergence.
21. **Chain of Truth Convergence**: Swarm participants converge on a verified outcome, grounding all results in empirical filesystem state and tool results.
22. **Substrate Ingestion**: Successful reasoning is staged and distilled into the **Native Tier 2 Reasoning Model** to close the loop between experience and memory.
23. **Autonomous Drift Correction**: The `EvolutionManager` audits the mission logs for capability gaps and triggers autonomous synthesis to heal the substrate.
24. **Decoupled Swarm Messaging Mechanics**: High-throughput communication between swarm agents operates via Reactor/Proactor event loops, lock-free work-stealing queues, and zero-copy ring buffers, enforcing credit-based backpressure and sub-2ms response convergence.

## 5. Phase D: Omni-Trace Thinking Synthesis (The Universal Trace)

25. **Omni-Trace Accumulation**: Throughout the swarm execution, every atomic event (tool call, agent state change, decision fork, model inference trace) is captured in a high-density, non-blocking telemetry buffer.
26. **Thinking Synthesis**: Upon mission completion, the `GAWD` orchestrator re-synthesizes the telemetry buffer into a structured natural language "Thinking" block. This block must include the raw model reasoning, logs, warnings, and error recovery paths.
27. **Streaming Trace Protocol**: AEON must stream the model's internal thinking and raw output tokens directly into the active thinking block as they are generated. This real-time trace ensures zero-latency transparency for Creators and Users.
28. **Result Presentation Gate**: The user-facing result is gated behind the thinking process. Results are only displayed once the model stream terminates and the substrate verifies the epistemic chain of truth.
29. **Evolutionary Feedback Loop**: The Omni-Trace is exposed to the user and Creator as the primary mechanism for substrate evolution. It serves as the definitive record for identifying architectural gaps and capability drift.

## 6. Phase E: Continuous Pulse Cycle (Fluid Interaction)

30. **Pulse Transition**: Upon completion of a pulse (Result Displayed), the `GAWD` orchestrator immediately interrogates the `SubstratePulseQueue` for the next entry.
31. **Recursive Correction**: If a new pulse is received during execution, it is evaluated for "Correction Priority". Corrective pulses (e.g., "stop", "change direction") can preempt or modify the active mission blackboard state.
32. **Unified Session Telemetry**: All pulses in a single interaction session are linked in a unified telemetry stream, allowing for cross-pulse reasoning and context persistence.
