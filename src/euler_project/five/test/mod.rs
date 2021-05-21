use super::*;

#[test]
fn test() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_find_divisible_by_all() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let numbers: Vec<usize> = (1..=10).collect();
    let n = match (1..std::usize::MAX)
        .into_par_iter()
        .find_first(|x| x.divisible_by_all(&numbers))
    {
        Some(a) => a,
        _ => panic!("Could not find a suitable number that is divisible by all numbers"),
    };
    assert_eq!(n, 2520);
}
