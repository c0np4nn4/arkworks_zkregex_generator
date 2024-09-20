use crate::{
    errors::CompilerError,
    structs::{
        DFAGraph, DFAGraphInfo, DFAStateInfo, DFAStateNode, RegexAndDFA,
        SubstringDefinitions, SubstringDefinitionsJson,
    },
};
use regex::Regex;
use regex_automata::dfa::{
    dense::DFA,
    StartKind,
};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    num::ParseIntError,
};

/// Parses DFA states from a string output and populates a `DFAGraphInfo` structure.
///
/// # Arguments
///
/// * `output` - A string slice containing the DFA state information.
/// * `dfa_info` - A mutable reference to a `DFAGraphInfo` to be populated with parsed states.
///
/// # Returns
///
/// A `Result` containing `()` if parsing is successful, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Uses regex to match state definitions and transitions in the input string.
/// - Iterates over state matches, creating `DFAStateInfo` objects for each state.
/// - Parses transitions for each state and adds them to the state's edges.
/// - Populates `dfa_info.states` with the parsed states.
fn parse_states(output: &str, dfa_info: &mut DFAGraphInfo) -> Result<(), CompilerError> {
    let state_re = Regex::new(r"\*?(\d+): ((.+?) => (\d+),?)+")?;
    let transition_re = Regex::new(
        r"\s+[^=]+\s*=>\s*(\d+)+\s*|\s+=+\s*=>\s*(\d+)+|\s+=-[^=]+=>\s*\s*(\d+)+\s*|\s+[^=]+-=\s*=>\s*(\d+)+\s*",
    )?;

    for captures in state_re.captures_iter(output) {
        let src = captures[1]
            .parse::<usize>()
            .map_err(|_| CompilerError::ParseError("Failed to parse state ID".to_string()))?;

        let mut state = DFAStateInfo {
            source: src,
            typ: if captures[0].starts_with('*') {
                "accept".to_string()
            } else {
                String::new()
            },
            edges: BTreeMap::new(),
        };

        for transition in transition_re.captures_iter(&captures[0]) {
            parse_transition(&mut state, &transition[0])?;
        }

        dfa_info.states.push(state);
    }

    Ok(())
}

/// Parses a single transition from a string and adds it to the DFA state.
///
/// # Arguments
///
/// * `state` - A mutable reference to the `DFAStateInfo` to which the transition will be added.
/// * `transition` - A string slice containing the transition information.
///
/// # Returns
///
/// A `Result` containing `()` if parsing is successful, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Splits the transition string into source and destination parts.
/// - Processes the source string to handle special character cases.
/// - Parses the destination as a usize.
/// - Adds the parsed transition to the state's edges.
fn parse_transition(state: &mut DFAStateInfo, transition: &str) -> Result<(), CompilerError> {
    let parts: Vec<&str> = transition.split("=>").collect();
    if parts.len() != 2 {
        return Err(CompilerError::ParseError(
            "Invalid transition format".to_string(),
        ));
    }

    let mut src = parts[0].trim().to_string();
    if src.len() > 2 && src.chars().nth(2) == Some('\\') && src.chars().nth(3) != Some('x') {
        src = format!("{}{}", &src[0..2], &src[3..]);
    }

    let dst = parts[1]
        .trim()
        .parse::<usize>()
        .map_err(|_| CompilerError::ParseError("Failed to parse destination state".to_string()))?;

    state.edges.insert(src, dst);
    Ok(())
}

/// Processes EOI (End of Input) transitions in the DFA graph.
///
/// Removes EOI transitions and marks their source states as accept states.
fn handle_eoi_transitions(dfa_info: &mut DFAGraphInfo) {
    for state in &mut dfa_info.states {
        if let Some(_) = state.edges.get("EOI") {
            state.typ = String::from("accept");
            state.edges.remove("EOI");
        }
    }
}

/// Finds the start state in the DFA output string.
///
/// # Arguments
///
/// * `output` - A string slice containing the DFA output.
///
/// # Returns
///
/// A `Result` containing the start state ID as `usize`, or a `CompilerError` if not found.
fn find_start_state(output: &str) -> Result<usize, CompilerError> {
    let start_state_re = Regex::new(r"START-GROUP\(anchored\)[\s*\w*\=>]*Text => (\d+)")?;
    start_state_re
        .captures(output)
        .and_then(|cap| cap[1].parse::<usize>().ok())
        .ok_or_else(|| CompilerError::ParseError("Failed to find start state".to_string()))
}

/// Sorts and renames states in a DFA graph, starting from a given start state.
///
/// # Arguments
///
/// * `dfa_info` - A reference to the original `DFAGraphInfo`.
/// * `start_state` - The ID of the start state.
///
/// # Returns
///
/// A new `DFAGraphInfo` with sorted and renamed states.
///
/// # Function Behavior
///
/// 1. Performs a Breadth-First Search (BFS) to sort states, starting from the start state.
/// 2. Creates a mapping of old state IDs to new state IDs.
/// 3. Renames states and updates their edges according to the new mapping.
fn sort_and_rename_states(dfa_info: &DFAGraphInfo, start_state: usize) -> DFAGraphInfo {
    let mut sorted_states = Vec::new();
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([start_state]);

    // BFS to sort states
    while let Some(state_id) = queue.pop_front() {
        if visited.insert(state_id) {
            if let Some(state) = dfa_info.states.iter().find(|s| s.source == state_id) {
                sorted_states.push(state.clone());
                queue.extend(state.edges.values().filter(|&dst| !visited.contains(dst)));
            }
        }
    }

    // Create mapping of old state IDs to new state IDs
    let state_map: BTreeMap<_, _> = sorted_states
        .iter()
        .enumerate()
        .map(|(new_id, state)| (state.source, new_id))
        .collect();

    // Rename states and update edges
    let renamed_states = sorted_states
        .into_iter()
        .enumerate()
        .map(|(new_id, mut state)| {
            state.source = new_id;
            for dst in state.edges.values_mut() {
                *dst = *state_map.get(dst).unwrap_or(dst);
            }
            state
        })
        .collect();

    DFAGraphInfo {
        states: renamed_states,
    }
}

/// Creates a mapping of special character representations to their ASCII values.
///
/// # Returns
///
/// A `BTreeMap` where keys are string representations of special characters,
/// and values are their corresponding ASCII byte values.
fn create_special_char_mappings() -> BTreeMap<&'static str, u8> {
    [
        ("\\n", 10),
        ("\\r", 13),
        ("\\t", 9),
        ("\\v", 11),
        ("\\f", 12),
        ("\\0", 0),
        ("\\\"", 34),
        ("\\'", 39),
        ("\\", 92),
        ("' '", 32),
    ]
    .iter()
    .cloned()
    .collect()
}

/// Processes a range edge in the DFA graph, adding all characters in the range to the edge set.
///
/// # Arguments
///
/// * `key` - The string representation of the range transition (e.g., "a-z").
/// * `value` - The destination state ID.
/// * `edges` - A mutable reference to the map of edges.
/// * `special_char_mappings` - A reference to the special character mappings.
/// * `re` - A reference to the compiled Regex for parsing ranges.
///
/// # Returns
///
/// A `Result` containing `()` if successful, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Extracts start and end characters of the range using the provided regex.
/// - Parses start and end characters to their byte values.
/// - Adds all characters in the range to the edge set for the given destination state.
fn process_range_edge(
    key: &str,
    value: usize,
    edges: &mut BTreeMap<usize, BTreeSet<u8>>,
    special_char_mappings: &BTreeMap<&str, u8>,
    re: &Regex,
) -> Result<(), CompilerError> {
    let capture = re
        .captures(key)
        .ok_or_else(|| CompilerError::ParseError("Failed to capture range".to_string()))?;
    let start_index = parse_char(&capture[1], special_char_mappings)?;
    let end_index = parse_char(&capture[2], special_char_mappings)?;
    let char_range: Vec<u8> = (start_index..=end_index).collect();

    edges
        .entry(value)
        .or_insert_with(BTreeSet::new)
        .extend(char_range);
    Ok(())
}

/// Processes a single character edge in the DFA graph.
///
/// # Arguments
///
/// * `key` - The string representation of the character.
/// * `value` - The destination state ID.
/// * `edges` - A mutable reference to the map of edges.
/// * `special_char_mappings` - A reference to the special character mappings.
///
/// # Returns
///
/// A `Result` containing `()` if successful, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Parses the character to its byte value.
/// - Adds the byte to the edge set for the given destination state.
fn process_single_edge(
    key: &str,
    value: usize,
    edges: &mut BTreeMap<usize, BTreeSet<u8>>,
    special_char_mappings: &BTreeMap<&str, u8>,
) -> Result<(), CompilerError> {
    let index = parse_char(key, special_char_mappings)?;
    edges
        .entry(value)
        .or_insert_with(BTreeSet::new)
        .insert(index);
    Ok(())
}

/// Processes an edge in the DFA graph, handling both range and single character transitions.
///
/// # Arguments
///
/// * `key` - The string representation of the transition.
/// * `value` - The destination state ID.
/// * `edges` - A mutable reference to the map of edges.
/// * `special_char_mappings` - A reference to the special character mappings.
///
/// # Returns
///
/// A `Result` containing `()` if successful, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Checks if the key represents a range (e.g., "a-z") or a single character.
/// - Delegates to `process_range_edge` or `process_single_edge` accordingly.
fn process_edge(
    key: &str,
    value: usize,
    edges: &mut BTreeMap<usize, BTreeSet<u8>>,
    special_char_mappings: &BTreeMap<&str, u8>,
) -> Result<(), CompilerError> {
    let re = Regex::new(r"(.+)-(.+)")?;
    if re.is_match(key) {
        process_range_edge(key, value, edges, special_char_mappings, &re)?;
    } else {
        process_single_edge(key, value, edges, special_char_mappings)?;
    }
    Ok(())
}

/// Parses a character representation into its corresponding byte value.
///
/// # Arguments
///
/// * `s` - The string representation of the character.
/// * `special_char_mappings` - A reference to the special character mappings.
///
/// # Returns
///
/// A `Result` containing the parsed byte value, or a `CompilerError` if parsing fails.
///
/// # Function Behavior
///
/// - Handles hexadecimal representations (e.g., "\x41").
/// - Looks up special characters in the provided mappings.
/// - Converts single-character strings to their byte value.
/// - Returns an error for invalid inputs.
fn parse_char(s: &str, special_char_mappings: &BTreeMap<&str, u8>) -> Result<u8, CompilerError> {
    if s.starts_with("\\x") {
        u8::from_str_radix(&s[2..], 16)
            .map_err(|e: ParseIntError| CompilerError::ParseError(e.to_string()))
    } else if let Some(&value) = special_char_mappings.get(s) {
        Ok(value)
    } else if s.len() == 1 {
        Ok(s.as_bytes()[0])
    } else {
        Err(CompilerError::ParseError(format!(
            "Invalid character: {}",
            s
        )))
    }
}

/// Processes all edges for a state in the DFA graph.
///
/// # Arguments
///
/// * `state_edges` - A reference to a map of edge labels to destination state IDs.
///
/// # Returns
///
/// A `Result` containing a map of destination state IDs to sets of byte values,
/// or a `CompilerError` if processing fails.
///
/// # Function Behavior
///
/// - Creates special character mappings.
/// - Iterates over all edges, processing each one.
/// - Handles the special case of space character representation.
fn process_state_edges(
    state_edges: &BTreeMap<String, usize>,
) -> Result<BTreeMap<usize, BTreeSet<u8>>, CompilerError> {
    let mut edges = BTreeMap::new();
    let special_char_mappings = create_special_char_mappings();

    for (key, value) in state_edges {
        let key = if key == "' '" { " " } else { key };
        process_edge(key, *value, &mut edges, &special_char_mappings)?;
    }

    Ok(edges)
}

/// Converts a DFA (Deterministic Finite Automaton) to a DFAGraph structure.
///
/// # Arguments
///
/// * `dfa` - The DFA to convert.
///
/// # Returns
///
/// A `Result` containing the converted `DFAGraph`, or a `CompilerError` if conversion fails.
///
/// # Function Behavior
///
/// 1. Converts the DFA to a string representation.
/// 2. Parses states from the string representation.
/// 3. Handles EOI (End of Input) transitions.
/// 4. Finds the start state and sorts/renames states accordingly.
/// 5. Processes edges for each state and constructs the final graph.
fn convert_dfa_to_graph(dfa: DFA<Vec<u32>>) -> Result<DFAGraph, CompilerError> {
    let dfa_str = format!("{:?}", dfa);

    let mut dfa_info = DFAGraphInfo { states: Vec::new() };

    parse_states(&dfa_str, &mut dfa_info)?;

    handle_eoi_transitions(&mut dfa_info);

    let start_state = find_start_state(&dfa_str)?;
    dfa_info = sort_and_rename_states(&mut dfa_info, start_state);

    let mut graph = DFAGraph { states: Vec::new() };
    for state in &dfa_info.states {
        let edges = process_state_edges(&state.edges)?;
        graph.states.push(DFAStateNode {
            state_type: state.typ.clone(),
            state_id: state.source,
            transitions: edges,
        });
    }

    Ok(graph)
}

/// Creates a DFA graph from a regex string.
///
/// # Arguments
///
/// * `regex` - A string slice containing the regex pattern.
///
/// # Returns
///
/// A `Result` containing a `DFAGraph` or a `CompilerError`.
fn create_dfa_graph_from_regex(regex: &str) -> Result<DFAGraph, CompilerError> {
    let config = DFA::config()
        .minimize(true)
        .start_kind(StartKind::Anchored)
        .byte_classes(false)
        .accelerate(true);

    let dfa = DFA::builder()
        .configure(config)
        .build(&format!(r"^{}$", regex))
        .map_err(|e| CompilerError::BuildError {
            regex: regex.to_string(),
            source: e,
        })?;

    // println!("[1] dfa:   {:?}", dfa.clone());
    // println!("[2] graph: {:#?}", convert_dfa_to_graph(dfa.clone()));

    convert_dfa_to_graph(dfa)
}

/// Creates a `RegexAndDFA` from a regex string and substring definitions.
///
/// # Arguments
///
/// * `regex_str` - A string slice containing the regex pattern.
/// * `substrs_defs_json` - A `SubstringDefinitionsJson` object.
///
/// # Returns
///
/// A `Result` containing a `RegexAndDFA` or a `CompilerError`.
pub(crate) fn create_regex_and_dfa_from_str_and_defs(
    regex_str: &str,
    substrs_defs_json: SubstringDefinitionsJson,
) -> Result<RegexAndDFA, CompilerError> {
    let dfa = create_dfa_graph_from_regex(regex_str)?;

    let substring_ranges = substrs_defs_json
        .transitions
        .into_iter()
        .map(|transitions| {
            transitions
                .into_iter()
                .collect::<BTreeSet<(usize, usize)>>()
        })
        .collect();

    let substrings = SubstringDefinitions {
        substring_ranges,
        substring_boundaries: None,
    };

    Ok(RegexAndDFA {
        regex_pattern: regex_str.to_string(),
        dfa,
        has_end_anchor: regex_str.ends_with('$'),
        substrings,
    })
}
