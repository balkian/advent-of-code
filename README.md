# Advent of Code

My solutions to the [Advent of Code](https://adventofcode.com/) challenge.
I mostly use AoC as a way to learn new languages, so don't expect quality or idiomatic code :)

So far I've done:

* 2025
  - [Rust](./2025/rust) (**In progress**)
* 2024
  - [Rust](./2024/rust) (**Complete**)
* 2023
  - [Rust](./2023/rust) (**Complete**)
* 2022
  - [Rust](./2022/rust) (**Complete**)
* 2021
  - [Rust](./2021/rust) (**Complete**)
* 2020
  - [Rust](./2018/rust) (**Complete**), with a bit of Python code for sanity checks.
* 2018
  - [Rust](./2018/rust) (**Complete**) 
  - [Go](./2018/go) (**Mostly complete**)
* 2016
  - [Rust](./2016/rust) (**10/25**)
* 2015
  - [Rust](./2015/rust). (**Complete**, but I took a long break in between, so the solutions use two different project structures.

## Project structure
### Rust (2016, 2018, 2021, 2022, 2023, 2024, 2025)

These Rust solutions use the `aoc_utils` crate (`utils/rust`).
The solutions for each day are in a file named `src/solutions/day<number>.rs`.

The binary will automatically download input files if they are not found in the expected folder (`../inputs`).
To download a file you need a session token (cookie), which should be stored in a file named `.aoc-session` in any directory in the hierarchy of the `$CWD`.
The downloader needs to know which year and day to download.
The day is determined by the arguments and the solutions implemented, and the year is computed from `PKG_NAME` variable (in my crates that's `aoc2022`, `aoc2021`, etc.).
You may override that value with the `-y` flag:

```
cargo run -y 2099
```

### Rust (2015, 2020 and 2021)

The solution for each day has its own project and binary.
The input file is included in each project's folder.
