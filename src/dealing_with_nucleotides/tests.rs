use crate::dealing_with_nucleotides::transcribing_dna_into_rna;
#[test]
fn it_works_when_transcribing_dna_into_rna() {
    let dna_string = String::from("GATGGAACTTGACTACGTAAATT");
    let rna_string = String::from("GAUGGAACUUGACUACGUAAAUU");
    let result = transcribing_dna_into_rna(&dna_string);
    assert_eq!(result, rna_string);
}
