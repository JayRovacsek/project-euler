mod test;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub fn solve() {
    let answer = find_evenly_divisible_number(20);
    println!("Answer is: {:?}", answer);
}

fn find_evenly_divisible_number(limit: u32) -> u32 {
    let numbers = (1..=limit).collect::<Vec<u32>>();
    let mut i = (1..=limit).sum();
    let mut result = 0;
    while result == 0 {
        let r = numbers.iter().filter(|x| i % *x != 0).collect::<Vec<&u32>>();
        if r.is_empty() {
            println!("Adding to list");
            result = i
        } else {
            i += 1;
        }
    };
    result
}
