use super::*;

#[test]
fn test() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_generate_fib_numbers() {
    assert_eq!(vec!(1, 2, 3, 5, 8, 13, 21, 34, 55, 89), generate_fib_numbers(90))
}