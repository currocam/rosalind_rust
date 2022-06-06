// A solution to Counting DNA Nucleotides problem from Rosalind
// Given: A DNA string s of length at most 1000 nt.
// Return: Four integers (separated by spaces) counting the respective number of times that the
// symbols 'A', 'C', 'G', and 'T' occur in s.
use std::io;
//use rosalind_rust::nucleotides;
fn main() {
    println!("Counting DNA Nucleotides 🧬");
    let mut sequence = String::new();
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    let result = count_nucleotides_dna(&sequence.trim());
    println!(
        "{} {} {} {}",
        result.adenine, result.cytosine, result.guanine, result.thymine
    )
}
const ADENINE: u8 = 65;
const CYTOSINE: u8 = 67;
const GUANINE: u8 = 71;
const THYMINE: u8 = 84;

pub struct CountingNucleotides {
    pub adenine: u32,
    pub cytosine: u32,
    pub guanine: u32,
    pub thymine: u32
}
pub fn count_nucleotides_dna(dna_string:&str)-> CountingNucleotides {
    let mut counting_nucleotides = CountingNucleotides {
        adenine: 0, cytosine: 0, guanine: 0, thymine: 0
    };
    let nucleotides =  dna_string.to_uppercase();
    for nucleotide in nucleotides.as_bytes().into_iter(){
        match *nucleotide {
            ADENINE => counting_nucleotides.adenine +=1,
            CYTOSINE => counting_nucleotides.cytosine+=1,
            GUANINE => counting_nucleotides.guanine +=1,
            THYMINE => counting_nucleotides.thymine +=1,
            _ => panic!("An invalid value was found.")}

    }
    return counting_nucleotides;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_with_sample_dataset() {
        let dna_string = String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
        let result = count_nucleotides_dna(&dna_string);
        assert_eq!(20, result.adenine);
        assert_eq!(12, result.cytosine);
        assert_eq!(17, result.guanine);
        assert_eq!(21, result.thymine);
    }
}