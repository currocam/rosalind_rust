// A solution to Transcribing DNA into RNA problem from Rosalind
// Given: A DNA string t having length at most 1000 nt.
// Return: The transcribed RNA string of t
use std::io;
//use rosalind_rust::nucleotides;
fn main() {
    println!("Transcribing DNA into RNA 🧬");
    let mut sequence = String::new();
    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");
    let result = transcribing_dna_into_rna(&sequence.
        trim()
        .to_uppercase());
    println!("{}",result)
}
pub fn get_complementing_dna_strand(dna_string:&str) -> String {
    let mut complementing_strand = String::new();
    for nucleotide in dna_string.chars().rev().into_iter(){
        match nucleotide {
            'A' => complementing_strand.push('T'),
            'T' => complementing_strand.push('A'),
            'C' => complementing_strand.push('G'),
            'G' => complementing_strand.push('C'),
            _ => panic!("An invalid value was found.")}
    }
    return complementing_strand;
}
pub fn transcribing_dna_into_rna(dna_string:&str) -> String {
    let mut rna_seq = String::new();
    let complementing_strand = get_complementing_dna_strand(dna_string);
    for nucleotide in complementing_strand.chars().rev().into_iter(){
        match nucleotide {
            'A' => rna_seq.push('U'),
            'T' => rna_seq.push('A'),
            'C' => rna_seq.push('G'),
            'G' => rna_seq.push('C'),
            _ => panic!("An invalid value was found.")}
    }
    return rna_seq;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_with_sample_dataset() {
        let dna_string = String::from("GATGGAACTTGACTACGTAAATT");
        let rna_string = String::from("GAUGGAACUUGACUACGUAAAUU");
        let result = transcribing_dna_into_rna(&dna_string);
        assert_eq!(result, rna_string);
    }
}