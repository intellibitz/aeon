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
    pub const TOPOLOGY_MD: &'static str = include_str!("../../.agents/TOPOLOGY.md");

    pub const AGENT_RULES: &[AeonAxiomRule] = GEN_AGENT_RULES;
    pub const ENGINE_AXIOMS: &[AeonAxiomRule] = GEN_ENGINE_AXIOMS;
    pub const DEPLOYMENT_RULES: &[AeonAxiomRule] = GEN_DEPLOYMENT_RULES;
    pub const RUNTIME_MANDATES: &[AeonAxiomRule] = GEN_RUNTIME_MANDATES;
    pub const TEST_PROTOCOLS: &[AeonAxiomRule] = GEN_TEST_PROTOCOLS;
    pub const RULES: &[AeonAxiomRule] = GEN_RULES;

    pub const COMPONENTS: &[AeonComponentSpec] = GEN_COMPONENTS;
    pub const META_COMPONENTS: &[AeonComponentSpec] = GEN_META_COMPONENTS;
    pub const META_CONTEXTS: &[AeonComponentSpec] = GEN_META_CONTEXTS;

    #[allow(dead_code)]
    pub fn inspect_compiled_binary_instructions() -> String {
        format!(
            "AEON Substrate Compiled Binary Instructions:\n- Version: {}\n- Paradigm: {}\n- Hardcoded Axiom Rules: {}\n- Baked Native Components: {}\n- Orchestrated Meta Components: {}\n- Orchestrated Meta Contexts: {}",
            Self::VERSION,
            Self::CORE_PARADIGM,
            Self::RULES.len(),
            Self::COMPONENTS.len(),
            Self::META_COMPONENTS.len(),
            Self::META_CONTEXTS.len()
        )
    }
}
