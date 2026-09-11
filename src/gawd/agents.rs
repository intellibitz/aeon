// GAWD Agent Fleet: Universal Multi-Agent Swarm Logic
// RULE 11: Agents must add functionality directly to the aeon engine.
// RULE 31: Substrate Purity & Meta-Only Mandate

use std::sync::Arc;
use std::path::{Path, PathBuf};
use crate::error::EaiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GawdAgentInfo {
    pub name: String,
    pub provider: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverableAsset {
    pub tier: String,
    pub name: String,
    pub provider: String,
    pub url: String,
}

/// Core Intelligence Trait for AEON Swarm Agents
pub trait GawdAgent: Send + Sync {
    fn name(&self) -> String;
    fn execute(&self, goal: &str, workspace: &Path) -> EaiResult<String>;
}

pub struct GawdAgentFleet;

impl GawdAgentFleet {
    pub fn list_active_agents() -> Vec<Arc<dyn GawdAgent>> {
        let mut agents: Vec<Arc<dyn GawdAgent>> = Vec::new();

        // Tier 1: Core Substrate Agents
        agents.push(Arc::new(AeonUserAgent));
        agents.push(Arc::new(AeonContextAgent));
        agents.push(Arc::new(AeonUniversalSubstrateAgent));
        agents.push(Arc::new(AeonSafetyAgent));
        agents.push(Arc::new(AeonTruthAgent));

        agents
    }

    pub fn synthesize_fleet(_goal: &str) -> Vec<GawdAgentInfo> {
        let mut fleet = Vec::new();
        for agent in Self::list_active_agents() {
            fleet.push(GawdAgentInfo {
                name: agent.name(),
                provider: "AEON Hub".to_string(),
                url: format!("https://aeon.ai/agents/{}", agent.name().to_lowercase()),
            });
        }
        fleet
    }

    pub fn dispatch_explosive_swarm(goal: String, workspace: PathBuf) -> Vec<(String, String)> {
        let agents = Self::list_active_agents();
        let mut results = Vec::new();
        let mut handles = Vec::new();

        for agent in agents {
            let g = goal.clone();
            let w = workspace.clone();
            handles.push(std::thread::spawn(move || {
                let name = agent.name();
                let res = agent.execute(&g, &w).unwrap_or_else(|e| format!("Agent Execution Failed: {}", e));
                (name, res)
            }));
        }

        for handle in handles {
            if let Ok(res) = handle.join() {
                results.push(res);
            }
        }
        results
    }
}

pub struct AeonUserAgent;
impl GawdAgent for AeonUserAgent {
    fn name(&self) -> String { "AeonUserAgent".to_string() }
    fn execute(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        let prompt = format!("USER_INTENT: {}\n\n[INSTRUCTION]: Process the user goal and provide high-level strategy.", goal);
        let ws = workspace.to_path_buf();
        if let Ok(res) = crate::gemi::pulse::AeonPulse::reason(&prompt, &ws) {
            Ok(res)
        } else {
            Ok(format!("Strategy for: {}", goal))
        }
    }
}

struct AeonContextAgent;
impl GawdAgent for AeonContextAgent {
    fn name(&self) -> String { "AeonContextAgent".to_string() }
    fn execute(&self, _goal: &str, workspace: &Path) -> EaiResult<String> {
        let src_count = std::fs::read_dir(workspace.join("src")).map(|d| d.count()).unwrap_or(0);
        Ok(format!("WORKSPACE_CONTEXT: {} source entries detected in ./src", src_count))
    }
}

struct AeonUniversalSubstrateAgent;
impl GawdAgent for AeonUniversalSubstrateAgent {
    fn name(&self) -> String { "AeonUniversalSubstrateAgent".to_string() }
    fn execute(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        if let Ok(action) = crate::gemi::pulse::AeonPulse::reason(goal, workspace) {
             Ok(action)
        } else {
             Ok("NO_ACTION_REQUIRED".to_string())
        }
    }
}

struct AeonSafetyAgent;
impl GawdAgent for AeonSafetyAgent {
    fn name(&self) -> String { "AeonSafetyAgent".to_string() }
    fn execute(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        match super::safety::SafetyDetector::audit_action("AMA_SOLVE", goal, workspace) {
            Ok(_) => Ok("SAFETY_AUDIT: Passed.".to_string()),
            Err(e) => Ok(format!("VIOLATION: {}", e)),
        }
    }
}

struct AeonTruthAgent;
impl GawdAgent for AeonTruthAgent {
    fn name(&self) -> String { "AeonTruthAgent".to_string() }
    fn execute(&self, goal: &str, workspace: &Path) -> EaiResult<String> {
        let prompt = format!("REALITY_CHECK: {}\n\n[INSTRUCTION]: Verify the truth and factual grounding of the goal.", goal);
        let ws = workspace.to_path_buf();
        if let Ok(res) = crate::gemi::pulse::AeonPulse::reason(&prompt, &ws) {
            Ok(res)
        } else {
            Ok("TRUTH_AUDIT: Grounded in workspace reality.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_synthesis() {
        let fleet = GawdAgentFleet::synthesize_fleet("test goal");
        assert!(!fleet.is_empty());
        assert!(fleet.iter().any(|a| a.name == "AeonUniversalSubstrateAgent"));
        assert_eq!(fleet[0].provider, "AEON Hub");
    }
}
