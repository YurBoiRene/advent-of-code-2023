# Advent of Code 2023

My goal for this year's AOC is to become familiar with the Rust Programming Language.

My goal has 3 key parts:

- **Practicing** Rust's syntax, type system, and standard library so that writing simple code in Rust is second nature.
- **Learning** useful crates for use cases outside of the standard library (a common occurrence).
- **Internalizing** the Rust way of doing things, moving away from OOP and embracing composition and function concepts.

My goal is **not** to write perfect code, solve all of the challenges, or solve the challenges as fast as possible.

## Progress

| Puzzle | Completed            | Notes |
| :----: | :------------------: |-------|
| 1-1    | :white_check_mark:   | I enjoyed using `filter()`. I am pleased with the simplicity of the solution. |
| 1-2    | :white_check_mark:   | Took much more time than `1-2`. It took a while for me to figure out that something like eightwo was 82. This messed up my original regex solution. I am not proud of my reverse regex solution. I also learned a little about the `anyhow` and `thiserror` crates but I would want to use them properly sometime. |
| 2-1    | :white_check_mark:   | Practice with simple structs and learned about the `FromStr` trait. Very much enjoyed using iterators again. |
| 2-2    | :white_check_mark:   | Part 2 was simple thanks to a solid design in `2-1`. Again, iterators made the meat of the puzzle quite trivial. |
| 3-1    | :white_check_mark: | Difficult, but I learned a lot. Check out the [README](puzzle-3-1/README.md). |
| 3-2    | :white_check_mark: | Less difficult but exposed some design flaws in 3-1. Check out the [README](puzzle-3-2/README.md). |
| 4-1    | :white_check_mark: | Not too hard. I appreciate rusts string spiting and iterators. |
| 4-2    | :white_check_mark: | Fun!! I initially made a recursive implementation because that "made sense" but quickly realized it would take a loooong time to compute. This lead me to find a constant time solution |
| 5-1    | :white_check_mark: | Not to bad, fun. I learned about itertools' `collect_tuple()` which is a neat function. |
| 5-2    | :white_check_mark: | Interesting. I learned about 2 new things, the [`array_chunks()`](https://github.com/rust-lang/rust/issues/100450) experimental feature which I didn't end up using and the [`rayon`](https://docs.rs/rayon/latest/rayon/) crate for parallelism. I have a brute force solution that works decently well considering the simplicity of implementation. With rayon it takes under a minute to finish. |
| 6-1    | :white_check_mark: | Not too hard, I enjoyed using `take_while()` to find the minimum winning time. I also liked using ranges to cheaply calculate the amount of winning times. |
| 6-2    | :white_check_mark: | My efficient design in 6-1 made this trivial |
| 7-1    | :white_large_square: ||
| 7-2    | :white_large_square: ||
| 8-1    | :white_large_square: ||
| 8-2    | :white_large_square: ||
| 9-1    | :white_large_square: ||
| 9-2    | :white_large_square: ||
| 10-1   | :white_large_square: ||
| 10-2   | :white_large_square: ||
| 11-1   | :white_large_square: ||
| 11-2   | :white_large_square: ||
| 12-1   | :white_large_square: ||
| 12-2   | :white_large_square: ||
| 13-1   | :white_large_square: ||
| 13-2   | :white_large_square: ||
| 14-1   | :white_large_square: ||
| 14-2   | :white_large_square: ||
| 15-1   | :white_large_square: ||
| 15-2   | :white_large_square: ||
| 16-1   | :white_large_square: ||
| 16-2   | :white_large_square: ||
| 17-1   | :white_large_square: ||
| 17-2   | :white_large_square: ||
| 18-1   | :white_large_square: ||
| 18-2   | :white_large_square: ||
| 19-1   | :white_large_square: ||
| 19-2   | :white_large_square: ||
| 20-1   | :white_large_square: ||
| 20-2   | :white_large_square: ||
| 21-1   | :white_large_square: ||
| 21-2   | :white_large_square: ||
| 22-1   | :white_large_square: ||
| 22-2   | :white_large_square: ||
| 23-1   | :white_large_square: ||
| 23-2   | :white_large_square: ||
| 24-1   | :white_large_square: ||
| 24-2   | :white_large_square: ||
