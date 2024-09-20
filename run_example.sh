# generated `generated_circuit.rs`
cargo run

# copy generated circuit to example folder
rm ./example/src/generated_circuit.rs
cp ./generated_circuit.rs ./example/src/

# run test code
cd example
cargo run

# return
cd ..

