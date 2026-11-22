---
title: "Put it together"
description: "Triage a puzzle, recognise the part-two pattern, and run a short mock."
---

The last day of the ramp does not add another algorithm. You already have a grid, a queue, a table of things you have seen, a simulation that can skip ahead, and enough arithmetic to keep numbers in range. What is missing is a way to begin when the statement is new and the clock is running.

That is what this day is for. We build a routine for the first few minutes, then look at the shapes a part two usually takes. After that we solve one small puzzle from the first line of input to the last test, and set up a mock so the first morning of December feels familiar.

This is the day the book stops handing you pieces and starts handing you the whole board.

## Triage

Triage is the habit of sorting what matters now from what can wait. On a puzzle morning, almost everything can wait except the shape of the input and the two questions.

The routine below is the one to follow when the page is new. It has seven steps, and the first three happen before you write a line of Rust.

1. **Read the statement twice.** The first read is for the story. The second read is for the shape: what one line of input looks like, what part one asks for, and what the example produces. Write those three things down. If you cannot fill all three, read a third time.
2. **Copy the example into a file.** The example is small, and it is the only input whose answer you can check by hand. Put it in `data/examples/`, named after the day, the same way the lab stores `07.txt`.
3. **Write the two tests.** One test calls part one with the example and compares the result with the answer the statement prints. The other does the same for part two. Both tests fail at first, because the functions do not exist yet.
4. **Write the parser.** One function, one return type. Turn the text into the thing the puzzle is about: a grid, a list of numbers, a list of rules. Print the size of what you parsed and compare it with the statement.
5. **Solve part one on the example.** Write the smallest code that turns the example into the expected answer. Run the test. Do not touch the real input until the example passes.
6. **Run part one on the real input.** Save the input in a file, run the program, and submit the number. If the number is wrong and the example passed, the section on wrong answers below is the place to look.
7. **Read part two before changing part one.** Say the new question in one sentence, out loud if the room allows it. Then decide which of the five twists it is. Only after that do you open the editor.

Steps three and four look backwards if you have never worked this way. They are the reason the rest of the routine is calm. A test written after the code describes what the code already does. A test written before the code describes what the puzzle asks. The second kind can still disagree with you at the end, and that disagreement is information.

The example is not a warm-up. It is a second input, small enough to trace with a finger, and it is the only place where a wrong answer is cheap. When the real input disagrees and the example agrees, you have narrowed the problem to scale, parsing, or a hidden rule. That is a much smaller search than "something is wrong".

:::tip
Write the input shape on paper before the parser. "Each line is a number, then a space, then a letter" is a sentence that catches half of all parsing bugs before they are born.
:::

## Part two is a new question

Part two usually arrives with the same story and the same input file. It is tempting to read it as "part one, but harder". Most of the time it is a different question about the same data, and the difference is one of five twists.

**A bigger input.** The same process now runs for a number of steps you cannot simulate one at a time. The answer is not a faster loop; it is a loop that notices a repeated state and skips ahead. The lab's fifth drill does this with a checksum that repeats every few ticks, and the cycle-skip pattern is in [Simulate, then skip ahead](/part-5/simulate-then-skip/).

**A new constraint.** The first part allowed any sequence of moves. The second part forbids one kind of move from following another. The search space stays the same size; the state grows by one field, and that field joins the key. The lab's fourth drill adds "no two threes in a row" and the answer changes because the state changed.

**The process runs backwards.** Instead of asking what comes out of a machine, the puzzle asks what goes in, or how many starting points reach a target. Running the rules in reverse is a different algorithm, not a bigger loop. Day 19 of 2015 is the classic example: the forward rewriting is short to describe and painful to search, while the reverse direction is a small, bounded search.

**A closed form replaces the loop.** Part one sums a hundred values; part two sums a hundred billion. If the values follow a rule, you can jump. Arithmetic series, least common multiples, and interval lengths all turn a loop into a formula. The lab's sixth drill uses the least common multiple of several cycles, and [Do the math](/part-6/do-the-math/) builds the habit.

**New state joins the position.** The map is the same, but the traveller now carries a direction, a set of keys, or a count of steps left. The position alone is no longer a state. The lab's third drill charges extra for turning, so the search key becomes "cell and direction" instead of "cell".

Before you write part two, name the twist. If none of the five fits, the answer is often two of them at once: a bigger input and a new field in the key. Saying the sentence first saves an hour of editing the wrong function.

## A guided composite

The lab's seventh drill is a complete small puzzle with both parts. It is a grid with a start, an exit, walls, and open floor.

```text title="rust-aoc-lab/data/examples/07.txt"
#####
#S..#
#.#.#
#..E#
#####
```

`S` is the start, `E` is the exit, `#` is a wall, and `.` is open floor. The outer wall closes the grid, so the only way out of a cell is through a neighbour that is not a wall. The example is built so that more than one walk ties for the shortest length, which is what makes part two worth asking.

The parser turns the text into a `Grid` and two cell numbers. A cell number is a single `usize` index into the flat `cells` vector; the grid module converts between that index and a row and column.

The first search is a breadth-first search. `VecDeque` is a queue from the standard library: you push on the back and pop from the front. `usize::MAX` is the largest number a `usize` can hold, and here it means "not reached yet".

```rust title="rust-aoc-lab/src/bin/drill07.rs"
fn distances(grid: &Grid, start: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; grid.cells.len()];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(index) = queue.pop_front() {
        for next in grid.neighbors4(index) {
            if grid.cells[next] == b'#' || dist[next] != usize::MAX {
                continue;
            }
            dist[next] = dist[index] + 1;
            queue.push_back(next);
        }
    }

    dist
}
```

Every edge costs one step, so the queue visits cells in rings: all cells at distance one, then all at distance two, and so on. The first time a cell is reached is therefore the shortest distance to it. Part one reads `dist[exit]` and returns it, with a zero for the unreachable case.

Part two asks how many different shortest walks reach the exit. A walk is a sequence of steps, and two walks count as different if they take different cells in a different order. The trick is to count in the same ring order the search already produced.

```rust title="rust-aoc-lab/src/bin/drill07.rs"
    let mut ways = vec![0u64; grid.cells.len()];
    ways[start] = 1;

    let mut order: Vec<usize> = (0..grid.cells.len())
        .filter(|&index| dist[index] != usize::MAX)
        .collect();
    order.sort_by_key(|&index| dist[index]);

    for index in order {
        for next in grid.neighbors4(index) {
            if dist[next] == dist[index] + 1 {
                ways[next] = (ways[next] + ways[index]) % MOD;
            }
        }
    }

    ways[exit]
```

The `filter` line keeps only the cells the search reached, `collect` gathers them into a `Vec`, and `sort_by_key` orders them by distance. Every open edge moves exactly one ring forward, so a cell only receives counts from the ring before it. When the loop reaches a cell, every contributor has already been counted.

`ways[start]` begins at one because there is one empty walk at the start. Each open edge that leads one ring forward adds the count from the cell behind it. The modulo keeps the number inside a `u64`; the drill uses a large prime out of habit, and real puzzles say in the statement when they want a remainder.

Run the tests:

```console
$ cargo test --bin drill07
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/bin/drill07.rs (target/debug/deps/drill07-3d1320a4a0c4f45c)

running 2 tests
test tests::drill07_part1_example ... ok
test tests::drill07_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The two `ok` lines are the oracle. The hash in the runner path and the finishing time will differ on your machine; the count of passing tests will not.

This is the shape most grid puzzles take: one search that produces a distance table, then a second question that reads the table in a different way. Part one measures. Part two counts. Both use the same walk.

## A short mock

Choose two past puzzles you have not read, and treat them as a rehearsal. Two useful ones are [Day 3 of 2024](https://adventofcode.com/2024/day/3), which is a parsing day, and [Day 12 of 2022](https://adventofcode.com/2022/day/12), which is a grid search with a height rule. Between them they cover the two skills that decide most mornings: reading messy text, and moving through a grid.

The protocol is the same for both.

1. Read the page twice. Write the input shape and both questions on paper.
2. Copy the example into `data/examples/`, and write one test per part.
3. Make part one pass on the example. Then part two.
4. Run both parts on the real input. Submit part one, then part two.
5. If a part is still wrong, stop and write down what you know before changing code.

Then add the timer. Set a timer before part two. When it rings, write down what you know: the shape of the input, the archetype you chose, the part that is slow or wrong, and the first thing you would check. Stand up and walk away for a moment. Coming back to a written state beats staying at the keyboard and changing lines at random.

The mock is not about the stars. It is about the moment when the page stops being a wall of text and becomes a list of steps you have already taken.

## The morning of December 1

There is a short list of things to have ready. None of them is clever, and all of them save time when the puzzle appears.

- **The project builds.** Run `cargo test` in `rust-aoc-lab` the night before. A passing run means the toolchain, the editor, and the terminal all agree with each other.
- **The example folder is ready.** One empty file per day in `data/examples/`, named `01.txt`, `02.txt`, and so on. When the page loads, you copy into a file that already exists.
- **The real inputs are gitignored.** A file named `.gitignore` at the project root lists paths that Git should never track. Puzzle inputs are personal, and they do not belong in a public project.
- **The cheat sheet is open.** [The cheat sheet](/appendix/cheat-sheet/) has the imports, the grid calls, and the queue calls in one place. Keep it in a tab.
- **You have slept.** A tired brain writes a parser that drops the last line. The puzzle will still be there after breakfast.

```text title=".gitignore"
/target
/data/inputs
```

The first line keeps build output out of Git; the second keeps the puzzle inputs out. The example files stay in the project, because they are small, yours to type, and useful in tests.

On the east coast of the United States the puzzle appears at midnight. If that is the middle of your night, solve it in the morning. The month is long, and the people who finish it are the ones who sleep.

## If something went wrong

**The answer is wrong on the real input, but the example passes.** The example is small and the input is not. Look for a header line, a blank line at the end, a number with more digits than you expected, or a rule that only shows up when two things overlap. Print the number of parsed rows and the largest value you read.

**You never checked the input shape.** Open the file and look at the first line, the last line, and one line in the middle. If the statement says "each line contains two numbers", confirm that every line does. A parser that assumes a fixed width breaks on the first ragged line.

**A number went backwards.** In a debug build, Rust stops with `attempt to subtract with overflow`. In a release build the same line wraps around to a huge number. Choose `u64` for counts, `i64` for coordinates that can go negative, and `usize` for indices, and say why in a comment.

**You are off by one.** Fence posts again: ten steps between eleven posts. Check whether the puzzle counts the starting cell, the final cell, or both. Move the example's numbers by hand and see which convention matches.

**Part one broke after the part two rewrite.** Keep `part1` and `part2` as separate functions with separate tests. When part two needs the same search with one extra field, write a second function instead of editing the first. Two small functions are cheaper than one clever one.

**The program is too slow.** Run it with `cargo run --release` before changing the algorithm. If the release build is still slow, count the states: a search that revisits the same cell a million times needs a `seen` table, not a faster machine.

## Save this step

Save the routine and the notes before the month starts. The commit is small, and it gives you a clean place to return to if an experiment goes wrong.

```console
$ git status
$ git add src
$ git commit -m "Add the triage routine and the drill 7 notes"
```

`git status` shows which files changed; `git add src` stages the source folder; `git commit` records the staged files with a message. The branch name and any hashes in your output will differ from mine.

<details>
<summary>Going deeper: the one-page review</summary>

Read this list once before the mock, and again on the last evening of November. If an answer does not come, the linked page has it.

- Can you write the day contract from memory: `parse`, `part1`, `part2`, and two tests?
- Can you explain why breadth-first search finds the shortest path when every edge costs one?
- Can you name the five part-two twists without looking?
- Can you count shortest paths using a distance table, and say why sorting by distance is enough?
- Which drill is closest to a grid puzzle, and which is closest to a simulation?
- What belongs in `.gitignore` on the first day?
- When the timer rings, what four things will you write down?

The answers are not the point. The point is that the questions arrive before the puzzle does.

</details>

The ramp ends here. Next, [the eleven years at a glance](/appendix/eleven-years/) shows what the last decade of Decembers actually asked for.
