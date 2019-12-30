mod test;

use rayon::prelude::*;

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub fn solve() {
    // Originally the upper bound of this search wasn't apparent to me,
    // however it became extremely obvious that a bounds of n where sqrt n
    // was less than or equal to our goal sum of a + b + c was a reasonable
    // approach to this. I'm sure it could be lowered even further
    let triplets = find_triplets(1000000);
    let triplet_answer = triplets
        .iter()
        .find(|x| x.0 + x.1 + x.2 == 1000_f64)
        .unwrap();
    let answer = triplet_answer.0 * triplet_answer.1 * triplet_answer.2;
    println!("Answer is: {:?}", answer);
}

fn find_triplets(limit: usize) -> Vec<(f64, f64, f64)> {
    let range_limit = (limit as f64).sqrt().floor();
    let suitable_numbers = (2..=range_limit as i64)
        .collect::<Vec<i64>>()
        .iter()
        .map(|x| *x as f64)
        .collect::<Vec<f64>>();

    suitable_numbers
        .par_iter()
        .map(|x| {
            suitable_numbers
                .iter()
                .filter(|y| y < &x)
                .map(|y| {
                    let ac = (x * x) - (*y * *y);
                    suitable_numbers
                        .iter()
                        .filter(|z| z < &y && **z == ac.sqrt())
                        .map(|z| (*z, *y, *x))
                        .collect::<Vec<(f64, f64, f64)>>()
                })
                .collect::<Vec<Vec<(f64, f64, f64)>>>()
        })
        .flatten()
        .flatten()
        .collect::<Vec<(f64, f64, f64)>>()
}
