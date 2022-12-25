# Advent of Code

My solutions to the [Advent of Code](https://adventofcode.com/) challenge.
I mostly use AoC as a way to learn new languages, so don't expect quality or idiomatic code :)

So far I've done:

* 2022
  - [Rust](./2022/rust) (**Complete**)
* 2016
  - [Rust](./2016/rust) (**10/25**)
* 2021
  - [Rust](./2021/rust) (**Complete**)
* 2020
  - [Rust](./2018/rust) (**Complete**), with a bit of Python code for sanity checks.
* 2018
  - [Rust](./2018/rust) (**Complete**) 
  - [Go](./2018/go) (**Mostly complete**)
* 2015
  - [Rust](./2015/rust). I took a long break, so the solutions use two different structures.


## Rust

Rust solutions that use the `aoc_utils` crate (`utils/rust`) will automatically download input files if they are not found in the expected folder (`inputs` by default, or `../inputs` if that folder already exist).
The download will fail unless there is a file named `.aoc-session` in the directory hierarchy of the `$CWD` with the content of the `session` cookie.

The crate will try to guess the year you're solving through the `PKG_NAME` variable (in my crates that's `aoc2022`, `aoc2021`, etc.).
You may override that value through the `-y` flag:

```
cargo run -y 2099
```
