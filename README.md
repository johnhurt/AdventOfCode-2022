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
| [Day 1](./src/day_1.rs) | `128.0µs` | `129.0µs` |
| [Day 2](./src/day_2.rs) | `144.0µs` | `138.0µs` |
| [Day 3](./src/day_3.rs) | `75.0µs` | `71.0µs` |
| [Day 4](./src/day_4.rs) | `91.0µs` | `84.0µs` |
| [Day 5](./src/day_5.rs) | `50.0µs` | `65.0µs` |
| [Day 6](./src/day_6.rs) | `10.0µs` | `11.0µs` |
| [Day 7](./src/day_7.rs) | `200.0µs` | `198.0µs` |
| [Day 8](./src/day_8.rs) | `183.0µs` | `728.0µs` |
| [Day 9](./src/day_9.rs) | `662.0µs` | `684.0µs` |
| [Day 10](./src/day_10.rs) | `30.0µs` | `32.0µs` |
| [Day 11](./src/day_11.rs) | `49.0µs` | `5.0ms` |
| [Day 12](./src/day_12.rs) | `399.0µs` | `47.0ms` |
| [Day 13](./src/day_13.rs) | `413.0µs` | `493.0µs` |
| [Day 14](./src/day_14.rs) | `551.0µs` | `2.5ms` |
| [Day 15](./src/day_15.rs) | `29.0µs` | `517.7ms` |
| [Day 16](./src/day_16.rs) | `23.9ms` | `3.4s` |
| [Day 17](./src/day_17.rs) | `154.0µs` | `50.3ms` |
| [Day 18](./src/day_18.rs) | `914.0µs` | `8.1ms` |
| [Day 19](./src/day_19.rs) | `8.1s` | `942.0ms` |
| [Day 20](./src/day_20.rs) | `30.4ms` | `374.5ms` |
| [Day 21](./src/day_21.rs) | `851.0µs` | `792.0µs` |
| [Day 22](./src/day_22.rs) | `824.0µs` | `492.0µs` |
| [Day 23](./src/day_23.rs) | `8.0ms` | `735.7ms` |
| [Day 24](./src/day_24.rs) | `1.5s` | `3.9s` |
| [Day 25](./src/day_25.rs) | `90.0µs` | `-` |

**Total: 19657.01ms**
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

_Benchmarks are the median of 5 runs per solution on real inputs (release build). Timings are measured inside the program and include reading the input._
