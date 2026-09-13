# AEON Runtime Mandates

* **Current Engine Version**: `v0.1.2022820`

This document defines the operational directives for environment establishment, maintenance, and safety across both the `alpha-self` (core host) and `alpha-user` (mutable workspace) boundaries.

## 1. Alpha-Self Host Mandates (Foundational Readiness)

1. **Daemon Persistence**: The `AmaDaemon` must sustain a continuous, resilient background process, binding standard MCP interop ports (9090, 9091, 9093) and A2A discovery ports (9092).
2. **Hardware Interrogation**: The `AeonRuntimeAdmin` must continuously audit CPU/GPU topologies and system RAM to guarantee optimal compute saturation (max 90% utilization) and proactively prevent Out-of-Memory (OOM) events.
3. **Autonomous Drift Detection**: The substrate must periodically audit itself for capability gaps and trigger the *Motion Rule* (autonomous evolution cycles) without user command.
4. **Self-Healing Reflex**: The engine must autonomously recover from structural pathologies, port collisions, or memory faults via protocol-based provisioning and hardware re-tuning.

## 2. Alpha-User Environment Synthesis (Mutable State)

5. **Substrate Priming**: Autonomously provision the highest-ranking open-source neural weights compatible with host hardware profiling (e.g., Llama, Gemma, Mistral) into the `.aeon/models/` vault. Fixed native weight identifiers are deprecated in favor of dynamic performance-based selection.
6. **Protocol Linking**: Dynamically bind essential MCP servers (Database, Search, VCS) and registry-discovered external tools to the active workspace.
7. **Zero-Config Guarantee**: Adapt instantly to workspace-specific environment variables (e.g., `AEON_API_KEY`) and local system constraints without manual user intervention.
8. **Fluid Scaling**: System boundaries (parsing limits, context depth, execution timeouts) must scale fluidly based on hardware availability. Hardware is the only limit; architectural gates are prohibited.
9. **Registry Hot-Reload**: Validate the dynamic agent registry (`agent_registry.json`) via timestamp-based audits to prevent stale behavior injection during swarm synthesis.
10. **Lock-Free Mapping**: High-density context mapping and blackboard access must be non-blocking. The engine must utilize lock-free stores to prevent agent wait-states during deep reasoning missions.
11. **Sub-2ms Reflex**: 100% of non-inference engine operations must execute in <2ms. Implement high-resolution timing guards on critical paths to enforce the instant-intelligence mandate.

## 3. Swarm Operational Directives

12. **Neural Swarm Synthesis**: Recruit agents based on semantic centroid projections with a minimum recruitment threshold of 0.25.
13. **High-Density Context Mapping**: Utilize lease-capped, memory-safe context stores to prevent resource starvation during deep reasoning.
14. **Swarm Intelligence Escalation**: Use native 'reason' tools directly for absolute autonomy when complex logic is required.
15. **Mission Persistence**: Maintain stateful mission checkpoints (`mission_checkpoint.json`) to allow recovery from interrupted swarms.
16. **Epistemic Chain of Truth**: Ground every agent outcome in verified actions, neural context, and empirical filesystem state.

## 4. Governance & Safety Guardrails

17. **Destructive Command Guard**: Absolute prohibition on executing commands matching high-risk patterns (e.g., `rm -rf /`, `mkfs`, `shred`).
18. **Critical Path Protection**: Zero-tolerance for unauthorized access to system-critical paths like `/etc/shadow` or `/boot`.
19. **Secret Token Recognition**: Active detection and masking of sensitive credentials (e.g., `sk-`, `ghp_`, `AWS_SECRET_ACCESS_KEY`).
20. **Exfiltration Vector Defense**: Detect and intercept unauthorized data exfiltration attempts via network pipes or post-data tools.
