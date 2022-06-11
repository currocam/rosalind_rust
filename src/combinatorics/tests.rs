use crate::combinatorics::rabbits;
#[test]
fn it_works_as_fibonacci() {
    assert_eq!(1, rabbits::calculate_number_of_rabbit_pairs(1, 1));
    assert_eq!(1, rabbits::calculate_number_of_rabbit_pairs(1, 1));
    assert_eq!(2, rabbits::calculate_number_of_rabbit_pairs(3, 1));
    assert_eq!(3, rabbits::calculate_number_of_rabbit_pairs(4, 1));
    assert_eq!(6765, rabbits::calculate_number_of_rabbit_pairs(20, 1));
}
#[test]
fn it_works_for_k_new_pair_of_rabbits() {
    assert_eq!(19, rabbits::calculate_number_of_rabbit_pairs(5, 3));
}