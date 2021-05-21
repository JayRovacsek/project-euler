mod test;

use rayon::prelude::*;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub fn solve() {
    let answer = find_divisible_by_all(20);
    println!("Answer is: {:?}", answer);
}

trait DivisibleByAll {
    fn divisible_by_all<'a>(&self, numbers: &'a Vec<usize>) -> bool;
}

impl DivisibleByAll for usize {
    fn divisible_by_all<'a>(&self, numbers: &'a Vec<usize>) -> bool {
        !numbers.iter().any(|x| self % x != 0)
    }
}

fn find_divisible_by_all(limit: usize) -> usize {
    let numbers: Vec<usize> = (1..=limit).collect();
    match (1_usize..=std::usize::MAX)
        .into_par_iter()
        .find_first(|x| x.divisible_by_all(&numbers))
    {
        Some(a) => a,
        _ => panic!("Could not find a suitable number that is divisible by all numbers"),
    }
}
