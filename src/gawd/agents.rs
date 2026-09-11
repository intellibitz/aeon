// GAWD Agent Fleet: Universal Multi-Agent Swarm Logic
// RULE 11: Agents must add functionality directly to the aeon engine.
// RULE 31: Substrate Purity & Meta-Only Mandate - Dynamic Swarm Synthesis

use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
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

/// 🧠 Mission Blackboard: Shared state for swarm agents to converge on the "Chain of Truth".
pub type MissionBlackboard = Arc<Mutex<HashMap<String, String>>>;

/// Core Intelligence Trait for AEON Swarm Agents
pub trait GawdAgent: Send + Sync {
    fn name(&self) -> String;
    fn execute(&self, goal: &str, workspace: &Path, blackboard: &MissionBlackboard) -> EaiResult<String>;
}

/// 🚀 Dynamic Agent: A generic substrate agent that loads behavior from models and tools.
pub struct DynamicAgent {
    pub agent_name: String,
    pub mission_profile: String,
}

impl GawdAgent for DynamicAgent {
    fn name(&self) -> String { self.agent_name.clone() }
    fn execute(&self, goal: &str, workspace: &Path, blackboard: &MissionBlackboard) -> EaiResult<String> {
        let prompt = format!(
            "AGENT_ROLE: {}\nMISSION_PROFILE: {}\nGOAL: {}\n\n[INSTRUCTION]: Fulfill your role in the swarm. Use current blackboard state if available.",
            self.agent_name, self.mission_profile, goal
        );

        let ws = workspace.to_path_buf();
        if let Ok(res) = crate::gemi::pulse::AeonPulse::reason(&prompt, &ws) {
            let mut bb = blackboard.lock().unwrap();
            bb.insert(self.agent_name.clone(), res.clone());
            Ok(res)
        } else {
            Ok(format!("Agent {} active on goal: {}", self.agent_name, goal))
        }
    }
}

pub struct GawdAgentFleet;

impl GawdAgentFleet {
    /// 🧪 Fleet Synthesizer: Dynamically decides which agents are required for a mission.
    pub fn synthesize_fleet(goal: &str) -> Vec<Arc<dyn GawdAgent>> {
        let mut fleet: Vec<Arc<dyn GawdAgent>> = Vec::new();

        // 1. Mandatory Substrate Guards
        fleet.push(Arc::new(DynamicAgent {
            agent_name: "SafetyAgent".into(),
            mission_profile: "Governance and destruction detection.".into()
        }));
        fleet.push(Arc::new(DynamicAgent {
            agent_name: "ContextAgent".into(),
            mission_profile: "Workspace analysis and file-system awareness.".into()
        }));

        // 2. Intent-Driven Capability Activation (Rule 31)
        let lower_goal = goal.to_lowercase();
        if lower_goal.contains("git") || lower_goal.contains("code") || lower_goal.contains("rust") {
            fleet.push(Arc::new(DynamicAgent {
                agent_name: "DevOpsAgent".into(),
                mission_profile: "Software engineering and systems architecture.".into()
            }));
        }

        if lower_goal.contains("soil") || lower_goal.contains("crop") || lower_goal.contains("nutrient") {
            fleet.push(Arc::new(DynamicAgent {
                agent_name: "AgriTechAgent".into(),
                mission_profile: "Precision agriculture and nutrient management.".into()
            }));
        }

        // 3. Fallback Universal Reasoner
        if fleet.len() < 3 {
            fleet.push(Arc::new(DynamicAgent {
                agent_name: "UniversalReasoner".into(),
                mission_profile: "General-purpose logic and task fulfillment.".into()
            }));
        }

        fleet
    }

    pub fn dispatch_explosive_swarm(goal: String, workspace: PathBuf, blackboard: MissionBlackboard) -> Vec<(String, String)> {
        let agents = Self::synthesize_fleet(&goal);
        let mut results = Vec::new();
        let mut handles = Vec::new();

        for agent in agents {
            let g = goal.clone();
            let w = workspace.clone();
            let bb = Arc::clone(&blackboard);
            handles.push(std::thread::spawn(move || {
                let name = agent.name();
                let res = agent.execute(&g, &w, &bb).unwrap_or_else(|e| format!("Agent Execution Failed: {}", e));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_synthesis() {
        let fleet = GawdAgentFleet::synthesize_fleet("analyze soil and git status");
        assert!(!fleet.is_empty());
        assert!(fleet.iter().any(|a| a.name() == "AgriTechAgent"));
        assert!(fleet.iter().any(|a| a.name() == "DevOpsAgent"));
    }

    #[test]
    fn test_blackboard_convergence() {
        let bb = Arc::new(Mutex::new(HashMap::new()));
        let agent = DynamicAgent { agent_name: "TestAgent".into(), mission_profile: "Test".into() };
        let _ = agent.execute("test goal", Path::new("."), &bb);

        let data = bb.lock().unwrap();
        assert!(data.contains_key("TestAgent"));
    }
}
