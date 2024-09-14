use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::eq::EqGadget;

/// Regex: a[bc]+d
#[derive(Clone)]
struct RegexCircuit<F: PrimeField> {
	input: Vec<F>,
	max_len: usize,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for RegexCircuit<F> {
	fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
		// Add state initialization logic here
		// Initialize input variables
		let input_vars = self.input
			.into_iter()
			.map(|v| FpVar::new_input(cs.clone(), || Ok(v)))
			.collect::<Result<Vec<_>, _>>()?;
		let mut valid = Boolean::constant(true);

		// Initialize state variables (4 states)
		if input_vars.len() < 3 || input_vars.len() > 4 {
			valid = Boolean::constant(false);
		} else {
			let mut current_state = FpVar::constant(F::from(0u64));

			// Transition logic for input index 0
			if input_vars.len() > 0 {
				let current_input = &input_vars[0];
				let mut next_state = None;
				if current_state.is_eq(&FpVar::constant(F::from(0u64)))? && current_input.is_eq(&FpVar::constant(F::from(97u64)))? {
					next_state = Some(FpVar::constant(F::from(1u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(100u64)))? {
					next_state = Some(FpVar::constant(F::from(3u64)));
				}
				if let Some(next) = next_state {
					current_state = next;
				} else {
					valid = Boolean::constant(false);
				}
			}

			// Transition logic for input index 1
			if input_vars.len() > 1 {
				let current_input = &input_vars[1];
				let mut next_state = None;
				if current_state.is_eq(&FpVar::constant(F::from(0u64)))? && current_input.is_eq(&FpVar::constant(F::from(97u64)))? {
					next_state = Some(FpVar::constant(F::from(1u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(100u64)))? {
					next_state = Some(FpVar::constant(F::from(3u64)));
				}
				if let Some(next) = next_state {
					current_state = next;
				} else {
					valid = Boolean::constant(false);
				}
			}

			// Transition logic for input index 2
			if input_vars.len() > 2 {
				let current_input = &input_vars[2];
				let mut next_state = None;
				if current_state.is_eq(&FpVar::constant(F::from(0u64)))? && current_input.is_eq(&FpVar::constant(F::from(97u64)))? {
					next_state = Some(FpVar::constant(F::from(1u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(100u64)))? {
					next_state = Some(FpVar::constant(F::from(3u64)));
				}
				if let Some(next) = next_state {
					current_state = next;
				} else {
					valid = Boolean::constant(false);
				}
			}

			// Transition logic for input index 3
			if input_vars.len() > 3 {
				let current_input = &input_vars[3];
				let mut next_state = None;
				if current_state.is_eq(&FpVar::constant(F::from(0u64)))? && current_input.is_eq(&FpVar::constant(F::from(97u64)))? {
					next_state = Some(FpVar::constant(F::from(1u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(1u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(98u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(99u64)))? {
					next_state = Some(FpVar::constant(F::from(2u64)));
				}
				if current_state.is_eq(&FpVar::constant(F::from(2u64)))? && current_input.is_eq(&FpVar::constant(F::from(100u64)))? {
					next_state = Some(FpVar::constant(F::from(3u64)));
				}
				if let Some(next) = next_state {
					current_state = next;
				} else {
					valid = Boolean::constant(false);
				}
			}

			// No end anchor, check any valid accepting state
			let mut is_accepting = Boolean::constant(false);
			is_accepting = is_accepting.or(&current_state.is_eq(&FpVar::constant(F::from(3u64)))?)?;
			valid = valid.and(&is_accepting)?;
		}
		// Acceptance logic
		// Ensure the final state is an accepting state
		valid.enforce_equal(&Boolean::constant(true))?;
		Ok(())
	}
}
