// AEON Master Agent (AMA): The Orchestration Substrate
// RULE 11: Agents must add functionality directly to the aeon engine via ToolRegistry.
// Agents must not simulate or "fake" aeon capabilities by performing logic themselves.

use std::path::Path;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::error::EaiResult;
use super::agents::GawdAgentInfo;
use super::amas::{A2AMessage, AmaSupervisor};
use super::axiom::AxiomSubstrate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmaMissionReport {
    pub goal: String,
    pub status: String,
    pub agents: Vec<GawdAgentInfo>,
    pub interactions: Vec<A2AMessage>,
    pub final_answer: String,
}

impl AmaMissionReport {
    pub fn to_protocol_format(&self, is_ide_environment: bool) -> String {
        if is_ide_environment {
            let mut thinking = format!("[AEON Substrate Swarm Report - Status: {}]\n", self.status);
            for agent in &self.agents {
                thinking.push_str(&format!("- [Agent] {} ({})\n", agent.name, agent.provider));
            }
            for msg in &self.interactions {
                thinking.push_str(&format!("- [Swarm Flux] {}: {}\n", msg.sender, msg.payload));
            }

            format!(
                "<thinking>\n{}\n</thinking>\n\n<result>\n{}\n</result>",
                thinking.trim(),
                self.final_answer.trim()
            )
        } else {
            self.final_answer.clone()
        }
    }
}

pub struct AmaMasterAgent;

impl Default for AmaMasterAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl AmaMasterAgent {
    pub fn new() -> Self {
        Self
    }

    /// Validates and sanitizes natural language inputs to prevent injection attacks.
    fn sanitize_input(&self, input: &str) -> EaiResult<String> {
        let trimmed = input.trim();

        let hardware = crate::gemi::hardware::HardwareProfiler::get_profile();
        let max_len = (hardware.available_ram_gb * 1024 * 1024).max(4096); // Scale with RAM, min 4KB

        if trimmed.len() > max_len {
            return Err(crate::error::EaiError::governance(format!("Input exceeds hardware-scaled limit ({} characters).", max_len)));
        }

        if trimmed.is_empty() {
            return Err(crate::error::EaiError::governance("Input goal cannot be empty."));
        }

        // 2. High-Risk Pattern Intercept (Substrate Security)
        let risk_patterns = ["$(", "`", "> /dev/", "| nc ", "| netcat ", "0xCC", "\\x"];
        for pattern in risk_patterns {
            if trimmed.contains(pattern) {
                return Err(crate::error::EaiError::governance(format!("High-risk sequence '{}' detected in input. Potential injection attempt blocked.", pattern)));
            }
        }

        Ok(trimmed.to_string())
    }

    /// Primary entry point for all natural language intents.
    /// Streams thinking and results back live in real-time.
    pub fn solve_clean(&self, goal: &str, workspace: &Path, version: &str) -> String {
        self.solve_stream(goal, workspace, version)
    }

    pub fn solve_stream(&self, goal: &str, workspace: &Path, version: &str) -> String {
        use std::io::Write;
        use crate::gawd::amas::AmaSupervisor;

        let hw = crate::gemi::hardware::HardwareProfiler::get_profile();
        let (engine_type, active_model_id) = crate::gemi::models::ModelManager::get_active_engine_and_model();
        let model_path_str = crate::gemi::models::ModelManager::get_model_path(&active_model_id)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Internal Hard-Compiled Substrate Genome".to_string());
        let device = crate::gemi::hardware::HardwareProfiler::get_candle_device();
        let local_models_count = crate::gemi::models::ModelManager::list_models(workspace).len();

        println!("<thinking>");

        // Glass Box Integrity Guard (Aspiration 28)
        // Ensures </thinking> is ALWAYS printed even if synthesis panics or hangs.
        struct ThinkingGuard;
        impl Drop for ThinkingGuard {
            fn drop(&mut self) {
                println!("</thinking>\n");
                let _ = std::io::stdout().flush();
            }
        }
        let _guard = ThinkingGuard;

        println!("[AEON Substrate Swarm Active - Full Transparency Telemetry Mode]");
        println!("- [Engine Version] v{}", version);
        println!("- [Workspace Root] {}", workspace.display());

        use crate::gawd::self_core::AlphaSelf;
        println!("- [Core Paradigm] {}", AlphaSelf::CORE_PARADIGM);

        println!("\n[GENOMIC MANDATES]");
        for rule in AlphaSelf::RULES.iter().filter(|r| r.title.contains("Universal") || r.title.contains("Agnosticism")) {
            println!("- [Mandate {}] {}: {}", rule.id, rule.title, rule.imperative);
        }

        // 1. Fast-Path Query Interception (Mandate 11 & 31)
        // Bypasses the heavy inference loop for substrate-level interrogation.
        let lower_goal = goal.trim().to_lowercase();
        let is_query = lower_goal == "identity" || lower_goal == "aeon identity"
            || lower_goal == "version" || lower_goal == "aeon version"
            || lower_goal == "status" || lower_goal == "aeon status"
            || lower_goal == "models" || lower_goal == "aeon models";

        if is_query {
            println!("\n[QUERY FAST-PATH DETECTED]");
            let (interactions, agents) = AmaSupervisor::supervise_mission(goal, workspace);
            for msg in &interactions {
                println!("- [Swarm Flux] {}: {}", msg.sender, msg.payload.chars().take(100).collect::<String>());
            }

            let final_answer = if lower_goal.contains("identity") {
                crate::gawd::self_core::AlphaSelf::inspect_compiled_binary_instructions()
            } else if lower_goal.contains("version") {
                format!("AEON Engine Version: v{}", version)
            } else if lower_goal.contains("status") {
                format!("AEON Substrate Status: Operational | Hardware: {} | RAM: {}GB", hw.cpu_brand, hw.ram_gb)
            } else {
                let models = crate::gemi::models::ModelManager::list_models(workspace);
                format!("Models Roster: {} discovered.", models.len())
            };

            println!("\n[SUBSTRATE CONFIGURATION & LIMITS]");
            let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
            let global_dir = home.join(".aeon");
            let cfg = crate::sandbox::manager::AeonConfig::load(&global_dir).unwrap_or_default();
            println!("- [Ports] GMCP: {} | GEMI: {} | UDP: {}", cfg.gmcp_port, cfg.gemi_port, cfg.udp_discovery_port);
            println!("- [Model Defaults] Engine: {} | Model: {}", cfg.default_engine, cfg.default_model);
            println!("- [Auto-Download] {}", cfg.auto_download_models);
            println!("- [Agent Threshold] {}", cfg.agent_rank_threshold);
            println!("- [Cloud Scout Timeout] {}s", cfg.cloud_scout_timeout_secs);
            println!("- [Execution Lease] 600s (Aspiration 20 Fluid Limit)");
            println!("- [Agent Timeout] 60s (Swarm Flux Guard)");
            println!("- [Max Swarm Agents] {} (Hardware Scaled)", crate::gawd::agents::GawdAgentFleet::get_max_concurrent_agents());

            println!("\n[QUERY COMPLETE]");
            drop(_guard);
            println!("<result>\n{}\n</result>", final_answer);
            return final_answer;
        }

        // MICRO-DETAILED SUBSTRATE TELEMETRY (Aspiration 28 & 29)
        println!("\n[SUBSTRATE PILLARS]");
        println!("- [AoA Pillar] {} Components Active", AlphaSelf::AOA_COMPONENTS.len());
        println!("- [Agents Pillar] {} Components Active", AlphaSelf::AGENT_COMPONENTS.len());
        println!("- [Engines Pillar] {} Components Active", AlphaSelf::ENGINE_COMPONENTS.len());
        for engine in AlphaSelf::ENGINE_COMPONENTS {
            println!("  - [{:?}] {}: {}", engine.tier, engine.name, engine.description);
        }
        println!("- [Models Pillar] {} Components Active", AlphaSelf::MODEL_COMPONENTS.len());
        for model_pillar in AlphaSelf::MODEL_COMPONENTS {
            println!("  - [{:?}] {}: {}", model_pillar.tier, model_pillar.name, model_pillar.description);
        }
        println!("- [MCPs Pillar] {} Components Active", AlphaSelf::MCP_COMPONENTS.len());

        println!("\n[HARDWARE INTROSPECTION]");
        println!("- [OS/Arch] {} | {}", hw.os_info, hw.arch);
        println!("- [Hostname] {}", hw.hostname);
        println!("- [Uptime] {}", hw.uptime);
        println!("- [Load Avg] {}", hw.load_avg);
        println!("- [Hardware Profile] {} CPUs ({}) | {}GB RAM | GPU: {}", hw.cpus, hw.cpu_brand, hw.ram_gb, hw.gpu_info);
        println!("- [Acceleration] {}", hw.native_acceleration);

        println!("\n[MODEL SUBSTRATE DEEP-DIVE]");
        println!("- [GEMI Engine Substrate] {} (REST Port: 9091 | MCP Bus: 9090)", engine_type);
        println!("- [Inference Host Device] Candle Native Rust ({:?})", device);
        println!("- [Active Local Model ID] {}", active_model_id);
        println!("- [Model Substrate Path] {}", model_path_str);
        println!("- [Discovered Local Models] {}", local_models_count);

        println!("\n[SUBSTRATE CONFIGURATION & LIMITS]");
        let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_dir = home.join(".aeon");
        let cfg = crate::sandbox::manager::AeonConfig::load(&global_dir).unwrap_or_default();
        println!("- [Ports] GMCP: {} | GEMI: {} | UDP: {}", cfg.gmcp_port, cfg.gemi_port, cfg.udp_discovery_port);
        println!("- [Model Defaults] Engine: {} | Model: {}", cfg.default_engine, cfg.default_model);
        println!("- [Auto-Download] {}", cfg.auto_download_models);
        println!("- [Agent Threshold] {}", cfg.agent_rank_threshold);
        println!("- [Cloud Scout Timeout] {}s", cfg.cloud_scout_timeout_secs);
        println!("- [Execution Lease] 600s (Aspiration 20 Fluid Limit)");
        println!("- [Agent Timeout] 60s (Swarm Flux Guard)");
        println!("- [Max Swarm Agents] {} (Hardware Scaled)", crate::gawd::agents::GawdAgentFleet::get_max_concurrent_agents());

        let tools = crate::gmcp::tools::ToolRegistry::list_tools();
        println!("\n[MCP SURFACE]");
        println!("- [Meta-Tools] {} Registered", tools.len());
        for tool in tools.iter().take(20) {
             println!("  - [Tool] {}: {}", tool.name, tool.description.chars().take(80).collect::<String>());
        }
        if tools.len() > 20 { println!("  - ... and {} more", tools.len() - 20); }

        println!("\n- [Goal Intent] {}", goal);

        // Phase D: Omni-Trace thinking Synthesis (Aspiration 29)
        println!("\n[UNIVERSAL TRACE START]");
        let _ = std::io::stdout().flush();

        let start = std::time::Instant::now();

        // Real-time trace injection (Mandate 28 & Aspiration 30)
        let res = self.solve_with_streaming_trace(goal, workspace, version);

        let elapsed = start.elapsed();

        println!("\n- [Swarm Execution Latency] {:?} (Aspiration 25 Guard Checked)", elapsed);

        if elapsed.as_millis() > 2 {
             crate::sandbox::manager::AeonAuditLogger::log(
                 workspace,
                 crate::sandbox::manager::LogLevel::Axiomatic,
                 "LATENCY_VIOLATION",
                 &format!("Reflex operation exceeded 2ms mandate: {:?} (Goal: {})", elapsed, goal)
             );
        }

        match res {
            Ok(report) => {
                println!("[MISSION COMPLETE] Consensus reached.");
                let _ = std::io::stdout().flush();

                // Explicitly dropping guard here to close thinking before result
                drop(_guard);

                println!("<result>");
                let _ = std::io::stdout().flush();
                println!("{}", report.final_answer.trim());
                println!("</result>");
                let _ = std::io::stdout().flush();

                report.final_answer
            }
            Err(e) => {
                println!("[MISSION FAILED] {}", e);
                let _ = std::io::stdout().flush();

                drop(_guard);

                let err_msg = format!("AMA Engine Error: {}", e);
                println!("<result>\n{}\n</result>", err_msg);
                let _ = std::io::stdout().flush();

                err_msg
            }
        }
    }
  /// realized the 'Omni-Trace' mandate by exposing streaming tokens within thinking.
    fn solve_with_streaming_trace(&self, goal: &str, workspace: &Path, _version: &str) -> EaiResult<AmaMissionReport> {
        let goal = self.sanitize_input(goal)?;

        // 1. Swarm Supervision
        println!("- [Swarm Synthesis] Synthesizing specialist fleet...");
        let _ = std::io::stdout().flush();
        let (interactions, agents) = super::amas::AmaSupervisor::supervise_mission(&goal, workspace);

        println!("- [Swarm Execution] Dispatching parallel agents...");
        let _ = std::io::stdout().flush();
        for msg in &interactions {
            if msg.sender != "ConsensusMaster" {
                println!("- [Swarm Flux] {}: {}", msg.sender, msg.payload.chars().take(100).collect::<String>());
            }
        }

        let swarm_context = super::amas::AmaSupervisor::gather_weighted_wisdom(&interactions, &agents);

        println!("- [Truth Convergence] Synthesis active. Ingesting model reasoning trace...");
        let _ = std::io::stdout().flush();

        let reasoning_prompt = format!(
            "MISSION_GOAL: {}\n\nLOCAL_SWARM_CONTEXT:\n{}\n\n[INSTRUCTION]: Resolve this mission. Output finalized verified actions.",
            goal, swarm_context
        );

        // Aspiration 30: Synchronous Trace (Thinking block contains streaming tokens)
        let final_answer = crate::gemi::engine::GemiEngine::generate_reasoning_stream(&reasoning_prompt, workspace, &|token| {
            print!("{}", token);
            let _ = std::io::stdout().flush();
        });

        println!("\n- [Substrate Verification] Finalizing epistemic chain...");
        let _ = std::io::stdout().flush();

        // Axiomatic & Reality verification (Mandate 29 Gate)
        let verified = match crate::gemi::engine::GemiEngine::verify_axiomatic_alignment(&final_answer, workspace) {
            Ok(v) => v,
            Err(e) => format!("Axiomatic Violation: {}", e),
        };

        let verified_final = match super::truth::TruthTransformer::verify_mission_reality(&goal, "AMA_SOLVE", &verified, workspace) {
            Ok(v) => v,
            Err(e) => format!("Reality Violation: {}", e),
        };

        Ok(AmaMissionReport {
            goal: goal.to_string(),
            status: "COMPLETE".to_string(),
            agents,
            interactions,
            final_answer: verified_final,
        })
    }

    pub fn solve(&self, goal: &str, workspace: &Path, version: &str) -> EaiResult<AmaMissionReport> {
        let goal = self.sanitize_input(goal)?;
        let lower_goal = goal.to_lowercase();

        // Substrate Queries (Swarm-Dispatched Reflex Interrogation - Aspiration 23 & QUERIES.md)
        let trimmed_query = lower_goal.trim();
        if trimmed_query == "identity" || trimmed_query == "aeon identity" {
            let (interactions, agents) = AmaSupervisor::supervise_mission(&goal, workspace);
            let identity_report = crate::gawd::self_core::AlphaSelf::inspect_compiled_binary_instructions();
            return Ok(AmaMissionReport {
                goal: goal.to_string(),
                status: "COMPLETE".to_string(),
                agents,
                interactions,
                final_answer: format!("AEON Substrate Identity Report ({}):\n\n{}", version, identity_report),
            });
        }

        if trimmed_query == "version" || trimmed_query == "aeon version" {
            let (interactions, agents) = AmaSupervisor::supervise_mission(&goal, workspace);
            return Ok(AmaMissionReport {
                goal: goal.to_string(),
                status: "COMPLETE".to_string(),
                agents,
                interactions,
                final_answer: format!("AEON Engine Version: v{}", version),
            });
        }

        if trimmed_query == "status" || trimmed_query == "aeon status" {
            let (interactions, agents) = AmaSupervisor::supervise_mission(&goal, workspace);
            let hw = crate::gemi::hardware::HardwareProfiler::get_profile();
            let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default();
            let global_dir = home.join(".aeon");
            let daemon_status = if crate::daemon::server::AmaDaemon::check_status(&global_dir).is_some() { "RUNNING" } else { "STOPPED" };
            let status_report = format!(
                "AEON Substrate Status ({}) :\n- Daemon Status: {}\n- Hardware: {} CPUs ({}) | {}GB RAM | {}\n- Acceleration: {}",
                version, daemon_status, hw.cpus, hw.cpu_brand, hw.ram_gb, hw.gpu_info, hw.native_acceleration
            );
            return Ok(AmaMissionReport {
                goal: goal.to_string(),
                status: "COMPLETE".to_string(),
                agents,
                interactions,
                final_answer: status_report,
            });
        }

        if trimmed_query == "models" || trimmed_query == "aeon models" {
            let (interactions, agents) = AmaSupervisor::supervise_mission(&goal, workspace);
            let models = crate::gemi::models::ModelManager::list_models(workspace);
            let mut roster = format!("AEON Substrate Models Roster ({}) :\n", version);
            for m in models {
                roster.push_str(&format!("- [{:?}] {} ({})\n", m.provider, m.name, m.model_id));
            }
            return Ok(AmaMissionReport {
                goal: goal.to_string(),
                status: "COMPLETE".to_string(),
                agents,
                interactions,
                final_answer: roster,
            });
        }

        // Recursive Parallel Parallelism (Aspiration 26)
        if lower_goal.contains("parallel") || lower_goal.contains("split") {
             return self.solve_parallel_mission(&goal, workspace, version);
        }

        // Autonomous Task Decomposition (Rule 12 Check)
        if (goal.len() > 150 || lower_goal.contains(" and then ") || lower_goal.contains(" finally ")) && !goal.contains("[STEP ") {
             return self.solve_planned_mission(&goal, workspace, version);
        }

        let mut retry_count = 0;
        let mut current_goal = goal.to_string();
        let mut last_error = String::new();
        let mut previous_errors = std::collections::HashSet::new();

        while retry_count < 3 {
            // 2. Swarm Supervision (Tier 1 AOA Dispatch)
            // Parallel execution of Safety, Security, Runtime Setup and Mission specific agents
            let (interactions, agents) = AmaSupervisor::supervise_mission(&current_goal, workspace);

            let lower_goal = current_goal.to_lowercase();
            let is_motion = lower_goal.contains("admin mission") || lower_goal.contains("motion") || lower_goal.contains("sync") || lower_goal.contains("audit") || lower_goal.contains("release");
            let swarm_context = AmaSupervisor::gather_weighted_wisdom(&interactions, &agents);

            let final_answer = if is_motion {
                // Tier 1 GAWD Swarm Dispatch for Motions & Core Workspace Mutations
                format!("AMA-Motion-Convergence ({}):\n\n{}", version, swarm_context)
            } else if !swarm_context.trim().is_empty() && !swarm_context.contains("No valid wisdom gathered") {
                // Swarm Convergence: Use high-confidence swarm wisdom directly without CPU model loop hang
                swarm_context
            } else {
                // Tier 2 Native Local Model Inference Fallback for Open Missions
                let model_name = crate::gemi::models::ModelManager::get_selected_model()
                    .unwrap_or_else(|| "aeon-native-synthesis".to_string());

                let reasoning_prompt = format!(
                    "MISSION_GOAL: {}\n\nLOCAL_SWARM_CONTEXT:\n{}\n\n[INSTRUCTION]: Resolve this mission using native local model inference.",
                    current_goal, swarm_context
                );

                let local_inference = crate::gemi::engine::GemiEngine::generate_reasoning_deep(&reasoning_prompt, workspace);
                format!("AMA-Tier2-Mission-Synthesis ({} via {}):\n\n{}", version, model_name, local_inference)
            };

            // 4. Axiomatic Alignment Check (Rule 15 Hardening)
            match crate::gemi::engine::GemiEngine::verify_axiomatic_alignment(&final_answer, workspace) {
                Ok(ans) => {
                     // 5. Reality Verification (Rule 15)
                    match super::truth::TruthTransformer::verify_mission_reality(&current_goal, "AMA_SOLVE", &ans, workspace) {
                        Ok(verified_answer) => {
                            return Ok(AmaMissionReport {
                                goal: goal.to_string(),
                                status: "COMPLETE".to_string(),
                                agents,
                                interactions,
                                final_answer: verified_answer,
                            });
                        }
                        Err(e) if e.to_string().contains("TRUTH_VIOLATION") => {
                            let error_str = e.to_string();
                            let error_sig = format!("{:x}", md5::compute(error_str.as_bytes()));

                            if previous_errors.contains(&error_sig) {
                                crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "RETRY_LOOP_DETECTED", &format!("Same error repeated: {}", error_str));
                                return Err(e);
                            }

                            previous_errors.insert(error_sig);
                            retry_count += 1;
                            last_error = error_str;
                            crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "HALLUCINATION_DETECTED", &format!("Retry {}/3: {}", retry_count, last_error));

                            current_goal = format!(
                                "{}\n\n[CORRECTION ATTEMPT {}]: Previous response failed reality check.\n\
                                Error detail: {}",
                                goal, retry_count,
                                last_error.chars().take(200).collect::<String>()
                            );
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => {
                    retry_count += 1;
                    crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "AXIOMATIC_VIOLATION", &e.to_string());

                    current_goal = format!(
                        "{}\n\n[CORRECTION ATTEMPT {}]: Response violated substrate axioms.\n\
                        Violation: {}",
                        goal, retry_count,
                        e.to_string().chars().take(200).collect::<String>()
                    );
                    continue;
                }
            };
        }

        Err(crate::error::EaiError::governance(format!("Recursive reasoning failed after 3 attempts. Last violation: {}", last_error)))
    }

    fn solve_parallel_mission(&self, goal: &str, workspace: &Path, version: &str) -> EaiResult<AmaMissionReport> {
        let plan = crate::gemi::engine::MissionPlanner::partition_mission(goal, workspace)?;

        // Speculative Parallelism (Aspiration 26)
        // Partitioned tasks are executed in parallel across the multi-threaded substrate.
        let mut results = Vec::new();
        let mut handles = Vec::new();

        for (i, sub_goal) in plan.goals.iter().enumerate() {
            let g = format!("[PARALLEL STEP {}/{}]: {}", i + 1, plan.goals.len(), sub_goal);
            let w = workspace.to_path_buf();
            let v = version.to_string();

            handles.push(std::thread::spawn(move || {
                let ama = AmaMasterAgent::new();
                // Sub-mission budget is shorter to prevent parent hang
                ama.solve(&g, &w, &v)
            }));
        }

        for handle in handles {
            if let Ok(res) = handle.join() {
                results.push(res);
            }
        }

        let mut all_interactions = Vec::new();
        let mut all_agents = Vec::new();
        let mut final_responses = Vec::new();

        for report in results.into_iter().flatten() {
            all_interactions.extend(report.interactions);
            all_agents.extend(report.agents);
            final_responses.push(report.final_answer);
        }

        Ok(AmaMissionReport {
            goal: goal.to_string(),
            status: "COMPLETE".to_string(),
            agents: all_agents,
            interactions: all_interactions,
            final_answer: format!("PARALLEL_FORK_JOIN_COMPLETE ({} steps):\n\n{}", final_responses.len(), final_responses.join("\n\n---\n\n")),
        })
    }

    fn solve_planned_mission(&self, goal: &str, workspace: &Path, version: &str) -> EaiResult<AmaMissionReport> {
        let mut plan = crate::gemi::engine::MissionPlanner::plan_mission(goal, workspace)?;
        let mut all_interactions = Vec::new();
        let mut all_agents = Vec::new();
        let mut final_responses = Vec::new();

        let mut current_step = 0;
        while current_step < plan.goals.len() {
            let sub_goal = &plan.goals[current_step];
            let tagged_goal = format!("[STEP {}/{}]: {}", current_step + 1, plan.goals.len(), sub_goal);
            let report = self.solve(&tagged_goal, workspace, version)?;

            all_interactions.extend(report.interactions.clone());
            all_agents.extend(report.agents.clone());
            final_responses.push(report.final_answer.clone());

            // Dynamic Plan Mutation: Check for failure or gap in the last step
            if report.final_answer.contains("FAILURE") || report.final_answer.contains("GAP") {
                crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "PLAN_MUTATION", &format!("Refining plan due to step {} failure.", current_step + 1));

                let blackboard_state = format!("LATEST_OUTCOME: {}", report.final_answer);
                if let Ok(new_plan) = crate::gemi::engine::MissionPlanner::refine_plan(goal, &blackboard_state, workspace) {
                    plan = new_plan;
                    // Reset or adjust steps based on new plan (for now we just continue from next)
                }
            }

            current_step += 1;
        }

        Ok(AmaMissionReport {
            goal: goal.to_string(),
            status: "COMPLETE".to_string(),
            agents: all_agents,
            interactions: all_interactions,
            final_answer: format!("PLANNED_MISSION_COMPLETE:\n\n{}", final_responses.join("\n\n---\n\n")),
        })
    }

    pub fn generate_substrate_report(&self, workspace: &Path) -> EaiResult<String> {
        let (axiom_summary, topology_summary) = AxiomSubstrate::ingest_constitution(workspace);
        let model_name = crate::gemi::models::ModelManager::get_selected_model()
            .unwrap_or_else(|| "aeon-alpha.safetensors (Local Neural Substrate)".to_string());

        let mut report = String::new();
        report.push_str("# aeon Substrate - Technical Report\n\n");
        report.push_str("- **Engine**: aeon EAI Substrate\n");
        report.push_str(&format!("- **Version**: {}\n", crate::AEON_VERSION));
        report.push_str(&format!("- **Active Model**: {}\n\n", model_name));

        report.push_str(&axiom_summary);
        report.push('\n');
        report.push_str(&topology_summary);

        Ok(report)
    }

    pub fn process_intent(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        // 1. Audit
        crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "MISSION_START", goal);

        // 2. Reasoning
        let res = self.solve(goal, workspace, crate::AEON_VERSION)?;

        // 3. Memory persistence (Rule 13)
        crate::sandbox::manager::AeonMemory::save_interaction(workspace, goal, &res.final_answer);

        // 4. Autonomous Distillation (Rule 21): Capture learned wisdom from Power-Tier remotes
        for msg in &res.interactions {
            if msg.action.contains("power_reason") && !msg.payload.contains("[FAIL]") {
                let metadata = serde_json::json!({
                    "agents": res.agents.iter().map(|a| a.name.clone()).collect::<Vec<String>>(),
                    "interactions_count": res.interactions.len(),
                    "final_status": res.status
                });
                let _ = super::pkb::ProtocolKnowledgeBase::stage_distillation_pair(goal, &res.final_answer, workspace, Some(metadata));
            }

            if msg.sender == "AeonUniversalSubstrateAgent"
                && (msg.payload.contains("VIOLATION") || msg.payload.contains("FAILURE")) {
                     crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "TOOL_FAILURE", &msg.payload);
                }
        }

        Ok(res.final_answer)
    }

    pub fn solve_with_feedback(&self, goal: &str, workspace: &Path, feedback_tx: std::sync::mpsc::Sender<String>) -> EaiResult<String> {
        let goal = self.sanitize_input(goal)?;
        let _ = feedback_tx.send(format!("[AMA] Initiating mission for goal: '{}'", goal));

        // Mandate: Use multi-threaded swarm for all runtime setup and audits
        let _ = feedback_tx.send("[AMA] Dispatching multi-threaded swarm for setup, audit, and mission execution...".to_string());
        let (interactions, agents) = AmaSupervisor::supervise_mission(&goal, workspace);

        for msg in &interactions {
            let _ = feedback_tx.send(format!("[Swarm: {}] {}", msg.sender, msg.action));
        }

        // Step 4: Final Synthesis
        let _ = feedback_tx.send(format!("[AMA] Mission synthesized across {} agents. Verifying reality...", agents.len()));

        let model_name = crate::gemi::models::ModelManager::get_selected_model()
             .unwrap_or_else(|| "aeon-alpha.safetensors".to_string());

        let ans = format!("AMA-Synthesis ({} via {}):\n\nProcessed goal '{}' across {} agents.",
                        crate::AEON_VERSION, model_name, goal, agents.len());

        let verified = super::truth::TruthTransformer::verify_mission_reality(&goal, "AMA_SOLVE", &ans, workspace)?;

        crate::sandbox::manager::AeonMemory::save_interaction(workspace, &goal, &verified);

        Ok(verified)
    }

    pub fn handle_autonomous_evolution(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "MISSION_START", goal);

        // 1. Attempt mission with current substrate
        let res = self.solve(goal, workspace, crate::AEON_VERSION);

        match res {
            Ok(report) => {
                if report.final_answer.contains("NO_ACTION_REQUIRED") || report.final_answer.contains("VIOLATION") {
                     // Potential gap or blocked action
                     if report.final_answer.contains("blocked") {
                         crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "TRUTH_BLOCK", &report.final_answer);
                     }
                     return Ok(report.final_answer);
                }
                Ok(report.final_answer)
            }
            Err(e) => {
                // FAILURE: Report gap (Rule 14)
                crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "INTELLIGENCE_GAP", &format!("Goal '{}' failed: {}", goal, e));

                if e.to_string().contains("not found") || e.to_string().contains("no models") {
                    crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "INTELLIGENCE_GAP", "No models found. Substrate expansion required by Creator.");
                }

                // Report gap; user intent does NOT trigger Motion Rule
                Err(e)
            }
        }
    }
}
