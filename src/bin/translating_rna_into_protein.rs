/*
Translating RNA into Protein
Given: An RNA string s corresponding to a strand of mRNA (of length at most 10 kbp).
Return: The protein string encoded by s.
cargo run --bin translating_rna_into_protein rosalind_prot.txt
*/
use std::{env, fs};
use rosalind_rust::dealing_with_nucleotides::proteins;
fn main() {
    println!("Translating a RNA strand 🧬");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let rna_seq = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let result = proteins::transcribing_tna_into_protein(&rna_seq);
    println!("{}",result);
}