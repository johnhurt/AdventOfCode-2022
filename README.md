<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2022/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2022/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2022/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2022/day/12) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2022/day/13) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2022/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2022/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2022/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2022/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2022/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2022/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2022/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2022/day/21) | ⭐ | ⭐ |
| [Day 22](https://adventofcode.com/2022/day/22) | ⭐ | ⭐ |
| [Day 23](https://adventofcode.com/2022/day/23) | ⭐ | ⭐ |
| [Day 24](https://adventofcode.com/2022/day/24) | ⭐ | ⭐ |
| [Day 25](https://adventofcode.com/2022/day/25) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/day_1.rs) | `0.1ns` | `0.1ns` |
| [Day 2](./src/day_2.rs) | `0.1ns` | `0.1ns` |
| [Day 3](./src/day_3.rs) | `0.1ns` | `0.1ns` |
| [Day 4](./src/day_4.rs) | `0.1ns` | `0.1ns` |
| [Day 5](./src/day_5.rs) | `0.0ns` | `0.1ns` |
| [Day 6](./src/day_6.rs) | `0.0ns` | `0.0ns` |
| [Day 7](./src/day_7.rs) | `0.2ns` | `0.2ns` |
| [Day 8](./src/day_8.rs) | `0.3ns` | `1499.0µs` |
| [Day 9](./src/day_9.rs) | `1265.0µs` | `1296.0µs` |
| [Day 10](./src/day_10.rs) | `0.0ns` | `0.0ns` |
| [Day 11](./src/day_11.rs) | `0.1ns` | `5362.0µs` |
| [Day 12](./src/day_12.rs) | `1310.0µs` | `53167.0µs` |
| [Day 13](./src/day_13.rs) | `1546.0µs` | `0.5ns` |
| [Day 14](./src/day_14.rs) | `0.9ns` | `2392.0µs` |
| [Day 15](./src/day_15.rs) | `0.3ns` | `518650.0µs` |
| [Day 16](./src/day_16.rs) | `24161.0µs` | `3409.8ms` |
| [Day 17](./src/day_17.rs) | `1099.0µs` | `50946.0µs` |
| [Day 18](./src/day_18.rs) | `3620.0µs` | `7807.0µs` |
| [Day 19](./src/day_19.rs) | `8151.0ms` | `942123.0µs` |
| [Day 20](./src/day_20.rs) | `32162.0µs` | `383251.0µs` |
| [Day 21](./src/day_21.rs) | `2209.0µs` | `1342.0µs` |
| [Day 22](./src/day_22.rs) | `2995.0µs` | `1591.0µs` |
| [Day 23](./src/day_23.rs) | `13030.0µs` | `846553.0µs` |
| [Day 24](./src/day_24.rs) | `1455.2ms` | `3927.7ms` |
| [Day 25](./src/day_25.rs) | `1.0ns` | `0.0ns` |

**Total: 19847.44ms**
<!--- benchmarking table --->

---

## Usage

This year uses a single binary with per-day modules instead of the template layout:

```sh
cargo run --release          # run all days
cargo run --release -- --day-1   # run a single day
cargo run --release -- --day-1 --p2   # run a single problem
cargo run --release -- -x     # run against the example inputs
./day.sh 3                    # scaffold a new day (module + input files)
```

Puzzle inputs live in `input/` and are gitignored — download them with
[aoc-cli](https://github.com/scarvalhojr/aoc-cli/): `aoc download -y 2022 -d 1 -i input/day_1.txt`.

_Benchmarks are single-run timings of each solution on real inputs, measured inside the program (release build)._
