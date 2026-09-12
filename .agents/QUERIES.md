# AEON Zero-Mutation Query Protocols

* **Current Engine Version**: `v0.1.2022785`

This document defines the absolute zero-mutation execution paths for the `aeon` substrate. These protocols bypass all evolutionary and mission-state pipelines. They are strictly read-only and ephemeral.

## 1. Substrate Interrogation

1. **Query Verification**: Even zero-mutation `[QUERY]` instructions must be logged in `pulse.md` for Truth Verification.
2. **Truth Auditing**: A Query is failing if the returned semantic report contradicts the hard-compiled `AlphaSelf` genome.
3. **Reflex Correction**: Failing Queries trigger a core `[MOTION]` to heal the reporting engine's logic.

## 2. Ephemeral Analytics

4. **Stateless Logic**: The substrate must execute rapid text, vision, or audio analysis using its native models and return the semantic report directly to `stdout` without staging the experience for distillation or writing to the workspace. (Intent Example: `analyze visual/image.png`)
