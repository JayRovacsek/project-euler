mod test;

use rayon::prelude::*;

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn solve() {
    let answer = sum_of_3_and_5_multiples(999);
    println!("Answer is: {}", answer);
}

fn sum_of_3_and_5_multiples(limit: usize) -> usize {
    (1..=limit)
        .collect::<Vec<usize>>()
        .par_iter()
        .filter(|x| *x % 3 == 0 || *x % 5 == 0)
        .sum()
}
