use crate::structs::DFAGraph; // 필요한 struct 임포트

/// Generates the complete Arkworks circuit as a string in Rust.
pub fn gen_arkworks_allstr(
    dfa_graph: &DFAGraph,
    struct_name: &str,
    regex_str: &str,
    end_anchor: bool,
) -> String {
    let state_len = dfa_graph.states.len();

    let declarations = generate_declarations_arkworks(
        struct_name,
        regex_str,
        state_len,
        end_anchor,
    );

    let init_code = generate_init_code_arkworks(state_len);

    let transition_logic = generate_state_transition_logic_arkworks(dfa_graph, state_len, end_anchor);

    let accept_logic = generate_accept_logic_arkworks(dfa_graph, end_anchor);

    let final_code = [
        declarations,
        init_code,
        transition_logic,
        accept_logic,
    ].concat();

    final_code.join("\n")
}

fn generate_declarations_arkworks(
    struct_name: &str,
    regex_str: &str,
    state_len: usize,
    _end_anchor: bool,
) -> Vec<String> {
    let mut declarations = vec![
        "use ark_ff::PrimeField;".to_string(),
        "use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};".to_string(),
        "use ark_r1cs_std::alloc::AllocVar;".to_string(),
        "use ark_r1cs_std::boolean::Boolean;".to_string(),
        "use ark_r1cs_std::fields::fp::FpVar;".to_string(),
        "use ark_r1cs_std::eq::EqGadget;".to_string(),
        "".to_string(),
        format!("/// Regex: {}", regex_str.replace('\n', "\\n").replace('\r', "\\r")),
        format!("#[derive(Clone)]\nstruct {}<F: PrimeField> {{", struct_name),
        "\tinput: Vec<F>,".to_string(),
        "\tmax_len: usize,".to_string(),
        "}\n".to_string(),
        format!("impl<F: PrimeField> ConstraintSynthesizer<F> for {}<F> {{", struct_name),
        "\tfn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {".to_string(),
    ];

    declarations.push("\t\t// Add state initialization logic here".to_string());

    declarations
}

fn generate_init_code_arkworks(state_len: usize) -> Vec<String> {
    vec![
        "\t\t// Initialize input variables".to_string(),
        "\t\tlet input_vars = self.input".to_string(),
        "\t\t\t.into_iter()".to_string(),
        "\t\t\t.map(|v| FpVar::new_input(cs.clone(), || Ok(v)))".to_string(),
        "\t\t\t.collect::<Result<Vec<_>, _>>()?;".to_string(),
        "\t\tlet mut valid = Boolean::constant(true);".to_string(),
        "".to_string(),
        format!("\t\t// Initialize state variables ({} states)", state_len),
    ]
}

fn generate_state_transition_logic_arkworks(
    dfa_graph: &DFAGraph,
    _state_len: usize,
    end_anchor: bool,
) -> Vec<String> {
    let mut lines = vec![];

    // Calculate the minimum length based on the shortest path from the initial state to any accept state
    let min_length = calculate_min_length(&dfa_graph);
    let max_length = dfa_graph.states.len(); // 기본적으로 DFA의 모든 상태 수를 최대 길이로 설정

    lines.push(format!(
        "\t\tif input_vars.len() < {} || input_vars.len() > {} {{",
        min_length, max_length
    ));
    lines.push("\t\t\tvalid = Boolean::constant(false);".to_string());
    lines.push("\t\t} else {".to_string());

    // Initialize states: Start from the initial state (assumed to be state 0)
    lines.push("\t\t\tlet mut current_state = FpVar::constant(F::from(0u64));".to_string());
    lines.push("".to_string());

    // Iterate over the input variables and generate transition logic
    for i in 0..max_length {
        lines.push(format!("\t\t\t// Transition logic for input index {}", i));
        lines.push(format!("\t\t\tif input_vars.len() > {} {{", i));
        lines.push(format!("\t\t\t\tlet current_input = &input_vars[{}];", i));

        // Generate transition logic based on DFA graph for each state
        lines.push("\t\t\t\tlet mut next_state = None;".to_string());

        // For each state, define transitions based on the DFA graph
        for state in &dfa_graph.states {
            let from_state = state.state_id;
            for (&to_state, char_set) in &state.transitions {
                // Create transition checks for each edge
                for &byte in char_set {
                    lines.push(format!(
                        "\t\t\t\tif current_state.is_eq(&FpVar::constant(F::from({}u64)))? && current_input.is_eq(&FpVar::constant(F::from({}u64)))? {{",
                        from_state, byte
                    ));
                    lines.push(format!(
                        "\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));",
                        to_state
                    ));
                    lines.push("\t\t\t\t}".to_string());
                }
            }
        }

        // Update the current state if a valid transition exists
        lines.push("\t\t\t\tif let Some(next) = next_state {".to_string());
        lines.push("\t\t\t\t\tcurrent_state = next;".to_string());
        lines.push("\t\t\t\t} else {".to_string());
        lines.push("\t\t\t\t\tvalid = Boolean::constant(false);".to_string());
        lines.push("\t\t\t\t}".to_string());

        lines.push("\t\t\t}".to_string());
        lines.push("".to_string());
    }

    // Check final state acceptance if end anchor is required
    if end_anchor {
        lines.push("\t\t\t// Check if the final state is an accepting state with end anchor".to_string());
        let accept_states: Vec<_> = dfa_graph.states.iter()
            .filter(|s| s.state_type == "accept")
            .map(|s| s.state_id)
            .collect();
        for &accept_state in &accept_states {
            lines.push(format!(
                "\t\t\tvalid = valid.and(&current_state.is_eq(&FpVar::constant(F::from({}u64)))?)?;",
                accept_state
            ));
        }
    } else {
        lines.push("\t\t\t// No end anchor, check any valid accepting state".to_string());
        let accept_states: Vec<_> = dfa_graph.states.iter()
            .filter(|s| s.state_type == "accept")
            .map(|s| s.state_id)
            .collect();
        lines.push("\t\t\tlet mut is_accepting = Boolean::constant(false);".to_string());
        for &accept_state in &accept_states {
            lines.push(format!(
                "\t\t\tis_accepting = is_accepting.or(&current_state.is_eq(&FpVar::constant(F::from({}u64)))?)?;",
                accept_state
            ));
        }
        lines.push("\t\t\tvalid = valid.and(&is_accepting)?;".to_string());
    }

    lines.push("\t\t}".to_string());
    lines
}

/// Calculates the minimum path length from the initial state to any accepting state in the DFA.
///
/// # Arguments
///
/// * `dfa_graph` - A reference to the DFA graph.
///
/// # Returns
///
/// The minimum number of transitions required to reach an accepting state.
fn calculate_min_length(dfa_graph: &DFAGraph) -> usize {
    use std::collections::{HashSet, VecDeque};

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start from the initial state (state 0)
    queue.push_back((0, 0)); // (state_id, distance)

    while let Some((state_id, distance)) = queue.pop_front() {
        if visited.contains(&state_id) {
            continue;
        }
        visited.insert(state_id);

        // Check if this is an accepting state
        if dfa_graph.states[state_id].state_type == "accept" {
            return distance;
        }

        // Enqueue all transitions from the current state
        for &to_state in dfa_graph.states[state_id].transitions.keys() {
            if !visited.contains(&to_state) {
                queue.push_back((to_state, distance + 1));
            }
        }
    }

    // Return a high value if no accepting state is found (shouldn't happen if the DFA is valid)
    dfa_graph.states.len()
}


fn generate_accept_logic_arkworks(
    _dfa_graph: &DFAGraph,
    _end_anchor: bool,
) -> Vec<String> {
    vec![
        "\t\t// Acceptance logic".to_string(),
        "\t\t// Ensure the final state is an accepting state".to_string(),
        "\t\tvalid.enforce_equal(&Boolean::constant(true))?;".to_string(),
        "\t\tOk(())".to_string(),
        "\t}".to_string(),
        "}".to_string(),
    ]
}

