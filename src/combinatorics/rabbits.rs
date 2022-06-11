// Rabbits and Recurrence Relations
// Given: Positive integers n≤40 and k≤5
//Return: The total number of rabbit pairs that will be present after n months, if we begin with
// 1 pair and in each generation, every pair of reproduction-age rabbits produces a litter of k rabbit pairs (instead of only 1 pair).


pub fn calculate_number_of_rabbit_pairs (n : u32, k : u32) -> u32{
    const  INITIAL_PAR_OF_RABBITS: u32 = 1;
    const  INITIAL_PAR_OF_AGE_REPRODUCTION_RABBITS: u32 = 0;
    fn recursive_rabbits(n: u32,k: u32, penultimate: u32, last: u32) -> u32 {
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
