// AEON Master Agent (AMA): The Orchestration Substrate
// RULE 11: Agents must add functionality directly to the aeon engine via ToolRegistry.
// Agents must not simulate or "fake" aeon capabilities by performing logic themselves.

use std::path::Path;
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

pub struct AmaMasterAgent;

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
    pub fn solve_clean(&self, goal: &str, workspace: &Path, version: &str) -> String {
        let res = self.solve(goal, workspace, version);
        match res {
            Ok(report) => report.final_answer,
            Err(e) => format!("AMA Engine Error: {}", e),
        }
    }

    pub fn solve(&self, goal: &str, workspace: &Path, version: &str) -> EaiResult<AmaMissionReport> {
        let goal = self.sanitize_input(goal)?;
        let lower_goal = goal.to_lowercase();
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

            // 3. Context Compression & Reflex Result Distillation (Tier 2 Hardening)
            let model_name = crate::gemi::models::ModelManager::get_selected_model()
                .unwrap_or_else(|| "aeon-alpha.safetensors".to_string());

            let lower_goal = current_goal.to_lowercase();
            let final_answer = if lower_goal.contains("admin mission") || lower_goal.contains("sync") || lower_goal.contains("audit") || lower_goal.contains("release") {
                // Mandate: Zero-Stall Technical Convergence (Aspiration 22)
                let data = AmaSupervisor::gather_weighted_wisdom(&interactions, &agents);
                format!("AMA-Technical-Convergence ({}):\n\n{}", version, data)
            } else {
                let blackboard_summary = if let Some(cp) = crate::sandbox::manager::SandboxManager::check_interrupted_checkpoint(workspace) {
                    crate::gemi::engine::ContextSummarizer::compress_blackboard(&cp.blackboard)
                } else {
                    "NO_PREVIOUS_CONTEXT".to_string()
                };

                if interactions.is_empty() {
                    format!("AMA-Reflex ({}): No active agents responded to '{}'. Context: {}", version, current_goal, blackboard_summary)
                } else {
                    let last_payload = &interactions.last().unwrap().payload;
                    if last_payload.len() > 10 {
                        last_payload.clone()
                    } else {
                        format!("AMA-Synthesis ({} via {}):\n\nProcessed goal '{}' across {} active agents. Context: {}",
                            version, model_name, current_goal, agents.len(), blackboard_summary)
                    }
                }
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
        report.push_str(&format!("- **Engine**: aeon EAI Substrate\n"));
        report.push_str(&format!("- **Version**: {}\n", crate::AEON_VERSION));
        report.push_str(&format!("- **Active Model**: {}\n\n", model_name));

        report.push_str(&axiom_summary);
        report.push_str("\n");
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

            if msg.sender == "AeonUniversalSubstrateAgent" {
                if msg.payload.contains("VIOLATION") || msg.payload.contains("FAILURE") {
                     crate::sandbox::manager::AeonAuditLogger::log_event(workspace, "TOOL_FAILURE", &msg.payload);
                }
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
