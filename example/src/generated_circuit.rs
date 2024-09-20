use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::eq::EqGadget;

/// Regex: ^[a-zA-Z]+[0-9]*@gmail\.com$
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

    // Initialize state variables (13 states)
    // 현재 상태 초기화
    let mut current_state = FpVar::constant(F::from(0u64));
    // 각 입력 인덱스에 대한 전이 로직
    for (index, current_input) in input_vars.iter().enumerate() {
      let is_padded = current_input.is_eq(&FpVar::constant(F::from(0u64)))?;
      let cond_padded = is_padded.not();
      let mut next_state = current_state.clone();
      let mut has_transitioned = Boolean::constant(false);

      let is_state_0 = current_state.is_eq(&FpVar::constant(F::from(0u64)))?;
      let cond_1 = is_state_0.and(&(current_input.is_eq(&FpVar::constant(F::from(65u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(66u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(67u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(68u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(69u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(70u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(71u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(72u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(73u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(74u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(75u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(76u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(77u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(78u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(79u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(80u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(81u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(82u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(83u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(84u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(85u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(86u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(87u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(88u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(89u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(90u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(97u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(98u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(99u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(100u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(101u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(102u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(103u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(104u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(105u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(106u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(107u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(108u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(109u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(110u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(111u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(112u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(113u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(114u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(115u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(116u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(117u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(118u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(119u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(120u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(121u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(122u64)))?)?))?;
      next_state = cond_1.select(&FpVar::constant(F::from(1u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_1)?;

      let is_state_1 = current_state.is_eq(&FpVar::constant(F::from(1u64)))?;
      let cond_2 = is_state_1.and(&(current_input.is_eq(&FpVar::constant(F::from(65u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(66u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(67u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(68u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(69u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(70u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(71u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(72u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(73u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(74u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(75u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(76u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(77u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(78u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(79u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(80u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(81u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(82u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(83u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(84u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(85u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(86u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(87u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(88u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(89u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(90u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(97u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(98u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(99u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(100u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(101u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(102u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(103u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(104u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(105u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(106u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(107u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(108u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(109u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(110u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(111u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(112u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(113u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(114u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(115u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(116u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(117u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(118u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(119u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(120u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(121u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(122u64)))?)?))?;
      next_state = cond_2.select(&FpVar::constant(F::from(1u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_2)?;
      let cond_3 = is_state_1.and(&(current_input.is_eq(&FpVar::constant(F::from(48u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(49u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(50u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(51u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(52u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(53u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(54u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(55u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(56u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(57u64)))?)?))?;
      next_state = cond_3.select(&FpVar::constant(F::from(2u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_3)?;
      let cond_4 = is_state_1.and(&(current_input.is_eq(&FpVar::constant(F::from(64u64)))?))?;
      next_state = cond_4.select(&FpVar::constant(F::from(3u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_4)?;

      let is_state_2 = current_state.is_eq(&FpVar::constant(F::from(2u64)))?;
      let cond_5 = is_state_2.and(&(current_input.is_eq(&FpVar::constant(F::from(48u64)))?.or(&current_input.is_eq(&FpVar::constant(F::from(49u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(50u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(51u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(52u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(53u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(54u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(55u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(56u64)))?)?.or(&current_input.is_eq(&FpVar::constant(F::from(57u64)))?)?))?;
      next_state = cond_5.select(&FpVar::constant(F::from(2u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_5)?;
      let cond_6 = is_state_2.and(&(current_input.is_eq(&FpVar::constant(F::from(64u64)))?))?;
      next_state = cond_6.select(&FpVar::constant(F::from(3u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_6)?;

      let is_state_3 = current_state.is_eq(&FpVar::constant(F::from(3u64)))?;
      let cond_7 = is_state_3.and(&(current_input.is_eq(&FpVar::constant(F::from(103u64)))?))?;
      next_state = cond_7.select(&FpVar::constant(F::from(4u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_7)?;

      let is_state_4 = current_state.is_eq(&FpVar::constant(F::from(4u64)))?;
      let cond_8 = is_state_4.and(&(current_input.is_eq(&FpVar::constant(F::from(109u64)))?))?;
      next_state = cond_8.select(&FpVar::constant(F::from(5u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_8)?;

      let is_state_5 = current_state.is_eq(&FpVar::constant(F::from(5u64)))?;
      let cond_9 = is_state_5.and(&(current_input.is_eq(&FpVar::constant(F::from(97u64)))?))?;
      next_state = cond_9.select(&FpVar::constant(F::from(6u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_9)?;

      let is_state_6 = current_state.is_eq(&FpVar::constant(F::from(6u64)))?;
      let cond_10 = is_state_6.and(&(current_input.is_eq(&FpVar::constant(F::from(105u64)))?))?;
      next_state = cond_10.select(&FpVar::constant(F::from(7u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_10)?;

      let is_state_7 = current_state.is_eq(&FpVar::constant(F::from(7u64)))?;
      let cond_11 = is_state_7.and(&(current_input.is_eq(&FpVar::constant(F::from(108u64)))?))?;
      next_state = cond_11.select(&FpVar::constant(F::from(8u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_11)?;

      let is_state_8 = current_state.is_eq(&FpVar::constant(F::from(8u64)))?;
      let cond_12 = is_state_8.and(&(current_input.is_eq(&FpVar::constant(F::from(46u64)))?))?;
      next_state = cond_12.select(&FpVar::constant(F::from(9u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_12)?;

      let is_state_9 = current_state.is_eq(&FpVar::constant(F::from(9u64)))?;
      let cond_13 = is_state_9.and(&(current_input.is_eq(&FpVar::constant(F::from(99u64)))?))?;
      next_state = cond_13.select(&FpVar::constant(F::from(10u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_13)?;

      let is_state_10 = current_state.is_eq(&FpVar::constant(F::from(10u64)))?;
      let cond_14 = is_state_10.and(&(current_input.is_eq(&FpVar::constant(F::from(111u64)))?))?;
      next_state = cond_14.select(&FpVar::constant(F::from(11u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_14)?;

      let is_state_11 = current_state.is_eq(&FpVar::constant(F::from(11u64)))?;
      let cond_15 = is_state_11.and(&(current_input.is_eq(&FpVar::constant(F::from(109u64)))?))?;
      next_state = cond_15.select(&FpVar::constant(F::from(12u64)), &next_state)?;
      has_transitioned = has_transitioned.or(&cond_15)?;

      let is_state_12 = current_state.is_eq(&FpVar::constant(F::from(12u64)))?;
      let invalid_transition = cond_padded.and(&has_transitioned.not())?;
      valid = valid.and(&invalid_transition.not())?;
      current_state = next_state;
    }
    // Acceptance logic
    let mut is_accepting = Boolean::constant(false);
    is_accepting = is_accepting.or(&current_state.is_eq(&FpVar::constant(F::from(12u64)))?)?;
    valid = valid.and(&is_accepting)?;
    valid.enforce_equal(&Boolean::constant(true))?;
    Ok(())
  }}