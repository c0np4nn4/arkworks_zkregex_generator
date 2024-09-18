use std::collections::BTreeSet;

// Helper function for the `+` regex rule: One or more repetitions
pub(crate) fn handle_plus_rule(
    from_state: usize,
    to_state: usize,
    char_set: &BTreeSet<u8>,
) -> Vec<String> {
    let mut lines = vec![];
    lines.push(format!(
        "\t\t\t\tif current_state.is_eq(&FpVar::constant(F::from({}u64)))? {{",
        from_state
    ));
    for &byte in char_set {
        lines.push(format!(
            "\t\t\t\t\tif current_input.is_eq(&FpVar::constant(F::from({}u64)))? {{",
            byte
        ));
        lines.push(format!(
            "\t\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));",
            to_state
        ));
        lines.push("\t\t\t\t\t}".to_string());
    }
    lines.push("\t\t\t\t}".to_string());
    lines
}

// Helper function for the `*` regex rule: Zero or more repetitions
pub(crate) fn handle_star_rule(
    from_state: usize,
    to_state: usize,
    char_set: &BTreeSet<u8>,
) -> Vec<String> {
    let mut lines = vec![];
    lines.push(format!(
        "\t\t\t\tif current_state.is_eq(&FpVar::constant(F::from({}u64)))? {{",
        from_state
    ));
    lines.push(format!(
        "\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));", // Allow staying in the same state
        from_state
    ));
    for &byte in char_set {
        lines.push(format!(
            "\t\t\t\t\tif current_input.is_eq(&FpVar::constant(F::from({}u64)))? {{",
            byte
        ));
        lines.push(format!(
            "\t\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));",
            to_state
        ));
        lines.push("\t\t\t\t\t}".to_string());
    }
    lines.push("\t\t\t\t}".to_string());
    lines
}

// Helper function for the `?` regex rule: Zero or one repetitions
pub(crate) fn handle_question_mark_rule(
    from_state: usize,
    to_state: usize,
    char_set: &BTreeSet<u8>,
) -> Vec<String> {
    let mut lines = vec![];
    lines.push(format!(
        "\t\t\t\tif current_state.is_eq(&FpVar::constant(F::from({}u64)))? {{",
        from_state
    ));
    lines.push(format!(
        "\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));", // Allow skipping the state
        to_state
    ));
    for &byte in char_set {
        lines.push(format!(
            "\t\t\t\t\tif current_input.is_eq(&FpVar::constant(F::from({}u64)))? {{",
            byte
        ));
        lines.push(format!(
            "\t\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));",
            to_state
        ));
        lines.push("\t\t\t\t\t}".to_string());
    }
    lines.push("\t\t\t\t}".to_string());
    lines
}

// Helper function for handling `[abc]` rule
pub(crate) fn handle_char_class_rule(
    from_state: usize,
    to_state: usize,
    char_set: &BTreeSet<u8>,
) -> Vec<String> {
    let mut lines = vec![];
    lines.push(format!(
        "\t\t\t\tif current_state.is_eq(&FpVar::constant(F::from({}u64)))? {{",
        from_state
    ));
    for &byte in char_set {
        lines.push(format!(
            "\t\t\t\t\tif current_input.is_eq(&FpVar::constant(F::from({}u64)))? {{",
            byte
        ));
        lines.push(format!(
            "\t\t\t\t\t\tnext_state = Some(FpVar::constant(F::from({}u64)));",
            to_state
        ));
        lines.push("\t\t\t\t\t}".to_string());
    }
    lines.push("\t\t\t\t}".to_string());
    lines
}

