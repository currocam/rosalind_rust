pub mod miscellaneous;
#[cfg(test)]
mod tests;
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