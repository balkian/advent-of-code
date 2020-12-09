#![feature(destructuring_assignment, or_patterns)]

mod hard;
mod hashmap;

use hard::{solve, solve_channel, solve_rayon};
use hashmap::{solve_hashmap, solve_hashmap2, solve_hashmap2_par, solve_hashmap_par};

use aoc_utils::clap;
use aoc_utils::clap::arg_enum;

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Variant {
        Simple,
        Thread,
        Rayon,
        Hashmap,
        HashmapPar,
        Hashmap2,
        Hashmap2Par,
    }
}

fn main() {
    pretty_env_logger::init();

    let matches = aoc_utils::app("4")
        .version("2.0")
        .arg(
            clap::Arg::with_name("variant")
                .long("variant")
                .possible_values(&Variant::variants())
                .default_value("Simple"),
        )
        .get_matches();

    let it = aoc_utils::file_iter_clap(&matches);

    let variant = clap::value_t_or_exit!(matches.value_of("variant"), Variant);

    match variant {
        Variant::Rayon => {
            println!("Running in parallel using rayon");
            solve_rayon(it);
        }
        Variant::Thread => {
            println!("Running in parallel using a thread");
            solve_channel(it);
        }
        Variant::Hashmap => {
            println!("Running synchronously using a Hashmap ");
            solve_hashmap(it);
        }
        Variant::HashmapPar => {
            println!("Running in parallel using a Hashmap ");
            solve_hashmap_par(it);
        }
        Variant::Hashmap2 => {
            println!("Running a Hashmap in functional style using helpers from aoc_utils");
            solve_hashmap2(it);
        }
        Variant::Hashmap2Par => {
            println!(
                "Running a Hashmap in parallel in functional style using helpers from aoc_utils"
            );
            solve_hashmap2_par(it);
        }
        _ => {
            println!("Solving using a struct and single-threaded code.");
            solve(it);
        }
    }
}
