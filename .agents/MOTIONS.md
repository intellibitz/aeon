# AEON Core Mutation Protocols (Motions)

* **Current Engine Version**: `v0.1.2022770`

This document defines the execution paths for altering the `alpha-self` core. A Motion is triggered only after a failing test is ingested via `pulse.md`.

## 1. Architectural Evolution

1. **Axiom Generation**: Every `[MOTION]` test results in the generation of new hard-compiled axiom rules.
2. **Self-Healing Recompilation**: A failing `[MOTION]` test triggers the `EvolutionManager` to autonomously synthesize Rust code and recompile the binary via the `BUILD.md` sequence.
3. **Axiomatic Pulse Ingestion**: The binary must autonomously classify intents and inject them into `pulse.md` before initiating any mutation.
4. **Verification Lock**: A motion is only marked as `[x]` (Resolved) in `pulse.md` after a successful `cargo test` and binary identity match.
