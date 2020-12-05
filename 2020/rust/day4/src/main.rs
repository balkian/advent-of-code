#![feature(destructuring_assignment, or_patterns)]
use std::env;

mod hard;
mod hashmap;

use hard::{solve, solve_par, solve_par2};
use hashmap::{solve_hashmap};

fn main() {
    let args: Vec<String> = env::args().collect();
    pretty_env_logger::init();
    let it = aoc_utils::file_iter_plain();
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
            solve_hashmap(it, false);
        }
        Some("--hashmap-par") => {
            println!("Running in parallel using a Hashmap ");
            solve_hashmap(it, true);
        },
        Some("--help") => {
            println!("Available methods: --par2, --par, --hashmap, --hashmap-par");
        }
        _ => {
            println!("Solving using a struct and single-threaded code.");
            solve(it);
        }
    }
}
