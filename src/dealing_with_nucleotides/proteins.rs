use phf::phf_map;
static RNA_CODON_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    "UUU" => "F",      "CUU" => "L",      "AUU" => "I",      "GUU" => "V",
    "UUC" => "F",      "CUC" => "L",      "AUC"=> "I",      "GUC" => "V",
    "UUA" => "L",      "CUA" => "L",      "AUA"=> "I",      "GUA" => "V",
    "UUG" => "L",      "CUG" => "L",      "AUG"=> "M",      "GUG" => "V",
    "UCU" => "S",      "CCU" => "P",      "ACU"=> "T",      "GCU" => "A",
    "UCC" => "S",      "CCC" => "P",      "ACC"=> "T",      "GCC" => "A",
    "UCA" => "S",      "CCA" => "P",      "ACA"=> "T",      "GCA" => "A",
    "UCG" => "S",      "CCG" => "P",      "ACG"=> "T",      "GCG" => "A",
    "UAU" => "Y",      "CAU" => "H",      "AAU"=> "N",      "GAU" => "D",
    "UAC" => "Y",      "CAC" => "H",      "AAC"=> "N",      "GAC" => "D",
    "UAA" => "Stop",   "CAA" => "Q",      "AAA"=> "K",      "GAA" => "E",
    "UAG" => "Stop",   "CAG" => "Q",      "AAG"=> "K",      "GAG" => "E",
    "UGU" => "C",      "CGU" => "R",      "AGU"=> "S",      "GGU" => "G",
    "UGC" => "C",      "CGC" => "R",      "AGC"=> "S",      "GGC" => "G",
    "UGA" => "Stop",   "CGA" => "R",      "AGA"=> "R",      "GGA" => "G",
    "UGG" => "W",      "CGG" => "R",      "AGG"=> "R",      "GGG" => "G", 
};
pub fn transcribing_tna_into_protein(rna_seq: &str)-> String{
    let codons = rna_seq.as_bytes();
    let mut protein_seq = Vec::new();
    
    for index in (0..codons.len()-2).step_by(3){
        let codon = std::str::from_utf8(&codons[index..index+3])
            .unwrap();
        let aminoacid = RNA_CODON_TABLE.get(codon).cloned();
        match aminoacid {
            Some("Stop") => break,
            Some(aa) => protein_seq.push(aa),
            None => panic!("Non standar codon was found"),
        }
    }
    return protein_seq.join("");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_rna_seq_it_returns_protein_seq() {
        let rna_string = String::from("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA");
        let result = transcribing_tna_into_protein(&rna_string);
        assert_eq!(result, "MAMAPRTEINSTRING");
    }
}

