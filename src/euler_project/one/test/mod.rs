use super::*;

#[test]
fn test() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_sum_of_3_and_5_multiples() {
    assert_eq!(233168, sum_of_3_and_5_multiples(999))
}