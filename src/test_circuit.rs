// test regex: "a[bc]+d"


// use ark_ff::PrimeField;
// use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
// use ark_r1cs_std::alloc::AllocVar;
// use ark_r1cs_std::boolean::Boolean;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::eq::EqGadget;
// use ark_snark::SNARK;
// use ark_groth16::{Groth16, ProvingKey, VerifyingKey, Proof};
// use ark_groth16::r1cs_to_qap::LibsnarkReduction;
// use ark_bn254::{Bn254, Fr};
// use rand::rngs::OsRng;
// use ark_r1cs_std::fields::FieldVar;

// #[derive(Clone)]
// struct RegexCircuit<F: PrimeField> {
//     input: Vec<F>, // input string as a vector of field elements
//     max_len: usize, // 최대 길이 추가
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for RegexCircuit<F> {
//     fn generate_constraints(
//         self,
//         cs: ConstraintSystemRef<F>,
//     ) -> Result<(), SynthesisError> {
//         // Step 1: Allocate input variables with padding if necessary
//         let padded_input = if self.input.len() < self.max_len {
//             let mut padded = self.input.clone();
//             padded.resize(self.max_len, F::from(0u64)); // 0으로 패딩
//             padded
//         } else {
//             self.input.clone()
//         };

//         let input_vars = padded_input
//             .into_iter()
//             .map(|v| FpVar::new_input(cs.clone(), || Ok(v)))
//             .collect::<Result<Vec<_>, _>>()?;
//         
//         // Step 2: Implement the regex check for "a[bc]+d"
//         let mut valid = Boolean::constant(true);

//         // Check that input has at least three elements for "a[bc]+d" pattern
//         if self.input.len() < 3 {
//             valid = Boolean::constant(false);
//         } else {
//             // Check first character is 'a'
//             let is_a = input_vars[0].is_eq(&FpVar::constant(F::from(1u64)))?;
//             valid = valid.and(&is_a)?;

//             // Check last character is 'd'
//             let is_d = input_vars[self.input.len() - 1].is_eq(&FpVar::constant(F::from(4u64)))?;
//             valid = valid.and(&is_d)?;

//             // Check middle characters are all 'b' or 'c'
//             for var in &input_vars[1..self.input.len() - 1] {
//                 let is_b = var.is_eq(&FpVar::constant(F::from(2u64)))?;
//                 let is_c = var.is_eq(&FpVar::constant(F::from(3u64)))?;
//                 let is_b_or_c = is_b.or(&is_c)?;
//                 valid = valid.and(&is_b_or_c)?;
//             }
//         }

//         // Ensure valid is true
//         valid.enforce_equal(&Boolean::constant(true))?;

//         Ok(())
//     }
// }

// fn main() {
//     // Example input for "abcbcd" -> [1, 2, 3, 2, 3, 4]
//     // let input: Vec<Fr> = vec![1u64, 2, 3, 2, 3, 4].into_iter().map(|x| {Fr::from(x)}).collect();
//     let input: Vec<Fr> = vec![1u64, 3, 2, 4].into_iter().map(|x| {Fr::from(x)}).collect();

//     // Define the circuit with the correct maximum length
//     let circuit = RegexCircuit { input: input.clone(), max_len: 7 };

//     // Prove and verify the circuit
//     let mut rng = OsRng;

//     // Setup proving and verifying keys
//     let (pk, vk): (ProvingKey<Bn254>, VerifyingKey<Bn254>) = Groth16::<Bn254, LibsnarkReduction>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

//     // Prove for the input circuit
//     let proof: Proof<Bn254> = Groth16::<Bn254, LibsnarkReduction>::prove(&pk, circuit, &mut rng).unwrap();

//     // Inputs must match the padded length
//     let mut padded_inputs = vec![Fr::from(1u64), Fr::from(3u64), Fr::from(2u64), Fr::from(4u64)];
//     padded_inputs.resize(7, Fr::from(0u64)); // Ensure padded length matches

//     // Verify the proof with correct inputs
//     let is_valid = Groth16::<Bn254, LibsnarkReduction>::verify(&vk, &padded_inputs, &proof).unwrap();

//     println!("Verification result: {}", is_valid);
// }

