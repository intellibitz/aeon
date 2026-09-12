use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_axioms.rs");

    let agents_md = fs::read_to_string(".agents/AGENTS.md").expect("Missing AGENTS.md");
    let aspirations_md = fs::read_to_string(".agents/ASPIRATIONS.md").expect("Missing ASPIRATIONS.md");
    let build_md = fs::read_to_string(".agents/BUILD.md").expect("Missing BUILD.md");
    let runtime_md = fs::read_to_string(".agents/RUNTIME.md").expect("Missing RUNTIME.md");
    let tests_md = fs::read_to_string(".agents/TESTS.md").expect("Missing TESTS.md");
    let topology_md = fs::read_to_string(".agents/TOPOLOGY.md").expect("Missing TOPOLOGY.md");

    let mut generated_code = String::new();

    // 1. AGENTS.md -> GEN_AGENT_RULES (1-15)
    generated_code.push_str("pub const GEN_AGENT_RULES: &[AeonAxiomRule] = &[\n");
    for line in agents_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0, rule.1, rule.2));
        }
    }
    generated_code.push_str("];\n\n");

    // 2. ASPIRATIONS.md -> GEN_ENGINE_AXIOMS (20-39)
    generated_code.push_str("pub const GEN_ENGINE_AXIOMS: &[AeonAxiomRule] = &[\n");
    let mut current_id = None;
    let mut current_title = None;
    for line in aspirations_md.lines() {
        if line.starts_with("### [Aspiration") {
            if let Some(caps) = parse_aspiration_header(line) {
                current_id = Some(caps.0);
                current_title = Some(caps.1);
            }
        } else if line.trim().starts_with("* **Core Paradigm**:") {
            if let (Some(id), Some(title)) = (current_id, current_title.take()) {
                let paradigm = line.splitn(2, ':').nth(1).unwrap().trim();
                generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", id + 20, title, paradigm));
                current_id = None;
            }
        }
    }
    generated_code.push_str("];\n\n");

    // 3. BUILD.md -> GEN_DEPLOYMENT_RULES (30-49)
    generated_code.push_str("pub const GEN_DEPLOYMENT_RULES: &[AeonAxiomRule] = &[\n");
    for line in build_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 30, rule.1, rule.2));
        }
    }
    generated_code.push_str("];\n\n");

    // 4. RUNTIME.md -> GEN_RUNTIME_MANDATES (40-69)
    generated_code.push_str("pub const GEN_RUNTIME_MANDATES: &[AeonAxiomRule] = &[\n");
    for line in runtime_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 40, rule.1, rule.2));
        }
    }
    generated_code.push_str("];\n\n");

    // 5. TESTS.md -> GEN_TEST_PROTOCOLS (60-89)
    generated_code.push_str("pub const GEN_TEST_PROTOCOLS: &[AeonAxiomRule] = &[\n");
    for line in tests_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 60, rule.1, rule.2));
        }
    }
    generated_code.push_str("];\n\n");

    // 6. TOPOLOGY.md -> Pillar-based Components
    let mut current_pillar = "";
    generated_code.push_str("pub const GEN_AOA_COMPONENTS: &[AeonComponentSpec] = &[\n");
    let mut agents = String::from("pub const GEN_AGENT_COMPONENTS: &[AeonComponentSpec] = &[\n");
    let mut engines = String::from("pub const GEN_ENGINE_COMPONENTS: &[AeonComponentSpec] = &[\n");
    let mut models = String::from("pub const GEN_MODEL_COMPONENTS: &[AeonComponentSpec] = &[\n");
    let mut mcps = String::from("pub const GEN_MCP_COMPONENTS: &[AeonComponentSpec] = &[\n");

    for line in topology_md.lines() {
        if line.starts_with("## 1. Agent of Agents") { current_pillar = "aoa"; }
        else if line.starts_with("## 2. Agents") { current_pillar = "agents"; }
        else if line.starts_with("## 3. Engines") { current_pillar = "engines"; }
        else if line.starts_with("## 4. Models") { current_pillar = "models"; }
        else if line.starts_with("## 5. MCPs") { current_pillar = "mcps"; }

        if let Some(comp) = parse_topology_item(line) {
            let tier = match comp.2.as_str() {
                "0" => "AeonCoreTier::Tier0Reflex",
                "2" => "AeonCoreTier::Tier2Reasoning",
                _ => "AeonCoreTier::Tier1Swarm",
            };
            let entry = format!("    AeonComponentSpec {{ name: {:?}, tier: {}, description: {:?} }},\n", comp.0, tier, comp.1);
            match current_pillar {
                "aoa" => generated_code.push_str(&entry),
                "agents" => agents.push_str(&entry),
                "engines" => engines.push_str(&entry),
                "models" => models.push_str(&entry),
                "mcps" => mcps.push_str(&entry),
                _ => {}
            }
        }
    }
    generated_code.push_str("];\n\n");
    agents.push_str("];\n\n");
    engines.push_str("];\n\n");
    models.push_str("];\n\n");
    mcps.push_str("];\n\n");
    generated_code.push_str(&agents);
    generated_code.push_str(&engines);
    generated_code.push_str(&models);
    generated_code.push_str(&mcps);

    // Combined COMPONENTS for legacy support
    generated_code.push_str("pub const GEN_COMPONENTS: &[AeonComponentSpec] = &[\n");
    for line in topology_md.lines() {
        if let Some(comp) = parse_topology_item(line) {
            let tier = match comp.2.as_str() {
                "0" => "AeonCoreTier::Tier0Reflex",
                "2" => "AeonCoreTier::Tier2Reasoning",
                _ => "AeonCoreTier::Tier1Swarm",
            };
            generated_code.push_str(&format!("    AeonComponentSpec {{ name: {:?}, tier: {}, description: {:?} }},\n", comp.0, tier, comp.1));
        }
    }
    generated_code.push_str("];\n\n");

    // 7. Unified RULES List
    generated_code.push_str("pub const GEN_RULES: &[AeonAxiomRule] = &[\n");
    for line in agents_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0, rule.1, rule.2));
        }
    }
    let mut current_id = None;
    let mut current_title = None;
    for line in aspirations_md.lines() {
        if line.starts_with("### [Aspiration") {
            if let Some(caps) = parse_aspiration_header(line) {
                current_id = Some(caps.0);
                current_title = Some(caps.1);
            }
        } else if line.trim().starts_with("* **Core Paradigm**:") {
            if let (Some(id), Some(title)) = (current_id, current_title.take()) {
                let paradigm = line.splitn(2, ':').nth(1).unwrap().trim();
                generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", id + 20, title, paradigm));
                current_id = None;
            }
        }
    }
    for line in build_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 30, rule.1, rule.2));
        }
    }
    for line in runtime_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 40, rule.1, rule.2));
        }
    }
    for line in tests_md.lines() {
        if let Some(rule) = parse_list_item(line) {
            generated_code.push_str(&format!("    AeonAxiomRule {{ id: {}, title: {:?}, imperative: {:?} }},\n", rule.0 + 60, rule.1, rule.2));
        }
    }
    generated_code.push_str("];\n");

    fs::write(&dest_path, generated_code).unwrap();

    println!("cargo:rerun-if-changed=.agents/AGENTS.md");
    println!("cargo:rerun-if-changed=.agents/ASPIRATIONS.md");
    println!("cargo:rerun-if-changed=.agents/BUILD.md");
    println!("cargo:rerun-if-changed=.agents/RUNTIME.md");
    println!("cargo:rerun-if-changed=.agents/TESTS.md");
    println!("cargo:rerun-if-changed=.agents/TOPOLOGY.md");
}

fn parse_list_item(line: &str) -> Option<(usize, String, String)> {
    let line = line.trim();
    if line.is_empty() || !line.chars().next().unwrap().is_digit(10) { return None; }
    let parts: Vec<&str> = line.splitn(2, '.').collect();
    if parts.len() < 2 { return None; }
    let id: usize = parts[0].parse().ok()?;
    let content = parts[1].trim();

    // Check if it's a rule item with bold title
    if content.starts_with("**") {
        let sub_parts: Vec<&str> = content.splitn(2, ':').collect();
        if sub_parts.len() < 2 { return None; }
        let title = sub_parts[0].trim_matches('*').trim();
        let imperative = sub_parts[1].trim();
        return Some((id, title.to_string(), imperative.to_string()));
    }

    // Check if it's a validation protocol item with simple title
    let sub_parts: Vec<&str> = content.splitn(2, ':').collect();
    if sub_parts.len() >= 2 {
        let title = sub_parts[0].trim();
        let imperative = sub_parts[1].trim();
        return Some((id, title.to_string(), imperative.to_string()));
    }

    None
}

fn parse_aspiration_header(line: &str) -> Option<(usize, String)> {
    let line = line.trim_start_matches('#').trim();
    if !line.starts_with("[Aspiration") { return None; }
    let parts: Vec<&str> = line.splitn(2, ']').collect();
    if parts.len() < 2 { return None; }
    let id: usize = parts[0].trim_start_matches("[Aspiration").trim().parse().ok()?;
    let title = parts[1].trim();
    Some((id, title.to_string()))
}

fn parse_topology_item(line: &str) -> Option<(String, String, String)> {
    let line = line.trim();
    if line.is_empty() || !line.chars().next().unwrap().is_digit(10) { return None; }
    let parts: Vec<&str> = line.splitn(2, '.').collect();
    if parts.len() < 2 { return None; }
    let content = parts[1].trim();
    let sub_parts: Vec<&str> = content.splitn(2, ':').collect();
    if sub_parts.len() < 2 { return None; }
    let name = sub_parts[0].trim_matches('*').trim();
    let desc_tier: Vec<&str> = sub_parts[1].splitn(2, "(Tier:").collect();
    let description = desc_tier[0].trim();
    let tier = if desc_tier.len() > 1 { desc_tier[1].trim_end_matches(')').trim() } else { "1" };
    Some((name.to_string(), description.to_string(), tier.to_string()))
}
