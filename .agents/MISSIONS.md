# AEON Alpha-User Mutation Protocols (Missions)

* **Current Engine Version**: `v0.1.2022764`

This document defines the mutable execution paths for the `aeon` substrate. These protocols operate within the `alpha-user` space, allowing the swarm to generate artifacts, manipulate the workspace, and distill experience without modifying the hard-compiled `alpha-self` core.

## 1. Dynamic Task Fulfillment

1. **Workspace Modification**: The substrate must synthesize a swarm to execute file generation, code refactoring, or repository state manipulation. All ephemeral data must remain within the git-ignored `.aeon/` sandbox. (Intent Example: `refactor src/main.rs to use new trait`)
2. **Information Synthesis**: The substrate must orchestrate Search and Translation agents to scrape external data, synthesize a new artifact, and save it to the local workspace. (Intent Example: `find opalite song lyrics, translate to tamil, and save to lyrics.md`)

## 2. Experience Distillation

3. **Substrate Ingestion Trigger**: Successful mission resolutions that exceed the semantic depth threshold must be written to `reasoning_experience.jsonl`. This state mutation triggers the `AeonRuntimeAdmin` to eventually retrain the Native Tier 2 Reasoning Model.
