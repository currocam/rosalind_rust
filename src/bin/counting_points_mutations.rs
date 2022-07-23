/*
Counting Point Mutations
Given: Two DNA strings s and t f equal length (not exceeding 1 kbp).
Return: The Hamming distance dH(s,t)
*/
use rosalind_rust::dealing_with_nucleotides::alignment;
use rosalind_rust::read_string;
fn main() {
    println!("Counting Point Mutations 🚬.");
    println!("Enter first sequence: ");
    let sequence1 = read_string();
    println!("Enter second sequence: ");
    let sequence2 = read_string();
    let result = alignment::calculate_hamming_distance(&sequence1, &sequence2);
    println!("{}",result)
} 