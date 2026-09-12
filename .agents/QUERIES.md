# AEON Zero-Mutation Query Protocols

* **Current Engine Version**: `v0.1.2022763`

This document defines the absolute zero-mutation execution paths for the `aeon` substrate. These protocols bypass all evolutionary and mission-state pipelines. They are strictly read-only and ephemeral.

## 1. Substrate Interrogation

1. **Identity Report**: The substrate must accurately report its hard-compiled genome, axiomatic rule count, and 5-pillar topology without altering its state or triggering context memory. (Intent: `aeon identity`)
2. **Health Status**: The substrate must report the live status of the daemon, the active execution engine, and hardware parallel threads. (Intent: `aeon status`)
3. **Model Roster**: The substrate must list all discovered models across local vaults and configured cloud registries without initiating new downloads or modifying the `RUNTIME` configuration. (Intent: `aeon models`)

## 2. Ephemeral Analytics

4. **Stateless Logic**: The substrate must execute rapid text, vision, or audio analysis using its native models and return the semantic report directly to `stdout` without staging the experience for distillation or writing to the workspace. (Intent Example: `analyze visual/image.png`)
