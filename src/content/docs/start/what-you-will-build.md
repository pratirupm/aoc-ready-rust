---
title: "What you will build"
description: "The practice lab, its drills, and the toolkit you will keep for December."
---

On the first morning of December, a new Advent of Code puzzle appears. You read it, download the input file that the site makes for your account, and open a folder on your computer. Inside is a small Rust project with a file for the day, a test that knows the example answer, and a place for your input. You paste the text, write the parsing and the two parts, run one command, and get a number to submit.

That project is `rust-aoc-lab`. A Rust project is called a crate, and this crate is deliberately plain: it uses only Rust's standard library, so there is nothing extra to install and no library hiding the interesting part. You can read every line of it.

The lab is not a collection of finished solutions. It is a set of drills: small, original exercises that practise one puzzle skill at a time. The ground floor has its own short programs, and each of the seven days adds one drill. The drills are yours to break, rename, and extend. The tests tell you when a drill behaves as intended.

## One shape for every drill

Every drill answers the same three questions. How do I read this input? What is the first answer? What is the second? The names stay the same from the first day to the last, so a new puzzle feels familiar even when the puzzle itself is strange.

```rust title="src/bin/drillNN.rs"
pub struct Input {
    pub values: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    ...
}

pub fn part1(input: &Input) -> u64 {
    ...
}

pub fn part2(input: &Input) -> u64 {
    ...
}
```

`parse` runs once and turns text into an `Input`, a struct that bundles the values a drill needs. A `Vec` is a growable list, so `Input` holds a list of numbers. Both parts borrow that bundle — they read it without copying or taking it away — instead of reading the text again, which keeps the two answers consistent. `u64` is an unsigned integer: a whole number that cannot go below zero.

## Running a drill

Each drill has tests beside it. A test is a small program that calls your code with the example input and compares the result with the answer the example gives. We call the tests the oracle because they settle the question without argument: when they pass, your parsing and both parts agree with the example. Then you point the drill at the real input and read the answer.

The command is `cargo test --bin drill01`; `cargo` is Rust's build tool, and `--bin drill01` narrows the run to one drill.

```console
$ cargo test --bin drill01
   Compiling rust-aoc-lab v0.1.0 (/home/you/rust-aoc-lab)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running unittests src/bin/drill01.rs (target/debug/deps/drill01-...)

running 2 tests
test tests::drill01_part1_example ... ok
test tests::drill01_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The word after each test name is the result: `ok` means that part matches the example. Your timings and the path under `target` will differ; the two lines with `ok` should not.

## The toolkit you keep

The drills share a set of modules. A module is a named group of code in its own file, and each one in the lab covers a family of puzzle chores.

`parse` turns raw text into blocks, signed and unsigned integers, and single numbers. It handles the small annoyances that every input brings: trailing spaces, blank lines, and numbers that need to become a different type. `grid` gives you a `Grid` that stores every cell in one flat `Vec`, plus bounds-checked lookups and neighbour steps in four or eight directions.

The search code lives in the drills themselves, where you can read the whole algorithm: breadth-first search, flood fill, Dijkstra's algorithm, and path counting. A drill is short enough to hold in your head, and nothing is hidden behind a library call.

`dsu` is a disjoint set union, also called union-find. It keeps track of which items belong to the same group, merging groups and answering "are these two connected?" without walking the whole structure each time. `num` collects the arithmetic that puzzles keep asking for: greatest common divisors, least common multiples, modular powers, and the care needed when an integer grows past its width. `intervals` stores ranges, merges overlaps, and counts how many positions a set of ranges covers.

These modules start small. A day adds what the drill needs, and you type the addition rather than importing a mystery. By the end, the toolkit is code you have read, tested, and changed.

## The seven drills

The ramp moves from reading input to combining techniques.

| Day | What the drill practises |
| --- | --- |
| Day 1 | Parsing text into values, then walking a flat grid of cells |
| Day 2 | Breadth-first search, depth-first search, and flood fill |
| Day 3 | Dijkstra's algorithm, a priority queue, and carrying extra state |
| Day 4 | Memoisation, dynamic programming, and counting paths |
| Day 5 | Simulation, cycle detection, and skipping ahead |
| Day 6 | Integer widths, modular arithmetic, intervals, and compression |
| Day 7 | Triage, part-two patterns, and a short mock puzzle |

Day 7 is a rehearsal. You get a small puzzle you have not seen, a chance to choose a technique, and some practice at the first moves you will make in December. Nothing in the table is a promise that every real puzzle fits one box. The point is to have a box to reach for.

## When December arrives

You keep the lab. It does not disappear when the book ends, and it does not need an internet connection to run. On December 1 you can create a new file beside the drills, copy the shape above, paste your input, and work in the same rhythm you practised.

You will be able to read a puzzle statement and separate the input from the question. You will be able to choose a starting point from the toolkit, run the tests, and change one piece at a time when an answer is wrong. You will also know how to save a working state in Git before trying something risky. Those are the skills that make the month feel manageable.

Next: [learn how the book works](/start/how-this-book-works/).
