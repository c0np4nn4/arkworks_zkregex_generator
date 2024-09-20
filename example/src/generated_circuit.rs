use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::eq::EqGadget;

/// Regex: a[bc]+d
#[derive(Clone)]
pub struct RegexCircuit<F: PrimeField> {
  pub input: Vec<F>,
  pub max_len: usize,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for RegexCircuit<F> {
  fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
    // Add state initialization logic here
    // Initialize and pad input variables
    let mut padded_input = self.input.clone();
    padded_input.resize(256, F::from(0u64));
    let input_vars = padded_input
      .into_iter()
      .map(|v| FpVar::new_input(cs.clone(), || Ok(v)))
      .collect::<Result<Vec<_>, _>>()?;
    let mut valid = Boolean::constant(true);

    // Initialize state variables (4 states)
    // 현재 상태 초기화
    let mut current_state = FpVar::constant(F::from(0u64));
    // 각 입력 인덱스에 대한 전이 로직
    for current_input in input_vars.iter() {
      let mut next_state = current_state.clone();
      // 패딩된 입력 처리 (0u64인 경우 상태 전이를 건너뜁니다)
      let is_padded = current_input.is_eq(&FpVar::constant(F::from(0u64)))?;
      let is_state_0 = current_state.is_eq(&FpVar::constant(F::from(0u64)))?;
      let cond_1 = is_state_0.and(&(current_input.is_eq(&FpVar::constant(F::from(97u64)))?))?;
      next_state = cond_1.select(&FpVar::constant(F::from(1u64)), &next_state)?;
      let is_state_1 = current_state.is_eq(&FpVar::constant(F::from(1u64)))?;
      let cond_2 = is_state_1.and(&(current_input.is_eq(&FpVar::constant(F::from(98u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(99u64)))?)?))?;
      next_state = cond_2.select(&FpVar::constant(F::from(2u64)), &next_state)?;
      let is_state_2 = current_state.is_eq(&FpVar::constant(F::from(2u64)))?;
      let cond_3 = is_state_2.and(&(current_input.is_eq(&FpVar::constant(F::from(98u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(99u64)))?)?))?;
      next_state = cond_3.select(&FpVar::constant(F::from(2u64)), &next_state)?;
      let cond_4 = is_state_2.and(&(current_input.is_eq(&FpVar::constant(F::from(100u64)))?))?;
      next_state = cond_4.select(&FpVar::constant(F::from(3u64)), &next_state)?;
      let is_state_3 = current_state.is_eq(&FpVar::constant(F::from(3u64)))?;
      let state_changed = current_state.is_eq(&next_state)?.not();
      valid = valid.and(&state_changed)?.or(&is_padded)?;
      current_state = next_state;
    }
    // Acceptance logic
    // Ensure the final state is an accepting state
    valid.enforce_equal(&Boolean::constant(true))?;
    Ok(())
  }
}