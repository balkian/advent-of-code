#![feature(destructuring_assignment, or_patterns)]
use std::env;

mod hard;
mod hashmap;

use hard::{solve, solve_par, solve_par2};
use hashmap::{solve_hashmap, solve_hashmap2, solve_hashmap_par};

fn main() {
    let args: Vec<String> = env::args().collect();
    pretty_env_logger::init();
    let it = aoc_utils::file_iter();
    match args.get(2).map(String::as_str) {
        Some("--par2") => {
            println!("Running in parallel using rayon");
            solve_par2();
        }
        Some("--par") => {
            println!("Running in parallel using a thread");
            solve_par(it);
        }
        Some("--hashmap") => {
            println!("Running synchronously using a Hashmap ");
            solve_hashmap(it);
        }
        Some("--hashmap-par") => {
            println!("Running in parallel using a Hashmap ");
            solve_hashmap_par(it);
        }
        Some("--hashmap2") => {
            println!("Running a Hashmap in functional style using helpers from aoc_utils");
            solve_hashmap2();
        }
        Some("--help") => {
            println!("Available methods: --par2, --par, --hashmap, --hashmap-par --hashmap2");
        }
        _ => {
            println!("Solving using a struct and single-threaded code.");
            solve(it);
        }
    }
}
