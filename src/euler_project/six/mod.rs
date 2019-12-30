mod test;

// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub fn solve() {
    let a = square_of_sum(100);
    let b = sum_of_squares(100);
    println!("Answer is: {:?}", a - b);
}

fn sum_of_squares(limit: usize) -> usize {
    (1..=limit)
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |a, b| a + (b * b))
}

fn square_of_sum(limit: usize) -> usize {
    let x: usize = (1..=limit).sum();
    x * x
}
