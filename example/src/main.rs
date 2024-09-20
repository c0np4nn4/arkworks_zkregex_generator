use ark_snark::SNARK;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey, Proof};
use ark_groth16::r1cs_to_qap::LibsnarkReduction;
use ark_bn254::{Bn254, Fr};
use rand::rngs::OsRng;

mod generated_circuit;
use crate::generated_circuit::RegexCircuit;


fn main() {
    // "abcbcd"
    let input: Vec<Fr> = vec![97, 98, 99, 100].into_iter().map(|x| {Fr::from(x)}).collect();

    // Define the circuit with the correct maximum length
    let circuit = RegexCircuit { input, max_len: 8 };

    // Prove and verify the circuit
    let mut rng = OsRng;

    // Setup proving and verifying keys
    let (pk, vk): (ProvingKey<Bn254>, VerifyingKey<Bn254>) = Groth16::<Bn254, LibsnarkReduction>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

    // Prove for the input circuit
    let proof: Proof<Bn254> = Groth16::<Bn254, LibsnarkReduction>::prove(&pk, circuit, &mut rng).unwrap();

    // Inputs must match the padded length
    let mut padded_inputs = vec![ //
        Fr::from(97u64), 
        Fr::from(98u64), 
        Fr::from(99u64), 
        Fr::from(100u64), 
    ];
    padded_inputs.resize(8, Fr::from(0u64)); // Ensure padded length matches

    // Verify the proof with correct inputs
    let is_valid = Groth16::<Bn254, LibsnarkReduction>::verify(&vk, &padded_inputs, &proof).unwrap();

    println!("Verification result: {}", is_valid);
}

