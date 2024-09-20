use crate::{handlers::{handle_char_class_rule, handle_plus_rule, handle_question_mark_rule, handle_star_rule}, structs::DFAGraph}; // 필요한 struct 임포트

/// Generates the complete Arkworks circuit as a string in Rust.
pub fn gen_arkworks_allstr(
    dfa_graph: &DFAGraph,
    struct_name: &str,
    regex_str: &str,
    end_anchor: bool,
) -> String {
    let state_len = dfa_graph.states.len();

    // (1)
    let declarations = generate_declarations_arkworks(
        struct_name,
        regex_str,
        state_len,
        end_anchor,
    );

    // (2)
    let init_code = generate_init_code_arkworks(state_len, 256usize);

    // (3)
    let transition_logic = generate_state_transition_logic_arkworks(dfa_graph, state_len, end_anchor);

    // (4)
    let accept_logic = generate_accept_logic_arkworks(dfa_graph, end_anchor);

    let final_code = [
        declarations, //////// (1)
        init_code, /////////// (2)
        transition_logic, //// (3)
        accept_logic, //////// (4)
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
        "use ark_r1cs_std::fields::FieldVar;".to_string(),
        "use ark_r1cs_std::alloc::AllocVar;".to_string(),
        "use ark_r1cs_std::boolean::Boolean;".to_string(),
        "use ark_r1cs_std::fields::fp::FpVar;".to_string(),
        "use ark_r1cs_std::eq::EqGadget;".to_string(),
        "".to_string(),
        format!("/// Regex: {}", regex_str.replace('\n', "\\n").replace('\r', "\\r")),
        format!("#[derive(Clone)]\npub struct {}<F: PrimeField> {{", struct_name),
        format!("{}pub input: Vec<F>,", put_space(1)),
        format!("{}pub max_len: usize,", put_space(1)),
        "}\n".to_string(),
        format!("impl<F: PrimeField> ConstraintSynthesizer<F> for {}<F> {{", struct_name),
        format!("{}fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {{", put_space(1)),
    ];

    declarations.push(format!("{}// Add state initialization logic here", put_space(2)));

    declarations
}

fn generate_init_code_arkworks(state_len: usize, max_len: usize) -> Vec<String> {
    vec![
        format!("{}// Initialize and pad input variables", put_space(2)),
        format!("{}let mut padded_input = self.input.clone();", put_space(2)),
        format!("{}padded_input.resize({}, F::from(0u64));", put_space(2), max_len),
        format!("{}let input_vars = padded_input", put_space(2)),
        format!("{}  .into_iter()", put_space(2)),
        format!("{}  .map(|v| FpVar::new_input(cs.clone(), || Ok(v)))", put_space(2)),
        format!("{}  .collect::<Result<Vec<_>, _>>()?;", put_space(2)),
        format!("{}let mut valid = Boolean::constant(true);", put_space(2)),
        "".to_string(),
        format!("{}// Initialize state variables ({} states)", put_space(2), state_len),
    ]
}

// /// Generates the state transition logic for the Arkworks circuit in Rust.
// fn generate_state_transition_logic_arkworks(
//     dfa_graph: &DFAGraph,
//     _state_len: usize,
//     _end_anchor: bool,
// ) -> Vec<String> {
//     let mut lines = vec![];

//     // 현재 상태 변수 초기화 (초기 상태는 DFA의 시작 상태, 일반적으로 state_id 0)
//     lines.push(format!("{}// 현재 상태 초기화", put_space(2)));
//     lines.push(format!("{}let mut current_state = FpVar::constant(F::from(0u64));", put_space(2))); // 초기 상태 설정

//     // 기본 상태 전이 로직을 추가
//     lines.push(format!("{}// 각 입력 인덱스에 대한 전이 로직", put_space(2)));
//     lines.push(format!("{}for (index, current_input) in input_vars.iter().enumerate() {{", put_space(2)));
//     lines.push(format!("{}let is_padded = current_input.is_eq(&FpVar::constant(F::from(0u64)))?;", put_space(3)));
//     lines.push(format!("{}let cond_padded = is_padded.not();", put_space(3)));
//     lines.push(format!("{}let mut next_state = current_state.clone();", put_space(3)));

//     // DFA 그래프에서 각 상태와 전이에 대해 로직 생성
//     let mut condition_counter = 0; // 고유한 조건 변수명을 만들기 위한 카운터
//     for state in &dfa_graph.states {
//         let from_state = state.state_id;

//         // 현재 상태 조건 추가
//         lines.push(format!(
//             "\n{}let is_state_{} = current_state.is_eq(&FpVar::constant(F::from({}u64)))?;",
//             put_space(3),
//             from_state,
//             from_state
//         ));

//         for (&to_state, char_set) in &state.transitions {
//             // 각 문자의 조건을 추가 (변수명 중복 방지를 위해 고유한 이름 부여)
//             condition_counter += 1;
//             let condition_var = format!("cond_{}", condition_counter);

//             // 논리적 OR 처리를 위해 Arkworks 메서드 사용
//             let conditions = char_set
//                 .iter()
//                 .map(|&c| format!("current_input.is_eq(&FpVar::constant(F::from({}u64)))?", c))
//                 .collect::<Vec<_>>();

//             // 여러 조건을 하나로 합치는 논리 연산 처리
//             let or_conditions = if conditions.len() > 1 {
//                 let mut combined_condition = format!("{}", conditions[0]);
//                 for condition in &conditions[1..] {
//                     combined_condition = format!(
//                         "{}.or(&{})?", 
//                         combined_condition, 
//                         condition
//                     );
//                 }
//                 combined_condition
//             } else {
//                 conditions[0].clone() // 하나의 조건일 때는 그냥 조건 그대로 사용
//             };

//             lines.push(format!(
//                 "{}let {} = is_state_{}.and(&({}))?;", 
//                 put_space(3),
//                 condition_var, 
//                 from_state, 
//                 or_conditions
//             ));

//             // 상태 전이 로직 추가
//             lines.push(format!(
//                 "{}next_state = {}.select(&FpVar::constant(F::from({}u64)), &next_state)?;",
//                 put_space(3),
//                 condition_var,
//                 to_state
//             ));
//         }
//     }

//     // 상태가 변경되었는지 확인 (패딩이 아닐 때만 체크)
//     lines.push(format!(
//         "{}valid = cond_padded.select(&valid.and(&current_state.is_eq(&next_state)?.not())?, &valid)?;", 
//         put_space(3)
//     ));

//     // 다음 상태로 업데이트
//     lines.push(format!("{}current_state = next_state;", put_space(3)));
//     lines.push(format!("{}}}", put_space(2)));

//     lines
// }

// /// Generates the acceptance logic for the Arkworks circuit in Rust.
// fn generate_accept_logic_arkworks(
//     dfa_graph: &DFAGraph,
//     _end_anchor: bool,
// ) -> Vec<String> {
//     let mut lines = vec![];

//     // 수락 상태가 있는지 확인
//     lines.push(format!("{}// Acceptance logic", put_space(2)));
//     lines.push(format!("{}let mut is_accepting = Boolean::constant(false);", put_space(2)));

//     let accept_states: Vec<_> = dfa_graph
//         .states
//         .iter()
//         .filter(|s| s.state_type == "accept")
//         .map(|s| s.state_id)
//         .collect();

//     // 수락 상태 중 하나라도 일치하면 수락
//     for &accept_state in &accept_states {
//         lines.push(format!(
//             "{}is_accepting = is_accepting.or(&current_state.is_eq(&FpVar::constant(F::from({}u64)))?)?;",
//             put_space(2),
//             accept_state
//         ));
//     }

//     lines.push(format!("{}valid = valid.and(&is_accepting)?;", put_space(2)));
//     lines.push(format!("{}valid.enforce_equal(&Boolean::constant(true))?;", put_space(2)));
//     lines.push(format!("{}Ok(())", put_space(2)));
//     lines.push(format!("{}}}}}", put_space(1)));

//     lines
// }
/// Generates the state transition logic for the Arkworks circuit in Rust.
fn generate_state_transition_logic_arkworks(
    dfa_graph: &DFAGraph,
    _state_len: usize,
    _end_anchor: bool,
) -> Vec<String> {
    let mut lines = vec![];

    // 현재 상태 변수 초기화 (초기 상태는 DFA의 시작 상태, 일반적으로 state_id 0)
    lines.push(format!("{}// 현재 상태 초기화", put_space(2)));
    lines.push(format!("{}let mut current_state = FpVar::constant(F::from(0u64));", put_space(2))); // 초기 상태 설정

    // 기본 상태 전이 로직을 추가
    lines.push(format!("{}// 각 입력 인덱스에 대한 전이 로직", put_space(2)));
    lines.push(format!("{}for (index, current_input) in input_vars.iter().enumerate() {{", put_space(2)));
    lines.push(format!("{}let is_padded = current_input.is_eq(&FpVar::constant(F::from(0u64)))?;", put_space(3)));
    lines.push(format!("{}let cond_padded = is_padded.not();", put_space(3)));
    lines.push(format!("{}let mut next_state = current_state.clone();", put_space(3)));
    lines.push(format!("{}let mut has_transitioned = Boolean::constant(false);", put_space(3)));

    // DFA 그래프에서 각 상태와 전이에 대해 로직 생성
    let mut condition_counter = 0; // 고유한 조건 변수명을 만들기 위한 카운터
    for state in &dfa_graph.states {
        let from_state = state.state_id;

        // 현재 상태 조건 추가
        lines.push(format!(
            "{}let is_state_{} = current_state.is_eq(&FpVar::constant(F::from({}u64)))?;",
            put_space(3),
            from_state,
            from_state
        ));

        for (&to_state, char_set) in &state.transitions {
            // 각 문자의 조건을 추가 (변수명 중복 방지를 위해 고유한 이름 부여)
            condition_counter += 1;
            let condition_var = format!("cond_{}", condition_counter);

            // 논리적 OR 처리를 위해 Arkworks 메서드 사용
            let conditions = char_set
                .iter()
                .map(|&c| format!("current_input.is_eq(&FpVar::constant(F::from({}u64)))?", c))
                .collect::<Vec<_>>();

            // 여러 조건을 하나로 합치는 논리 연산 처리
            let or_conditions = if conditions.len() > 1 {
                let mut combined_condition = format!("{}", conditions[0]);
                for condition in &conditions[1..] {
                    combined_condition = format!(
                        "{}.or(&{})?", 
                        combined_condition, 
                        condition
                    );
                }
                combined_condition
            } else {
                conditions[0].clone() // 하나의 조건일 때는 그냥 조건 그대로 사용
            };

            lines.push(format!(
                "{}let {} = is_state_{}.and(&({}))?;", 
                put_space(3),
                condition_var, 
                from_state, 
                or_conditions
            ));

            // 상태 전이 로직 추가
            lines.push(format!(
                "{}next_state = {}.select(&FpVar::constant(F::from({}u64)), &next_state)?;",
                put_space(3),
                condition_var,
                to_state
            ));

            lines.push(format!("{}has_transitioned = has_transitioned.or(&{})?;", put_space(3), condition_var));
        }
    }

    // 유효하지 않은 전이 확인 (패딩이 아닐 때)
    lines.push(format!(
        "{}let invalid_transition = cond_padded.and(&has_transitioned.not())?;", put_space(3)
    ));
    lines.push(format!("{}valid = valid.and(&invalid_transition.not())?;", put_space(3)));

    // 다음 상태로 업데이트
    lines.push(format!("{}current_state = next_state;", put_space(3)));
    lines.push(format!("{}}}", put_space(2)));

    lines
}

/// Generates the acceptance logic for the Arkworks circuit in Rust.
fn generate_accept_logic_arkworks(
    dfa_graph: &DFAGraph,
    _end_anchor: bool,
) -> Vec<String> {
    let mut lines = vec![];

    // 수락 상태가 있는지 확인
    lines.push(format!("{}// Acceptance logic", put_space(2)));
    lines.push(format!("{}let is_accepting = current_state.is_eq(&FpVar::constant(F::from(3u64)))?;", put_space(2)));
    lines.push(format!("{}valid = valid.and(&is_accepting)?;", put_space(2)));
    lines.push(format!("{}valid.enforce_equal(&Boolean::constant(true))?;", put_space(2)));
    lines.push(format!("{}Ok(())", put_space(2)));
    lines.push(format!("{}}}}}", put_space(1)));

    lines
}
/// Returns a string with two spaces for each level of indentation.
fn put_space(indent_level: usize) -> String {
    "  ".repeat(indent_level)  // 두 칸의 공백을 indent_level 만큼 반복하여 생성
}

