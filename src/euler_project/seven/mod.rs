mod test;

use primes::PrimeSet;

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

pub fn solve() {
    let mut ps = PrimeSet::new();
    for prime in ps.iter().skip(10000).take(1) {
        println!("Answer is: {:?}", prime);
    };
}