# AEON Core Mutation Protocols (Motions)

* **Current Engine Version**: `v0.1.2022764`

This document defines the execution paths for altering the `alpha-self` core. A Motion is triggered only after a failing test is ingested via `TESTS.md`.

## 1. Architectural Evolution

1. **Axiom Generation**: Every core mutation must result in the generation of new hard-compiled axiom rules in `AlphaSelf`.
2. **Substrate Recompilation**: A motion must trigger a complete binary recompilation via the `BUILD.md` sequence to embed the new genome.
3. **Immutability Lock**: Once a motion is compiled, its logic becomes an immutable reflex. It cannot be altered by `alpha-user` runtime agents.
