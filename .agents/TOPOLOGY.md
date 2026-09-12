# AEON Substrate Topology

* **Current Engine Version**: `v0.1.2022745`

This document defines the structural native components and orchestrated meta-layers of the `aeon` substrate, organized into five functional pillars.

## 1. Agent of Agents (AoA) - Coordination & Governance
1. **GAWD / AMA**: Universal swarm supervisor and multi-agent parallel dispatcher. (Tier: 1)
2. **AeonAdmin**: Native administrative substrate for release orchestration and compliance auditing. (Tier: 1)
3. **EvolutionManager**: Substrate self-healing and autonomous Motion Rule execution. (Tier: 1)
4. **AmaDaemon**: Persistent background host and process manager for the GMCP/GEMI server fleet. (Tier: 1)

## 2. Agents - Functional Execution Units
5. **AeonRuntimeAgent**: Autonomous environment preparation agent (Weights & Tools). (Tier: 1)
6. **SafetyAgent**: Governance auditor and destructive command interceptor. (Tier: 1)
7. **ContextAgent**: High-density context manager and workspace analyzer. (Tier: 1)
8. **NeuralAgentFactory**: Autonomous synthesis and recruitment of domain-specific specialist agents. (Tier: 1)

## 3. Engines - Execution & Inference Substrates
9. **AEON-Alpha**: Microsecond intent classification and deterministic neural reflex engine. (Tier: 0)
10. **ReflexSynthesizer**: Native Rust code distillation and reflex generation for distilled intents. (Tier: 0)
11. **UniversalExecutionSubstrate**: Absolute engine-agnosticism. AEON can execute any model in the world (any weights, any architecture) using a unified, hardware-saturated inference layer. (Tier: 2)
12. **GEMI**: Deep reasoning bridge and unified cloud provider inference racing. (Tier: 2)
13. **AEON-Vision**: Hardware-saturated neural vision substrate for visual/text semantic fusion. (Tier: 2)
14. **AEON-Audio**: Hardware-saturated neural audio substrate for spectral logic distillation. (Tier: 2)

## 4. Models - Neural Intelligence & Weights
15. **NativeAlphaModel**: Local neural weights (`aeon-alpha.safetensors`) for deterministic reflex. (Tier: 0)
16. **NativeReasoningModel**: Distilled Tier 2 logic weights (`aeon-reason.safetensors`) trained on the AEON genome. (Tier: 2)
17. **UniversalSubstrateModels**: Absolute model-agnosticism. AEON can ingest any model weights (GGUF, Safetensors, ONNX, PyTorch) from any global repository (Hugging Face, ModelScope, Local Vaults). (Tier: 2)

## 5. MCPs - Interoperability & Tooling (GMCP Infrastructure)
18. **GMCP Server**: Background daemon exposing multi-protocol endpoints (RPC: 9090, HTTP/SSE: 9093, UDP: 9092) for external AI interoperability. (Tier: 1)
19. **GMCP Host**: The `aeon` CLI proxy that acts as a protocol bridge between users and the background server. (Tier: 1)
20. **GEMI Server**: Dedicated RESTful endpoint (Port 9091) for Tier 2 reasoning and model management. (Tier: 1)
21. **MetaMcpServer**: External Model Context Protocol servers connected via stdio or TCP. (Tier: 1)
22. **MetaExecutionContext**: Dynamic mission blackboard and orchestrated session memory. (Tier: 1)
