/*
A solution to Counting DNA Nucleotides problem from Rosalind
Given: A DNA string s of length at most 1000 nt.
Return: Four integers (separated by spaces) counting the respective number of times that the
symbols 'A', 'C', 'G', and 'T' occur in s.
*/
use std::io;
use rosalind_rust::dealing_with_nucleotides::miscellaneous;
fn main() {
    println!("Counting DNA Nucleotides 🧬");
    let mut sequence = String::new();
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    let result = miscellaneous::count_nucleotides_dna(&sequence
        .trim()
        .to_uppercase());
    println!(
        "{} {} {} {}",
        result.adenine, result.cytosine, result.guanine, result.thymine
    )
}
