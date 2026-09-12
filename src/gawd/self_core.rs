// AEON Core Substrate: Compiled Binary Instructions Core
// Eliminates runtime string parsing by encoding axioms, agent rules, and component topologies
// directly into strongly-typed compiled Rust data structures and enums.

#[derive(Debug, Clone, Copy)]
pub enum AeonCoreTier {
    Tier0Reflex,
    Tier1Swarm,
    Tier2Reasoning,
}

#[derive(Debug, Clone)]
pub struct AeonComponentSpec {
    pub name: &'static str,
    pub tier: AeonCoreTier,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct AeonAxiomRule {
    pub id: usize,
    pub title: &'static str,
    pub imperative: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/generated_axioms.rs"));

pub struct AlphaSelf;

impl AlphaSelf {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const CORE_PARADIGM: &'static str = "EAI (Exponential Intelligence for Any AI) - Intelligence Reflex & Execution Substrate";

    pub const AGENTS_MD: &'static str = include_str!("../../.agents/AGENTS.md");
    pub const BUILD_MD: &'static str = include_str!("../../.agents/BUILD.md");
    pub const ASPIRATIONS_MD: &'static str = include_str!("../../.agents/ASPIRATIONS.md");
    pub const RUNTIME_MD: &'static str = include_str!("../../.agents/RUNTIME.md");
    pub const TESTS_MD: &'static str = include_str!("../../.agents/TESTS.md");
    pub const MOTIONS_MD: &'static str = include_str!("../../.agents/MOTIONS.md");
    pub const TOPOLOGY_MD: &'static str = include_str!("../../.agents/TOPOLOGY.md");
    pub const WORKFLOW_MD: &'static str = include_str!("../../.agents/WORKFLOW.md");
    pub const MISSIONS_MD: &'static str = include_str!("../../.agents/MISSIONS.md");
    pub const QUERIES_MD: &'static str = include_str!("../../.agents/QUERIES.md");
    pub const CREATORS_MD: &'static str = include_str!("../../.agents/CREATORS.md");

    pub const RULES: &[AeonAxiomRule] = GEN_RULES;
    pub const WORKFLOW_STEPS: &[AeonAxiomRule] = GEN_WORKFLOW_STEPS;
    pub const TEST_PROTOCOLS: &[AeonAxiomRule] = GEN_TEST_PROTOCOLS;
    pub const MISSION_PROTOCOLS: &[AeonAxiomRule] = GEN_MISSION_PROTOCOLS;
    pub const QUERY_PROTOCOLS: &[AeonAxiomRule] = GEN_QUERY_PROTOCOLS;
    pub const CREATOR_PROTOCOLS: &[AeonAxiomRule] = GEN_CREATOR_PROTOCOLS;

    // 5 Pillar Component Topology
    pub const AOA_COMPONENTS: &[AeonComponentSpec] = GEN_AOA_COMPONENTS;
    pub const AGENT_COMPONENTS: &[AeonComponentSpec] = GEN_AGENT_COMPONENTS;
    pub const ENGINE_COMPONENTS: &[AeonComponentSpec] = GEN_ENGINE_COMPONENTS;
    pub const MODEL_COMPONENTS: &[AeonComponentSpec] = GEN_MODEL_COMPONENTS;
    pub const MCP_COMPONENTS: &[AeonComponentSpec] = GEN_MCP_COMPONENTS;
    pub const COMPONENTS: &[AeonComponentSpec] = GEN_COMPONENTS;

    #[allow(dead_code)]
    pub fn inspect_compiled_binary_instructions() -> String {
        format!(
            "AEON Substrate Compiled Binary Instructions:\n- Version: {}\n- Paradigm: {}\n- Hardcoded Axiom Rules: {}\n- AoA Pillar: {}\n- Agents Pillar: {}\n- Engines Pillar: {}\n- Models Pillar: {}\n- MCPs Pillar: {}",
            Self::VERSION,
            Self::CORE_PARADIGM,
            Self::RULES.len(),
            Self::AOA_COMPONENTS.len(),
            Self::AGENT_COMPONENTS.len(),
            Self::ENGINE_COMPONENTS.len(),
            Self::MODEL_COMPONENTS.len(),
            Self::MCP_COMPONENTS.len()
        )
    }
}
