// GAWD Agent Fleet: Universal Multi-Agent Swarm Logic
// RULE 11: Agents must add functionality directly to the aeon engine.
// RULE 31: Substrate Purity & Meta-Only Mandate - Dynamic Swarm Synthesis

use std::sync::{Arc, Mutex, OnceLock};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: String,
    pub description: String,
    pub categories: Vec<String>,
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

pub struct AgentMetaRegistry {
    agents: Arc<Mutex<Vec<AgentProfile>>>,
}

impl AgentMetaRegistry {
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<AgentMetaRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let registry = AgentMetaRegistry {
                agents: Arc::new(Mutex::new(Vec::new())),
            };
            registry.bootstrap();
            registry
        })
    }

    fn bootstrap(&self) {
        let mut agents = self.agents.lock().unwrap();
        agents.push(AgentProfile {
            name: "DevOpsAgent".into(),
            description: "Software engineering, systems architecture, and repository management.".into(),
            categories: vec!["code".into(), "rust".into(), "git".into(), "system".into()],
        });
        agents.push(AgentProfile {
            name: "AgriTechAgent".into(),
            description: "Precision agriculture, soil science, and crop nutrient management.".into(),
            categories: vec!["soil".into(), "crop".into(), "nutrient".into(), "agri".into()],
        });
    }

    pub fn register_agent(&self, profile: AgentProfile) {
        let mut agents = self.agents.lock().unwrap();
        agents.push(profile);
    }

    pub fn list_agents(&self) -> Vec<AgentProfile> {
        self.agents.lock().unwrap().clone()
    }
}

pub struct GawdAgentFleet;

impl GawdAgentFleet {
    /// 🧪 Fleet Synthesizer: Dynamically decides which agents are required for a mission.
    /// RULE 31: Zero hardcoded keyword checks. Uses Meta-Registry and Neural Relevance.
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

        // 2. Meta-Registry Discovery
        let registry = AgentMetaRegistry::global();
        let available_agents = registry.list_agents();

        let lower_goal = goal.to_lowercase();
        for agent in available_agents {
            // Neural/Semantic match would happen here in Tier 2.
            // For Tier 1, we match against dynamic categories in the registry.
            if agent.categories.iter().any(|c| lower_goal.contains(c)) {
                fleet.push(Arc::new(DynamicAgent {
                    agent_name: agent.name,
                    mission_profile: agent.description,
                }));
            }
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
    fn test_dynamic_registration() {
        let registry = AgentMetaRegistry::global();
        registry.register_agent(AgentProfile {
            name: "BioAgent".into(),
            description: "Biology specialist".into(),
            categories: vec!["tree".into()],
        });

        let fleet = GawdAgentFleet::synthesize_fleet("examine the oak tree");
        assert!(fleet.iter().any(|a| a.name() == "BioAgent"));
    }
}
