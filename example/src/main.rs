use ark_snark::SNARK;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey, Proof};
use ark_groth16::r1cs_to_qap::LibsnarkReduction;
use ark_bn254::{Bn254, Fr};
use rand::rngs::OsRng;
use std::io::{self, Write};

mod generated_circuit;
use crate::generated_circuit::RegexCircuit;

const MAX_LEN: usize = 256;

fn main() {
    // 입력을 stdin으로 받아 처리
    let input_string = read_input();

    // 입력된 문자열을 u64로 변환해 벡터에 넣음 (각 문자를 u64로 변환)
    let input: Vec<Fr> = input_string
        .chars()
        .map(|c| Fr::from(c as u64))  // 각 문자를 u64로 변환
        .collect();

    println!("[example/src/main.rs] input: {:#?}", input);

    // Define the circuit with the correct maximum length
    let circuit = RegexCircuit { input: input.clone(), max_len: MAX_LEN };
    println!("[+] Circuit done");

    // Prove and verify the circuit
    let mut rng = OsRng;

    // Setup proving and verifying keys
    let (pk, vk): (ProvingKey<Bn254>, VerifyingKey<Bn254>) = Groth16::<Bn254, LibsnarkReduction>::circuit_specific_setup(
        circuit.clone(), &mut rng
    ).unwrap();
    println!("[+] Proving and Verifying keys have been generated");

    // Prove for the input circuit
    let proof: Proof<Bn254> = Groth16::<Bn254, LibsnarkReduction>::prove(&pk, circuit, &mut rng).unwrap();
    println!("[+] Proof has been generated");

    // 패딩된 입력 생성
    let mut padded_inputs: Vec<Fr> = input;
    padded_inputs.resize(MAX_LEN, Fr::from(0u64)); // Ensure padded length matches

    // Verify the proof with correct inputs
    let is_valid = Groth16::<Bn254, LibsnarkReduction>::verify(&vk, &padded_inputs, &proof).unwrap();

    println!("Verification result: {}", is_valid);
}

/// Reads a line of input from stdin and returns it as a String.
fn read_input() -> String {
    print!("Enter string to check: ");
    io::stdout().flush().unwrap();  // Ensure the prompt is printed before waiting for input

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read input");

    input_string.trim().to_string()  // Remove leading/trailing whitespace
}

