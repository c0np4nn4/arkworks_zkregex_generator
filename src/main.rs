use std::{fs::File, io::Write};

use regex::create_regex_and_dfa_from_str_and_defs;
use structs::SubstringDefinitionsJson;
use arkworks::gen_arkworks_allstr;

mod structs;
mod regex;
mod errors;
mod arkworks;
mod handlers;


fn main() {
    // Example regex input
    // let regex_str = r"^[a-zA-Z]{2,}\s[a-zA-Z]{1,}'?-?[a-zA-Z]{2,}\s?([a-zA-Z]{1,})?$";
    let regex_str = r"a[bc]+d";

    // Placeholder SubstringDefinitionsJson; replace with actual data
    let substr_defs_json = SubstringDefinitionsJson {
        transitions: vec![vec![(1, 2)]],
    };

    // Create RegexAndDFA from regex string
    match create_regex_and_dfa_from_str_and_defs(//
        regex_str, 
        substr_defs_json
    ) {
        Ok(regex_and_dfa) => {
            println!("regex and dfa: {:#?}", regex_and_dfa);

            // Generate Rust code for Arkworks circuit
            let rust_code = gen_arkworks_allstr(//
                &regex_and_dfa.dfa, 
                "RegexCircuit", 
                regex_str, 
                regex_and_dfa.has_end_anchor
            );

            // Save the generated code to a Rust file
            let mut file = File::create("generated_circuit.rs").expect("Unable to create file");
            file.write_all(rust_code.as_bytes()).expect("Unable to write data");

            println!("Generated Rust circuit code saved to 'generated_circuit.rs'");
        },
        Err(e) => {
            println!("Failed to create RegexAndDFA: {}", e);
        }
    }
}
