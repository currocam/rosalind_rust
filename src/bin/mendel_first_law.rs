/*
Mendel's First Law
Given: Three positive integers k, m, and n, representing a population containing k+m+n
organisms: k individuals are homozygous dominant for a factor, m are heterozygous, and
n are homozygous recessive.
Return: The probability that two randomly selected mating organisms will produce an individual
possessing a dominant allele (and thus displaying the dominant phenotype). Assume that
any two organisms can mate.
*/

use rosalind_rust::probability;
fn main() {
    println!("Calculate the probability that dominant individual.");
    println!("Enter k: ");
    let k =  rosalind_rust::read_number();
    println!("Enter m: ");
    let m =  rosalind_rust::read_number();
    println!("Enter n: ");
    let n =  rosalind_rust::read_number();
    let result = probability::calculate_dominant_phenotype(k, m, n);
    println!("{:.5}",result)
}