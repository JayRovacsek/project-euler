mod test;

use std::collections::HashSet;

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use rayon::prelude::*;

pub fn solve() {
    let products = generate_products(100, 999);
    let answer = products
        .par_iter()
        .filter(|x| is_palindrome(**x))
        .max()
        .unwrap();
    println!("Answer is: {:?}", answer);
}

fn generate_products(min: usize, max: usize) -> HashSet<usize> {
    (min..=max)
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|x| {
            (min..=max)
                .collect::<Vec<usize>>()
                .iter()
                .map(|y| x * y)
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect()
}

fn is_palindrome(input: usize) -> bool {
    let chars = input.to_string().chars().collect::<Vec<char>>();
    let (front, back): (Vec<(usize, &char)>, Vec<(usize, &char)>) = chars
        .iter()
        .enumerate()
        .partition(|c| c.0 < chars.len() / 2);
    !front
        .iter()
        .zip(back.iter().rev())
        .map(|x| {
            let a = x.0;
            let b = x.1;
            a.1 == b.1
        })
        .any(|x| !x)
}
