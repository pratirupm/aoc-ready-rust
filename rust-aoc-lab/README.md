# rust-aoc-lab

The zero-dependency practice crate for *Rust Before December*.

```console
$ cargo test
$ cargo run --bin drill01
```

## What is here

| Path | Job |
| --- | --- |
| `src/parse.rs` | split blocks, read integers |
| `src/grid.rs` | a flat byte grid and its neighbours |
| `src/dsu.rs` | disjoint-set union |
| `src/num.rs` | gcd, lcm, modular powers, `rem_euclid` |
| `src/intervals.rs` | merge inclusive intervals, measure them |
| `src/bin/drill01.rs` … `drill07.rs` | one original drill per day |
| `data/examples/` | the original example inputs (committed) |
| `data/inputs/` | your real Advent of Code inputs (gitignored) |

## The day contract

Every drill has the same shape:

```rust
pub fn parse(input: &str) -> Input
pub fn part1(input: &Input) -> u64
pub fn part2(input: &Input) -> u64
```

The example is embedded with `include_str!`. The tests are the oracle. Run one
drill's tests with:

```console
$ cargo test --bin drill03
```

For a real input, read the file at runtime instead:

```rust
let input = std::fs::read_to_string("data/inputs/03.txt").unwrap();
```

Never commit `data/inputs/`.
