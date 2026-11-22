---
title: "The twelve-day playbook"
description: "Set up a project, run the daily loop, and finish December without burning out."
---

The first puzzle of December is close. In 2025 the event ran for twelve days, December 1 to December 12, with no global leaderboard. The [2026 calendar](https://adventofcode.com/2026) is the authority this year; check it before planning around a date. The [2025 about page](https://adventofcode.com/2025/about) explains why.

A month of puzzles goes better when the first morning is not also the first time you set up a project. The work before December is small: one folder, one settings file, twelve empty day files, and a place for inputs. After that, each morning has the same shape, and the shape carries you through a hard day.

This page covers the setup, the daily loop, the difficulty arc, the five common part-two twists, and the moves to make when you are stuck. It contains no puzzle text and no answers. The puzzles belong to Advent of Code; the routine belongs to you.

## Before December 1

Make a new project beside the lab. A Rust project is called a crate, and `cargo new` creates one with a settings file and a source folder. The `--vcs none` part says "do not create a Git repository for me"; we will do that ourselves.

```console
$ cargo new aoc2026 --vcs none
    Creating binary (application) `aoc2026` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Move into the folder with `cd aoc2026` before running anything else. A crate can hold several programs. `src/main.rs` is the small program Cargo wrote, and it can stay. A file in `src/bin/` becomes its own binary, named after the file, so `src/bin/day01.rs` builds `day01`. Make one file for each day, from `day01.rs` through `day12.rs`. The folder looks like this when the setup is done:

```text
aoc2026/
├── Cargo.toml
├── .gitignore
├── data/
│   ├── examples/
│   │   └── day01.txt
│   └── inputs/
└── src/
    ├── main.rs
    └── bin/
        ├── day01.rs
        ├── day02.rs
        └── ...
```

`--vcs none` left Git alone. Before the first commit, run `git init` in the project folder; Git prints the path to the hidden `.git` folder.

`data/examples/` holds the small example from each puzzle page. Those examples are safe to keep in Git because you type them yourself. `data/inputs/` holds the personal input file that the site gives your account. Inputs do not belong in a shared project, so Git ignores that folder.

The settings file stays short:

```toml title="Cargo.toml"
[package]
name = "aoc2026"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`edition` names the version of Rust the code is written for, and `2024` is current. `[dependencies]` lists outside libraries; leave it empty. The standard library has what the puzzles need, and no dependency means no version problem on a puzzle morning.

Create `.gitignore` beside `Cargo.toml` with two lines:

```text title=".gitignore"
/target
/data/inputs
```

The first line keeps build output out of Git. `target/` is large, it changes on every build, and it can be rebuilt from source. The second keeps your puzzle inputs private. The example files stay tracked because the tests read them.

Each day file uses the same three functions: `parse`, `part1`, and `part2`. `parse` turns text into a value, and the two parts take that value and return a number. A test module beside the functions holds one test per part. The test embeds the example with `include_str!`, a macro that reads a file while the program is built, so the test does not depend on the folder you run it from.

A day file starts with the same shape:

```rust title="src/bin/day01.rs"
const EXAMPLE: &str = include_str!("../../data/examples/day01.txt");

pub fn parse(input: &str) -> Vec<u64> {
    ...
}

pub fn part1(input: &[u64]) -> u64 {
    ...
}

pub fn part2(input: &[u64]) -> u64 {
    ...
}

fn main() {
    let input = std::fs::read_to_string("data/inputs/day01.txt")
        .expect("day01 input file");
    let data = parse(&input);
    println!("part1 = {}", part1(&data));
    println!("part2 = {}", part2(&data));
}
```

The `...` lines are places to fill in, not valid Rust. `main` reads the real input with `fs::read_to_string`, which returns a `Result`: the text or an error. `expect` unwraps the text and stops with a message when the file is missing.

Three commands cover the month. `cargo test` checks every day. `cargo test --bin day01` checks one day while you work. `cargo run --release --bin day01` runs that day against the real input; `--release` builds an optimised program, and a search that crawls in the debug build often finishes quickly.

Run `cargo test` on the last evening of November. A passing run means the toolchain, the editor, and the terminal agree. That is one less thing to discover at midnight.

## The daily loop

The loop is the same every morning. The puzzle changes; the order does not.

1. Read the page twice: once for the story, once for the rules, the input shape, and the detail you are likely to miss.
2. Copy the example into `data/examples/dayNN.txt`. The example is the only oracle before submission, the one answer you can check without the site.
3. Add a test for part one. Put the example answer in the test, then make it pass.
4. Write `parse` and print the number of lines, rows, columns, or values. If the count looks wrong, stop before writing the search.
5. Solve part one on the example. Run `cargo test --bin dayNN`. When it passes, run the real input and submit.
6. Read part two. Add its test before you change the code. Then solve it and submit.
7. Commit. `git add -A` stages every changed file, and `git commit -m "day 01"` records them. Hashes and branch names differ on your machine.
8. Stop for the day, even with a part still open. A fresh read finds what a tired one cannot.

The order matters at step three. A test written after the code tends to agree with it, mistakes included. A test written from the example is a statement about the puzzle, and it can disagree.

Part two does not appear until part one is accepted, so add its test as soon as the question appears. Keep `part1` and `part2` separate even when they share a search. Two small functions with two tests are easier to repair than one function answering both questions.

The commit is not decoration. It marks a state you can return to when an experiment goes wrong. Commit a working part, a half-written parser, or a page of notes; Git does not care whether the code is finished.

## The difficulty arc

The first four days usually ask you to read text, build a grid, and follow a simulation. The parsing is the puzzle: a header line, a list of pairs, a map with a border. These days are gentle on purpose. If you did the lab, the moves are familiar; the risk is going too fast and missing a rule.

The middle four days add the tools that make a search interesting. Breadth-first search finds the shortest path when every step costs the same. Dijkstra's algorithm handles steps with different prices. Dynamic programming counts arrangements by remembering smaller answers. Interval merging handles ranges that overlap. Each day adds one idea and expects you to carry the earlier ones.

The last four days are the wall. The statement may be shorter, the input larger, and the first idea too slow. The state space grows: a position plus a direction, a set of collected keys, a phase of a cycle. The arithmetic matters, and a number can pass the limit of the type you chose. This is where a new idea hides, and where the people who finish are the ones who wrote down what they knew before changing code.

Every year differs. The shape above is a tendency, not a promise. A gentle day can appear late, and a wall can appear on the sixth. Treat the first four days as a chance to build speed and the last third as a signal to slow down, not as a fixed map.

## Part two is a new question

Part two is not more of the same typing. It is a second question on the same input, often with the same parse. Five twists cover most of them.

- **Bigger.** The same question, on an input that makes the part-one approach too slow. The fix is a better data structure or a way to skip work, not a faster loop over the same states.
- **A new constraint.** One extra rule changes which moves are allowed. "No two threes in a row" is a constraint; so is "you may pass a wall once". The constraint usually belongs in the state.
- **Inverted.** Part one finds a path; part two counts the shortest paths. Part one simulates a system; part two asks when a state repeats. Turn the question around before you turn the code inside out.
- **A closed form.** Simulating every step is too slow, but the system repeats. Find the cycle, compute the remainder, and jump. The drill on simulation and cycle skipping is the rehearsal for this.
- **More state.** The answer depends on where you are and what you carry. Add one field to the state, widen the table, and run the same search over a larger set of states.

Most part-two work reuses the parse and one search from part one. Resist the urge to rewrite from scratch. Ask what stayed the same, then change only the piece the new question touches.

## When you are stuck

Stuck is normal. The first idea is often incomplete, and the puzzle expects you to think. Work through the list before you reach for a hint.

1. Re-read the statement, one sentence at a time, and read the rules aloud. Most wrong answers come from a rule remembered wrong.
2. Check the whole input: the first line, the last line, one line in the middle. Confirm the header, the blank lines, and the columns.
3. Print dimensions and counts. A grid that should be forty rows wide and is thirty-nine has a parsing bug, not a search bug.
4. Run the example again. If it fails, the bug is small and close. If it passes and the real input fails, the bug is in a rule the example does not exercise.
5. Write a smaller case and solve it by hand. Three cells and two rules are easier to trace than a thousand.
6. Take a break. Walk, drink water, sleep if it is late. The answer often arrives while you are doing something else.
7. Then read a hint: the daily thread, a friend, or a past solution. Read only enough to unblock the next step, then close the tab and write the code yourself.

One rule has no exception. Do not use AI during the attempt. The official request puts it plainly: "If you send a friend to the gym on your behalf, would you expect to get stronger?" The answer to whether to use AI to solve the puzzles is no. A solution you did not write teaches you nothing about the next puzzle. Read the full request on [the 2025 about page](https://adventofcode.com/2025/about).

A hint from a person is different from a finished solution. Ask what they would check next, not what the answer is. The first gives you a move; the second takes the move away.

## Private leaderboards and rest

The global leaderboard is gone. After ten years it had become a source of stress for the event and its users, so it was removed. The private leaderboard remains: a small board you share with friends, a class, or a group at work. Its owner sets the rules.

Midnight on the east coast of the United States is the early morning in Europe and the working day in Asia. The puzzle stays unlocked, so solve in the morning if that suits you. Sleep matters more than a midnight start.

Twelve days is a rhythm, not a sprint. Some days take an evening; some take a walk and a second evening. The puzzles remain available after December, so there is no penalty for finishing late.

## After the event

Keep the toolkit. The lab does not disappear when the book ends, and the `aoc2026` folder is still there in January. The parse helpers, the grid, the search patterns, and the tests are yours to reuse.

Revisit one hard day without a timer. Read your code first, then one solution from someone else. Compare the state they chose, not the lines they typed. Write the difference down and keep the pattern if it earns a place.

The next December will arrive with a new calendar and a new set of puzzles. The folder, the loop, and the habit of writing down what you know will still work.

Next: when the compiler disagrees, [the Rust error decoder](/appendix/rust-error-decoder/) turns its messages into plain words.
