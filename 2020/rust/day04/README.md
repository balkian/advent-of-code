When working on the rust version, I stumbled upon an off-by-one error reading the file. I quickly put together a solution in Python to compare the results.

Since I had that anyway, I took some extra time to compare both implementations in terms of memory and CPU usage.
I used a 500MB file, made from concatenating the original `input.txt`:

```
 du -xsh input-big.txt
493M    input-big.txt
```

The first Rust version I came up with used ~3GB of RAM, and took around 50s.
The Python version used +10GB of RAM, and took about the same time.

```
 ❯ time python check.py input-big.txt
Part 1 5644801 7049729
Part 2 4014080 7049729
python check.py input-big.txt  45.12s user 3.89s system 99% cpu 49.467 total
>>> elapsed time 49s

 ❯ time ../target/release/day4 input-big.txt
Valid: 5644801 / 7049729
Valid2: 4014080 / 7049729
../target/release/day4 input-big.txt  48.01s user 1.20s system 99% cpu 49.285 total
>>> elapsed time 50s
```


Then I re-implemented the problem in Rust, using hashmaps and a functional style.
It barely used any RAM at all, and run much faster than the previous two:


```
 ❯ time cargo run --release input-big.txt --hashmap
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --hashmap`
Running synchronously using a Hashmap
Valid in part 1: 5644801
Valid in part 2: 4014080
cargo run --release input-big.txt --hashmap  14.63s user 0.06s system 99% cpu 14.705 total
```

I tried parallelizing this version using the rayon library.
This is not really a good fit for Rayon because the file iterator cannot be safely shared acrossed threads, so it means we have to rely on `par_bridge`, which is not as performant as other solutions.
Interestingly, it took even longer than the single-threaded version:

```
 ❯ time cargo run --release input-big.txt --hashmap-par
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --hashmap-par`
Running in parallel using a Hashmap
Valid in part 1: 5644801
Valid in part 2: 4014080
cargo run --release input-big.txt --hashmap-par  196.47s user 146.90s system 753% cpu 45.546 total
```

I'm probably doing something wrong with the references, and it must be copying data needlessly around threads.
I might revisit this some other time.

# Hyperfine 

For future reference, here are the results for every rust implementation using hyperfine and a 8MB file:

```
❯ du -xsh input-big.txt
131M    input-big.txt
❯ hyperfine --prepare 'cargo build --release'  '../target/release/day4 --variant {variant} input-big.txt' -L variant Simple,Rayon,Channel,Hashmap,HashmapPar,Hashmap2
Benchmark #1: ../target/release/day4 --variant Simple input-big.txt
  Time (mean ± σ):     13.270 s ±  0.099 s    [User: 13.945 s, System: 0.636 s]
  Range (min … max):   13.160 s … 13.428 s    10 runs

Benchmark #2: ../target/release/day4 --variant Rayon input-big.txt
  Time (mean ± σ):      4.681 s ±  0.064 s    [User: 28.383 s, System: 1.253 s]
  Range (min … max):    4.589 s …  4.764 s    10 runs

Benchmark #3: ../target/release/day4 --variant Channel input-big.txt
  Time (mean ± σ):      4.920 s ±  0.091 s    [User: 29.594 s, System: 1.386 s]
  Range (min … max):    4.826 s …  5.098 s    10 runs

Benchmark #4: ../target/release/day4 --variant Hashmap input-big.txt
  Time (mean ± σ):      3.884 s ±  0.048 s    [User: 3.845 s, System: 0.031 s]
  Range (min … max):    3.819 s …  3.987 s    10 runs

Benchmark #5: ../target/release/day4 --variant HashmapPar input-big.txt
  Time (mean ± σ):     11.582 s ±  0.260 s    [User: 51.493 s, System: 34.467 s]
  Range (min … max):   11.030 s … 11.886 s    10 runs

Benchmark #6: ../target/release/day4 --variant Hashmap2 input-big.txt
  Time (mean ± σ):      4.888 s ±  0.089 s    [User: 4.859 s, System: 0.025 s]
  Range (min … max):    4.778 s …  5.009 s    10 runs

Summary
  '../target/release/day4 --variant Hashmap input-big.txt' ran
    1.21 ± 0.02 times faster than '../target/release/day4 --variant Rayon input-big.txt'
    1.26 ± 0.03 times faster than '../target/release/day4 --variant Hashmap2 input-big.txt'
    1.27 ± 0.03 times faster than '../target/release/day4 --variant Channel input-big.txt'
    2.98 ± 0.08 times faster than '../target/release/day4 --variant HashmapPar input-big.txt'
    3.42 ± 0.05 times faster than '../target/release/day4 --variant Simple input-big.txt'
```
