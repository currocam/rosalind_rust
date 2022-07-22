pub fn calculate_hamming_distance(s: &str, t: &str) -> u64{
    let mut hamming_distance: u64 = 0;
    let s = s.chars().into_iter();
    let mut t = t.chars().into_iter();
    for nucleotide1 in s{
        let nucletide2 = t.next().expect("To Do");
        if nucleotide1 != nucletide2 {
            hamming_distance += 1;
        }
    }
    return hamming_distance;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_2_equal_length_sequences_it_returns_hamming_distance() {
        let dna_string1 = String::from("GAGCCTACTAACGGGAT");
        let dna_string2 = String::from("CATCGTAATGACGGCCT");
        let result = calculate_hamming_distance(&dna_string1, &dna_string2);
        assert_eq!(result, 7);
    }
}