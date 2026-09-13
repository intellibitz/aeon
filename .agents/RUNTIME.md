# AEON Runtime Mandates

* **Current Engine Version**: `v0.1.2022793`

This document defines the operational directives for environment establishment, maintenance, and safety across both the `alpha-self` (core host) and `alpha-user` (mutable workspace) boundaries.

## 1. Alpha-Self Host Mandates (Foundational Readiness)

1. **Daemon Persistence**: The `AmaDaemon` must sustain a continuous, resilient background process, binding standard MCP interop ports (9090, 9091, 9093) and A2A discovery ports (9092).
2. **Hardware Interrogation**: The `AeonRuntimeAdmin` must continuously audit CPU/GPU topologies and system RAM to guarantee 100% compute saturation for the execution core.
3. **Autonomous Drift Detection**: The substrate must periodically audit itself for capability gaps and trigger the *Motion Rule* (autonomous evolution cycles) without user command.
4. **Self-Healing Reflex**: The engine must autonomously recover from structural pathologies, port collisions, or memory faults via protocol-based provisioning and hardware re-tuning.

## 2. Alpha-User Environment Synthesis (Mutable State)

5. **Substrate Priming**: Autonomously provision missing neural weights (`aeon-alpha.safetensors`, `aeon-reason.safetensors`) into the `.aeon/models/` vault.
6. **Protocol Linking**: Dynamically bind essential MCP servers (Database, Search, VCS) and registry-discovered external tools to the active workspace.
7. **Zero-Config Guarantee**: Adapt instantly to workspace-specific environment variables (e.g., `AEON_API_KEY`) and local system constraints without manual user intervention.
8. **Registry Hot-Reload**: Validate the dynamic agent registry (`agent_registry.json`) via timestamp-based audits to prevent stale behavior injection during swarm synthesis.

## 3. Swarm Operational Directives

9. **Neural Swarm Synthesis**: Recruit agents based on semantic centroid projections with a minimum recruitment threshold of 0.25.
10. **High-Density Context Mapping**: Utilize lease-capped, memory-safe context stores to prevent resource starvation during deep reasoning.
11. **Swarm Intelligence Escalation**: Use native 'reason' tools directly for absolute autonomy when complex logic is required.
12. **Mission Persistence**: Maintain stateful mission checkpoints (`mission_checkpoint.json`) to allow recovery from interrupted swarms.
13. **Epistemic Chain of Truth**: Ground every agent outcome in verified actions, neural context, and empirical filesystem state.

## 4. Governance & Safety Guardrails

14. **Destructive Command Guard**: Absolute prohibition on executing commands matching high-risk patterns (e.g., `rm -rf /`, `mkfs`, `shred`).
15. **Critical Path Protection**: Zero-tolerance for unauthorized access to system-critical paths like `/etc/shadow` or `/boot`.
16. **Secret Token Recognition**: Active detection and masking of sensitive credentials (e.g., `sk-`, `ghp_`, `AWS_SECRET_ACCESS_KEY`).
17. **Exfiltration Vector Defense**: Detect and intercept unauthorized data exfiltration attempts via network pipes or post-data tools.
