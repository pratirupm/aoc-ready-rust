# Rust style

The lab is teaching code. It should be readable by someone who has never
written Rust, and it should still pass `cargo fmt`, `cargo clippy`, and
`cargo test`.

## Toolchain

- Edition 2024.
- `rustfmt` with `max_width = 100`.
- `clippy` with warnings treated as errors before a commit.

## Shape

1. Types.
2. Their `impl` blocks.
3. `main`.
4. Other functions.
5. Tests.

`rustfmt` will not move items, so this order is up to us. In the chapter, add
code where the step says; a full listing may group it for reading.

## Keep it plain

- **Concrete types beat clever types.** `Vec<u8>`, `usize`, `u64`, `i64`, and
  `HashMap<(i32, i32), u32>` are enough for the core path.
- **No lifetimes** in the core path. Return owned values, or borrow the input
  for the length of one function.
- **No generics or custom traits** in the core path. A page may introduce one
  in "Going deeper" when the reader has seen the concrete version first.
- **No external crates** in the core lab. `std` only.
- **Prefer `for` and `while`** in the main example. One short iterator chain is
  fine in "Going deeper".
- **Explicit types** where the reader is meeting a type for the first time.
- **`unwrap()` is allowed** in teaching code, with a sentence saying what it
  does. Introduce `?` only near the end of the ground floor.
- **Small functions.** If a function does not fit in a 25-line code block, split
  it into named steps.
- **Comments explain why.** Do not narrate what the line already says.

## Names

- `parse`, `part1`, `part2`, `solve`.
- `Grid`, `Point`, `Dsu`, `Interval`.
- `width`, `height`, `row`, `col`, `cells`.
- `dist`, `seen`, `queue`, `heap`, `cost`.
- `drill01`, `drill02`, ... for binaries.

## The day contract

Every drill uses the same shape:

```rust
pub fn parse(input: &str) -> Input
pub fn part1(input: &Input) -> u64
pub fn part2(input: &Input) -> u64
```

Parse once. Both parts borrow the parsed input. The example is embedded with
`include_str!` in tests; the real input is read at runtime.

## Tests

- Every drill has one test per part against the example input.
- The test file is the oracle. Do not put answers in the prose.
- A test name says what it checks: `drill01_part1_example`.
- Run `cargo test` from `rust-aoc-lab/`.
- Run `cargo test --release` when a drill is slow in the dev profile.

## Checks before a commit

```console
$ cargo fmt
$ cargo clippy --all-targets -- -D warnings
$ cargo test
```
