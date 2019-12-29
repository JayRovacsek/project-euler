mod euler_project;
mod input;

extern crate humantime;
extern crate rayon;
extern crate rand;
extern crate primes;

use std::time::Instant;

fn main() {
    loop {
        let option: u8 = input::get_option("Project Euler ID?");
        match_puzzle(option);
    }
}

fn match_puzzle(option: u8) {
    let start = Instant::now();
    match option {
        1 => euler_project::one::solve(),
        2 => euler_project::two::solve(),
        3 => euler_project::three::solve(),
        4 => euler_project::four::solve(),
        _ => {
            let u: u8 = input::get_rand_u8(4);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            match_puzzle(u)
        }
    };
    let elapsed = start.elapsed();
    println!("Time elapsed: {}", humantime::format_duration(elapsed));
}
