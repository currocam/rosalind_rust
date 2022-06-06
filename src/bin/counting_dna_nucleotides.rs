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
    println!("{} {} {} {}", result.0, result.1, result.2, result.3)
}
const ADENINE: u8 = 65;
const CYTOSINE: u8 = 67;
const GUANINE: u8 = 71;
const THYMINE: u8 = 84;
pub fn count_nucleotides_dna(dna_string:&str)-> (u32, u32, u32, u32){
    // Counting nucleotides
    let mut counting_tuple:(u32, u32, u32, u32) = (0, 0, 0, 0);
    let nucleotides =  dna_string.to_uppercase();
    for nucleotide in nucleotides.as_bytes().into_iter(){
        match *nucleotide {
            ADENINE => counting_tuple.0 +=1,
            CYTOSINE => counting_tuple.1 +=1,
            GUANINE => counting_tuple.2 +=1,
            THYMINE => counting_tuple.3 +=1,
            _ => panic!("An invalid value was found.")}

    }
    counting_tuple
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_with_sample_dataset() {
        let dna_string = String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
        let result:(u32, u32, u32, u32) = (20, 12, 17, 21);
        assert_eq!(count_nucleotides_dna(&dna_string), result);
    }
}