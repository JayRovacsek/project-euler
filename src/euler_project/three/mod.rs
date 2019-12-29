mod test;

use primes::PrimeSet;
use rayon::prelude::*;

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub fn solve() {
    let number = 600851475143;
    let primes = generate_possible_prime_factors(&number);
    let answer = get_max_prime_factor(primes, number);
    println!(
        "Answer is: {:?}",
        match answer {
            Some(a) => a,
            None => panic!("No suitable values found!"),
        }
    );
}

fn get_max_prime_factor(primes: Vec<u64>, number: usize) -> Option<u64> {
    primes
        .par_iter()
        .filter(|x| number % **x as usize == 0)
        .cloned()
        .max()
}

fn generate_possible_prime_factors(number: &usize) -> Vec<u64> {
    let limit = (*number as f64).sqrt() as u64;
    let mut suitable_primes: Vec<u64> = Vec::new();
    let mut ps = PrimeSet::new();
    let mut ps_iter = ps.iter();
    loop {
        let next_prime = ps_iter.next().unwrap_or(0);
        match next_prime {
            n if n <= limit => suitable_primes.push(n),
            _ => break,
        }
    }
    suitable_primes
}
