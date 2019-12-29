use super::*;

#[test]
fn test() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_generate_possible_prime_factors() {
    assert_eq!(
        vec!(
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113
        ),
        generate_possible_prime_factors(&13195)
    )
}
