## advent-of-code-2022
Advent of Code 2022 in Rust. All puzzles can be found on the [AoC 2022 website][aoc-link]. 

## Installation and Usage

1. Clone this repo and move into the directory:
```shell
git clone https://github.com/heuristic-pedals/advent-of-code-2022
cd advent-of-code-2022
```

2. Create an `input.txt` and add the the puzzle input. Add the file in the respective days' `data/` directory following the convention `data/day_XX/input.txt` (where XX is the day number, with a leading 0 for the first 9 days).

3. Build the compiled binary using `cargo`:
```shell
cargo build --release
```

4. Execute the day of interest, where `<DAY_NUMBER>` is the day of interest (between 1 and 25 inclusive):
```shell
./target/release/advent-of-code-2022 <DAY_NUMBER>
```

5. The puzzle solutions will be printed in the terminal.

[aoc-link]: https://adventofcode.com/2022