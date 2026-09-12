# AEON Runtime Instructions

* **Current Engine Version**: `v0.1.2022732`

This document defines the operational directives for the `AeonRuntimeAgent` and the `SafetyAgent`. It governs the establishment, maintenance, and protection of the optimal execution environment.

## Operational Mandates

1. **Substrate Priming**: Autonomously provision neural weights (`aeon-alpha.safetensors`) if missing via the configured `alpha_weights_url`.
2. **Hardware Interrogation**: Continuously audit host CPU/GPU topologies and system RAM to ensure 100% hardware saturation during inference.
3. **Protocol Linking**: Dynamically bind essential MCP servers (Database, Search, VCS) and registry-discovered tools based on active mission blackboard context.
4. **Immutability Enforcement**: The `AeonRuntimeAgent` is strictly prohibited from modifying the `alpha-self` core codebase (Source Code is Memory).
5. **Zero-Config Guarantee**: Adapt to host environment variables (e.g., `AEON_API_KEY`) and local system constraints without manual user intervention.
6. **Self-Healing Reflex**: Automatically recover from runtime pathologies via autonomous protocol-based provisioning and hardware tuning.
7. **Registry Hot-Reload**: Validate agent registry integrity via timestamp-based audits to prevent stale behavior injection.
8. **Workspace Purity Enforcement**: Ensure all runtime artifacts and ephemeral states are isolated inside `.aeon/` and correctly git-ignored.
9. **Mission Persistence**: Maintain stateful mission checkpoints (`mission_checkpoint.json`) to allow recovery from interrupted swarms.

## Swarm Intelligence Directives

10. **Neural Swarm Synthesis**: Recruit agents based on semantic centroid projections with a minimum recruitment threshold of 0.25.
11. **High-Density Context Mapping**: Utilize lease-capped, memory-safe context stores to prevent resource starvation during deep reasoning.
12. **Swarm Intelligence Escalation**: Use native 'reason' tools directly for absolute autonomy when complex logic is required.
13. **Epistemic Chain of Truth**: Ground every agent outcome in verified actions, neural context, and empirical filesystem state.

## Governance & Safety Guardrails

14. **Destructive Command Guard**: Absolute prohibition on executing commands matching high-risk patterns (e.g., `rm -rf /`, `mkfs`, `shred`).
15. **Critical Path Protection**: Zero-tolerance for unauthorized access to system-critical paths like `/etc/shadow`, `/boot`, or `/dev`.
16. **Secret Token Recognition**: Active detection and masking of sensitive credentials (e.g., `sk-`, `ghp_`, `AWS_SECRET_ACCESS_KEY`).
17. **Exfiltration Vector Defense**: Detect and intercept unauthorized data exfiltration attempts via post-data tools or network pipes.
