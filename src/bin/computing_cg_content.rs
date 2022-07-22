/*
A solution to Computing GC Content problem from Rosalind
Given: At most 10 DNA strings in FASTA format (of length at most 1 kbp each).
Return: The ID of the string having the highest GC-content, followed by the GC-content of that string.
Rosalind allows for a default error of 0.001 in all decimal answers unless otherwise stated;
please see the note on absolute error below.

cat rosalind_gc.txt| computing_cg_content
*/
use rosalind_rust::dealing_with_nucleotides;
use bio::io::fasta;
use   std::io;
fn main() {
    println!("Computing CG Content % 🧬");
    let reader = fasta::Reader::new(io::stdin());
    let mut fasta_header= "".to_owned();
    let mut cg_percentage:f32= -1.0;
    for record in reader.records() {
        let record = record.unwrap();
        let temp_cg_percentage = dealing_with_nucleotides::compute_cg_content(
            std::str::from_utf8(record.seq()).unwrap()
        );
        if temp_cg_percentage> cg_percentage{
            fasta_header = record.id().to_owned();
            cg_percentage= temp_cg_percentage;        
        }
    }
    println!("{}", fasta_header);
    println!("{}", cg_percentage*100.0);

}
