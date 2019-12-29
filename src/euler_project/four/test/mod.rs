use super::*;

#[test]
fn test() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_generate_products() {
    let mut products = generate_products(1, 3)
        .iter()
        .cloned()
        .collect::<Vec<usize>>();
    products.sort();
    assert_eq!(vec!(1, 2, 3, 4, 6, 9), products);
    assert_ne!(vec!(1, 2, 3, 4, 6, 12), products)
}

#[test]
fn test_is_palindrome() {
    assert_eq!(false, is_palindrome(123123));
    assert_eq!(true, is_palindrome(123321));
}
