use crate::combinatorics::rabbits;
#[test]
fn it_works_calculate_number_of_rabbit_pairs_with_sample_data() {
    assert_eq!(19, rabbits::calculate_number_of_rabbit_pairs(5, 3));
}