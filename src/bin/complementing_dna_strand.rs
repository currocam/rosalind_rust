/*
Complementing a Strand of DNA
Given: A DNA string s of length at most 1000 bp.
Return: The reverse complement sc of s.
*/
use std::io;
use rosalind_rust::dealing_with_nucleotides;
fn main() {
    println!("Complementing a DNA strand 🧬");
    let mut sequence = String::new();
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    let result = dealing_with_nucleotides::get_complementing_dna_strand(&sequence.
        trim()
        .to_uppercase());
    println!("{}",result)
}