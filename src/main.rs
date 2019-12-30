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
        5 => euler_project::five::solve(),
        6 => euler_project::six::solve(),
        7 => euler_project::seven::solve(),
        8 => euler_project::eight::solve(),
        9 => euler_project::nine::solve(),
        10 => euler_project::ten::solve(),
        11 => euler_project::eleven::solve(),
        _ => {
            let u: u8 = input::get_rand_u8(11);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            match_puzzle(u)
        }
    };
    let elapsed = start.elapsed();
    println!("Time elapsed: {}", humantime::format_duration(elapsed));
}
