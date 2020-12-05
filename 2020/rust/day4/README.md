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

# Logs 

For future reference, here are the results for every implementation:

```
 ❯ time cargo run --release input-big.txt --par
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --par`
Running in parallel using a thread
Valid: 5644801 / 7049729
Valid2: 4014080 / 7049729
cargo run --release input-big.txt --par  108.55s user 4.29s system 640% cpu 17.606 total
>>> elapsed time 18s

 j@panther ~/g/b/a/2/r/day4    master ⁝ ✱ ?
 ❯ time cargo run --release input-big.txt --par2                                             [10:40:35]
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --par2`
Running in parallel using rayon
Valid: 5644801 / 7049729
Valid2: 4014080 / 7049729
cargo run --release input-big.txt --par2  108.53s user 4.39s system 651% cpu 17.320 total
>>> elapsed time 17s

 ❯ time cargo run --release input-big.txt --hashmap-par
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --hashmap-par`
Running in parallel using a Hashmap
Valid in part 1: 5644801
Valid in part 2: 4014080
cargo run --release input-big.txt --hashmap-par  195.98s user 146.63s system 775% cpu 44.179 total
>>> elapsed time 45s

 ❯ time cargo run --release input-big.txt --hashmap-par2
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --hashmap-par2`
Solving using a struct and single-threaded code.
Valid: 5644801 / 7049729
Valid2: 4014080 / 7049729
cargo run --release input-big.txt --hashmap-par2  53.18s user 2.15s system 109% cpu 50.430 total
>>> elapsed time 50s

 ❯ time cargo run --release input-big.txt --hashmap
    Finished release [optimized] target(s) in 0.01s
     Running `/home/j/git/balkian/advent-of-code/2020/rust/target/release/day4 input-big.txt --hashmap`
Running synchronously using a Hashmap
Valid in part 1: 5644801
Valid in part 2: 4014080
cargo run --release input-big.txt --hashmap  15.06s user 0.08s system 99% cpu 15.163 total

```
