pub fn calculate_number_of_rabbit_pairs (n : u64, k : u64) -> u64{
    const  INITIAL_PAR_OF_RABBITS: u64 = 1;
    const  INITIAL_PAR_OF_AGE_REPRODUCTION_RABBITS: u64 = 0;
    fn recursive_rabbits(n: u64,k: u64, penultimate: u64, last: u64) -> u64 {
        let penultimate= penultimate*k;
        match n {
            0 => penultimate,
            1 => last,
            _ => recursive_rabbits(n - 1,k, last, penultimate + last),
        }
    }
    recursive_rabbits(
        n,k,
        INITIAL_PAR_OF_AGE_REPRODUCTION_RABBITS, INITIAL_PAR_OF_RABBITS
    )
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works_as_fibonacci() {
        assert_eq!(1, calculate_number_of_rabbit_pairs(1, 1));
        assert_eq!(1, calculate_number_of_rabbit_pairs(1, 1));
        assert_eq!(2, calculate_number_of_rabbit_pairs(3, 1));
        assert_eq!(3, calculate_number_of_rabbit_pairs(4, 1));
        assert_eq!(6765, calculate_number_of_rabbit_pairs(20, 1));
    }
    #[test]
    fn it_works_for_k_new_pair_of_rabbits() {
        assert_eq!(19, calculate_number_of_rabbit_pairs(5, 3));
    }
}
