use std::io;

pub mod dealing_with_nucleotides;
pub mod combinatorics;
pub mod probability;

pub fn read_number() -> u64 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: u64 = number.trim().parse().expect("Please type a number!");
    return number;
}