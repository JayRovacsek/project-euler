mod test;

use primes::PrimeSet;

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

pub fn solve() {
    let answer = prime_sum(2_000_000);
    println!("Answer is: {:?}", answer);
}

fn prime_sum(limit: u64) -> u64 {
    let mut pset = PrimeSet::new();
    let (n, _) = pset.find(limit);
    pset.iter().take(n).fold(0, |a, b| a + b)
}
