/*
Finding a Motif in DNA
Given: Two DNA strings s and t (each of length at most 1 kbp).
Return: All locations of t as a substring of s.
cargo run --bin translating_rna_into_protein rosalind_prot.txt
*/
use std::{env, fs::{File}, io::{self, BufRead}, path::Path};
use rosalind_rust::dealing_with_nucleotides::{self};
use itertools::Itertools;
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Find a motif in a DNA sequence 🧬");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if let Ok(mut lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let sequence = lines
            .next().unwrap().unwrap();
        let motif = lines
            .next().unwrap().unwrap();
        let positions:Vec<usize> = dealing_with_nucleotides::find_a_motif_in_dna(&sequence, &motif)
            .into_iter().map(|x| x + 1).collect();
        println!("{}", positions.iter().format(" "));
    }
}