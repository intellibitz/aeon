# AEON Runtime Mandates

* **Current Engine Version**: `v0.1.2022750`

This document defines the operational directives for environment establishment, maintenance, and safety within the `alpha-user` space.

## 1. Environment Synthesis Mandates

1. **Substrate Priming**: Autonomously provision missing neural weights (`aeon-alpha.safetensors`, `aeon-reason.safetensors`) via the configured `alpha_weights_url`.
2. **Hardware Interrogation**: Deployment of the `AeonRuntimeAdmin` to continuously audit CPU/GPU topologies and system RAM, ensuring 100% hardware saturation and peak model selection.
3. **Protocol Linking**: Dynamically bind essential MCP servers (Database, Search, VCS) and registry-discovered tools.
4. **Zero-Config Guarantee**: Adapt to host environment variables and local constraints without manual user intervention.
5. **Self-Healing Reflex**: Automatically recover from runtime environment pathologies via protocol-based provisioning and hardware tuning.
6. **Registry Hot-Reload**: Validate agent registry integrity via timestamp-based audits to prevent stale behavior injection.

## 2. Swarm Operational Directives

7. **Neural Swarm Synthesis**: Recruit agents based on semantic centroid projections with a minimum recruitment threshold of 0.25.
8. **High-Density Context Mapping**: Utilize lease-capped, memory-safe context stores to prevent resource starvation during deep reasoning.
9. **Swarm Intelligence Escalation**: Use native 'reason' tools directly for absolute autonomy when complex logic is required.
10. **Mission Persistence**: Maintain stateful mission checkpoints (`mission_checkpoint.json`) to allow recovery from interrupted swarms.
11. **Epistemic Chain of Truth**: Ground every agent outcome in verified actions, neural context, and empirical filesystem state.

## 3. Governance & Safety Guardrails

12. **Destructive Command Guard**: Absolute prohibition on executing commands matching high-risk patterns (e.g., `rm -rf /`, `mkfs`, `shred`).
13. **Critical Path Protection**: Zero-tolerance for unauthorized access to system-critical paths like `/etc/shadow` or `/boot`.
14. **Secret Token Recognition**: Active detection and masking of sensitive credentials (e.g., `sk-`, `ghp_`, `AWS_SECRET_ACCESS_KEY`).
15. **Exfiltration Vector Defense**: Detect and intercept unauthorized data exfiltration attempts via network pipes or post-data tools.
16. **Autonomous Drift Detection**: The background daemon must periodically audit the substrate for capability gaps and trigger autonomous evolution cycles (Motion Rule) without user command.
