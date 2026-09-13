# AEON Substrate Topology

* **Current Engine Version**: `v0.1.2022826`

This document defines the structural native components and orchestrated meta-layers of the `aeon` substrate, organized into five functional pillars.

## 1. Agent of Agents (AoA) - Coordination & Governance
1. **GAWD / AMA**: Universal swarm supervisor and multi-agent parallel dispatcher. (Tier: 1)
2. **AeonAdmin**: Native administrative substrate for release orchestration, compliance auditing, and axiomatic pulse ingestion. (Tier: 1)
3. **AeonRuntimeAdmin**: Substrate maintenance authority (Hardware audit, Model provisioning & peak selection). (Tier: 1)
4. **EvolutionManager**: Substrate self-healing and autonomous Motion Rule execution. (Tier: 1)
5. **AmaDaemon**: Persistent background host and process manager for the GMCP/GEMI server fleet. (Tier: 1)

## 2. Agents - Functional Execution Units
6. **AeonRuntimeAgent**: Autonomous environment preparation agent (Weights & Tools). (Tier: 1)
7. **HardwareAgent**: Autonomous hardware interrogation and compute resource saturation agent. (Tier: 1)
8. **SafetyAgent**: Governance auditor and destructive command interceptor. (Tier: 1)
9. **ContextAgent**: High-density context manager and workspace analyzer. (Tier: 1)
10. **NeuralAgentFactory**: Autonomous synthesis and recruitment of domain-specific specialist agents. (Tier: 1)

## 3. Engines - Execution & Inference Substrates
11. **AEON-Alpha**: Microsecond intent classification and deterministic neural reflex engine. (Tier: 0)
12. **ReflexSynthesizer**: Native Rust code distillation and reflex generation for distilled intents. (Tier: 0)
13. **UniversalExecutionSubstrate**: Absolute engine-agnosticism. AEON can execute any model in the world using a unified, hardware-saturated inference layer. (Tier: 2)
14. **GEMI**: Deep reasoning bridge and unified cloud provider inference racing. (Tier: 2)
15. **AEON-Vision**: Hardware-saturated neural vision substrate for visual/text semantic fusion. (Tier: 2)
16. **AEON-Audio**: Hardware-saturated neural audio substrate for spectral logic distillation. (Tier: 2)

## 4. Models - Neural Intelligence & Weights
17. **NativeAlphaModel**: Local neural weights (`aeon-alpha.safetensors`) for deterministic reflex. (Tier: 0)
18. **NativeReasoningModel**: Distilled Tier 2 logic weights (`aeon-reason.safetensors`) trained on the AEON genome. (Tier: 2)
19. **UniversalSubstrateModels**: Absolute model-agnosticism. AEON can ingest any model weights (GGUF, Safetensors, ONNX, PyTorch) from any global repository. (Tier: 2)

## 5. MCPs - Interoperability & Tooling (GMCP Infrastructure)
20. **GMCP Server**: Background daemon exposing multi-protocol endpoints (RPC: 9090, HTTP/SSE: 9093, UDP: 9092). (Tier: 1)
21. **GMCP Host**: The `aeon` CLI proxy that acts as a protocol bridge between users and the background server. (Tier: 1)
22. **GEMI Server**: Dedicated RESTful endpoint (Port 9091) for Tier 2 reasoning and model management. (Tier: 1)
23. **MetaMcpServer**: External Model Context Protocol servers connected via stdio or TCP. (Tier: 1)
24. **MetaExecutionContext**: Dynamic mission blackboard and orchestrated session memory. (Tier: 1)
