# AEON Alpha-User Mutation Protocols (Missions)

* **Current Engine Version**: `v0.1.2022787`

This document defines the mutable execution paths for the `aeon` substrate. These protocols operate within the `alpha-user` space, allowing the swarm to generate artifacts, manipulate the workspace, and distill experience without modifying the hard-compiled `alpha-self` core.

## 1. Dynamic Task Fulfillment

1. **Mission Validation**: Every `[MISSION]` instruction must be recorded in `pulse.md`. A Mission is failing if the filesystem state does not match the semantic intent.
2. **Swarm Correction**: A failing `[MISSION]` triggers the `NeuralAgentFactory` to synthesize a new specialist fleet to correct the workspace state.

## 2. Experience Distillation

3. **Substrate Ingestion Trigger**: Successful mission resolutions that exceed the semantic depth threshold must be written to `reasoning_experience.jsonl`. This state mutation triggers the `AeonRuntimeAdmin` to eventually retrain the Native Tier 2 Reasoning Model.
4. **World User Evolution**: In the absence of source code (World User mode), Experience Distillation is the primary mechanism for substrate evolution.
