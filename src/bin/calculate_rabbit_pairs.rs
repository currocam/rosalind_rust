// Rabbits and Recurrence Relations
// Given: Positive integers n≤40 and k≤5
//Return: The total number of rabbit pairs that will be present after n months, if we begin with
// 1 pair and in each generation, every pair of reproduction-age rabbits produces a litter of k rabbit pairs (instead of only 1 pair).

use std::io;
use rosalind_rust::combinatorics::rabbits;
fn main() {
    println!("Calculate number of rabbit pairs 🐰.");
    println!("Enter n: ");
    let n = rosalind_rust::read_number();
    println!("Enter k: ");
    let k = rosalind_rust::read_number();
    let result = rabbits::calculate_number_of_rabbit_pairs(n, k);
    println!("{}",result)
}