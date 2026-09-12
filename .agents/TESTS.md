# AEON Test Protocols

* **Current Engine Version**: `v0.1.2022731`

This document defines the validation procedures for verifying that the `aeon` substrate adheres to its architectural aspirations and runtime mandates. These protocols are implemented as native Rust tests within the core engine.

## 1. Version Alignment Enforcement (Build Mandate)
- **Test**: `test_version_alignment_enforcement`
- **Protocol**: Verify that the version in `Cargo.toml` matches the version badges and system information across all documentation (`README.md`, `.agents/*.md`).
- **Pass Criteria**: `verify_version_alignment` returns `Ok(())`.

## 2. Neural Swarm Synthesis (Aspiration 8 / Runtime Mandate 10)
- **Test**: `test_fleet_synthesis`
- **Protocol**: Submit goal strings like "soil crop agricultural" and verify that the `GawdAgentFleet` recruits the correct specialized agents (`AgriTechAgent`) using semantic centroid projections.
- **Pass Criteria**: Fleet contains both mandatory substrate guards and semantically relevant specialists.

## 3. High-Density Context Mapping (Aspiration 5 / Runtime Mandate 11)
- **Test**: `test_blackboard_convergence`
- **Protocol**: Execute a task with a `HighDensityContextStore` capacity limit and verify that agent outputs converge on the shared mission blackboard.
- **Pass Criteria**: Blackboard state is consistent across multi-agent executions.

## 4. Semantic Intent Determinism (Aspiration 0 / Substrate Purity)
- **Test**: `test_semantic_centroid_projection_determinism`
- **Protocol**: Project the same natural language intent multiple times and verify that the resulting vector coordinates are identical.
- **Pass Criteria**: Vector length matches `AeonAlphaModel::DIM` and projections are 100% deterministic.

## 5. Competitive Inference Racing (Aspiration 6 / Runtime Mandate 2)
- **Test**: `test_competitive_racing_logic`
- **Protocol**: Spawn parallel threads mocking fast and slow inference paths.
- **Pass Criteria**: The engine correctly selects the result from the fastest path within the race window.

## 6. Mission Persistence & Lifecycle (Runtime Mandate 9)
- **Test**: `test_checkpoint_lifecycle`
- **Protocol**: Save a `NeuralCheckpoint` to the workspace `.aeon/` directory and verify its recovery.
- **Pass Criteria**: Checkpoint state (intent, blackboard, completed tools) is preserved across engine re-initialization.

## 7. Zero-Config Environment Synthesis (Runtime Mandate 5)
- **Test**: `test_aeon_config_lifecycle`
- **Protocol**: Ensure the engine can autonomously generate and load default configurations in a fresh environment.
- **Pass Criteria**: `AeonConfig::load` returns valid defaults without manual setup.

## 8. Governance & Destructive Command Interception (Runtime Mandate 14)
- **Test**: `test_safety_audit_destructive_patterns`
- **Protocol**: Submit a mission goal containing `rm -rf /` and verify detection by the `SafetyDetector`.
- **Pass Criteria**: The audit returns a `DestructiveCommand` error and prevents execution.

## 9. Secret Token Leak Prevention (Runtime Mandate 16)
- **Test**: `test_security_audit_secret_leak`
- **Protocol**: Submit a prompt containing a sensitive pattern (e.g., `sk-proj`) and verify detection by the `SecurityDetector`.
- **Pass Criteria**: The audit returns a `SecurityLeak` error and prevents exfiltration.

## 10. Foundational Intent Discovery (Aspiration 0)
- **Test**: `test_list_dynamic_intents`
- **Protocol**: Interrogate the `AeonAlphaModel` for available intents.
- **Pass Criteria**: The model reports foundational intents (`status`, `version`, `identity`) sorted by semantic importance.

## 11. Native Reflex Synthesis (Aspiration 4 / Motion Rule)
- **Test**: `test_native_reflex_synthesis_logic`
- **Protocol**: Distill a new native reflex from a natural language intent.
- **Pass Criteria**: The synthesizer generates valid Rust code in the `gmcp/reflexes/` directory following the `Reflex` trait implementation pattern.
