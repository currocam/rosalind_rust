/*
A solution to Transcribing DNA into RNA problem from Rosalind
Given: A DNA string t having length at most 1000 nt.
Return: The transcribed RNA string of t
*/
use std::io;
use rosalind_rust::dealing_with_nucleotides;
fn main() {
    println!("Transcribing DNA into RNA 🧬");
    let mut sequence = String::new();
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    let result = dealing_with_nucleotides::transcribing_dna_into_rna(&sequence.
        trim()
        .to_uppercase());
    println!("{}",result)
}
