---
title: "Read the input, draw the grid"
description: "Turn text into lines, numbers, and a flat grid you can walk."
---

Every puzzle in December arrives the same way: a page of story and a file of text. The story changes each morning. The file is always lines of digits, letters, dots, and hashes. Before the program can count anything, those lines have to become values it can hold.

Today we do that in two steps. First we read a header, the line that gives the grid's size, split it into named numbers, and turn the rest into rows. Then we put every row into one flat list of bytes, with a width and a height, so a cell can be found with arithmetic instead of nested lists.

This is the least glamorous part of the month, and the part that decides whether the rest goes smoothly. A grid you can index safely is a grid you can walk, search, and count without surprises.

## What you need to know

A `String` owns a piece of text. A `&str`, called a string slice, borrows text that something else owns. Functions that only read text take `&str`, so they accept a literal and a `String` through a reference:

```rust
let owned: String = String::from("..#");
let borrowed: &str = &owned;
let literal: &str = "..#";
```

The three names read the same three bytes. `owned` owns a buffer holding them; the two `&str` values only look.

`.lines()` walks the text one line at a time and leaves the newline out. `for line in input.lines()` gives `line` as a `&str` for each row. `.trim()` removes spaces, tabs, and carriage returns from both ends, so `"  #  ".trim()` is `"#"`.

`.split_once('=')` cuts a piece of text at the first `=`. It returns an `Option`, a value that is either `Some(pair)` or `None`. `"width=8".split_once('=')` gives `Some(("width", "8"))`; text without an `=` gives `None`.

`.parse::<u64>()` reads text as a number. It returns a `Result`, which is either `Ok(number)` or `Err(problem)`. `"42".parse::<u64>()` gives `Ok(42)`. The `u64` inside the angle brackets says what kind of number to produce. `unwrap()` takes the number out of an `Ok` and stops the program with a message when it finds an `Err`.

A `Vec<u8>` is a growable list of bytes: `let cells: Vec<u8> = vec![b'.', b'#'];`. A byte is an 8-bit number. ASCII is the simple text encoding used by these puzzles, where each English letter, digit, or punctuation mark is one byte: `b'.'` is 46 and `b'#'` is 35. Storing a grid as bytes uses one byte per cell, which keeps a large map small.

`usize` is an unsigned integer used for lengths and positions: `let width: usize = 8;`. It cannot be negative, and on a 64-bit machine it is 64 bits wide.

`i32` is a signed 32-bit integer, which can be negative: `let step: i32 = -1;`. We use it for small offsets such as one row up.

## The shape of a day

Every drill in the lab answers the same three questions in the same order. How do I read this input? What is the first answer? What is the second? The names never change:

```rust title="src/bin/drill01.rs"
pub fn parse(input: &str) -> Grid
pub fn part1(grid: &Grid) -> u64
pub fn part2(grid: &Grid) -> u64
```

`pub` means other files and the tests may call the function. `parse` takes the whole input as a borrowed `&str` and returns a `Grid` that it owns. Each part borrows that grid with `&Grid`, so neither part copies it or takes it away. Parse once, answer twice.

The example input is baked into the test program at compile time. The line `const EXAMPLE: &str = include_str!("../../data/examples/01.txt");` tells Rust to read that file while building and store its text inside the compiled program. The path is relative to the source file that names it. The real input is not baked in; the program reads it when it runs, usually with `std::fs::read_to_string` returning a `String`, then passes `&text` to `parse`.

The tests are the oracle. A test calls `parse` on the example and `assert_eq!` on each part, comparing the result with the answer the example gives. `assert_eq!` checks that two values are equal and stops the test with both values printed when they are not. This is why the page never tells you the answer: the test already knows it, and passing tests are the proof.

`main` is the small program that prints both answers for the example:

```rust title="src/bin/drill01.rs"
fn main() {
    let grid = parse(EXAMPLE);
    println!("part1 = {}", part1(&grid));
    println!("part2 = {}", part2(&grid));
}
```

`parse(EXAMPLE)` runs once, and both calls read the same `grid`. Change `EXAMPLE` to your own text, or read a file into a `String`, and the same two lines answer the real puzzle.

## One flat grid

A grid looks two-dimensional, but the computer's memory is one long row of bytes. The lab stores a grid in a single `Vec<u8>`, row after row, and keeps two numbers beside it:

```rust title="src/grid.rs"
pub struct Grid {
    pub cells: Vec<u8>,
    pub width: usize,
    pub height: usize,
}
```

`cells` holds every byte in reading order: the first `width` bytes are row 0, the next `width` bytes are row 1, and so on. `width` and `height` say how to cut that list into rows. A struct is a bundle of named fields, and `pub` on a field means code outside this file may read it.

The position of row `r` and column `c` is `r * width + c`. On an 8-wide grid, row 1 column 3 is `1 * 8 + 3`, which is flat index 11. An index is a position in a list, and Rust counts from zero. That one multiplication replaces a list of lists and keeps every cell next to its neighbours in memory, which the processor likes.

The method that turns a row and column into that index has a name:

```rust title="src/grid.rs"
pub fn idx(&self, row: usize, col: usize) -> usize {
    row * self.width + col
}
```

The opposite direction uses division and remainder: `row_col(11)` on the same grid gives `(1, 3)`, because `11 / 8` is 1 and `11 % 8` is 3. `%` is the remainder operator. With these two methods, a flat index and a row-and-column pair are two views of the same cell.

Here is the middle of `Grid::parse`, where lines become cells:

```rust title="src/grid.rs"
let height = rows.len();
let width = rows.first().map_or(0, |row| row.len());
let mut cells = Vec::with_capacity(width * height);

for row in &rows {
    assert_eq!(row.len(), width, "every grid row must have the same length");
    cells.extend_from_slice(row.as_bytes());
}
```

`rows` is the list of non-empty lines collected above. `rows.first()` returns an `Option`, and `map_or(0, ...)` supplies zero when there are no rows. `Vec::with_capacity` reserves room for every cell up front, so the list does not have to grow while it fills. `assert_eq!` stops the program with a clear message if a row is a different length, because a ragged grid would make `r * width + c` point at the wrong cell. `as_bytes()` views a `&str` as bytes without copying, and `extend_from_slice` appends them all.

Three small methods sit on top of the index; a method is a function attached to a struct. `at(row, col)` returns the byte and trusts that the position is inside. `set(row, col, value)` writes a byte in place. `get(row, col)` is the careful one: it returns `Some(byte)` inside the grid and `None` outside, which is what the neighbour code uses.

## Neighbours and edges

A cell has up to four orthogonal neighbours: above, below, left, and right. If diagonals count too, it has up to eight. The eight are the four plus the four corners, so they can be written as every offset from -1 to 1 in each direction, skipping the cell itself:

```rust title="src/grid.rs"
for dr in -1..=1 {
    for dc in -1..=1 {
        if dr == 0 && dc == 0 {
            continue;
        }
        if self.get(row as i32 + dr, col as i32 + dc).is_some() {
            ...
        }
    }
}
```

`-1..=1` is an inclusive range: -1, 0, and 1. `dr` and `dc` are `i32` values, the signed type from earlier; the call to `get` expects `i32`, so the range is read that way. `continue` skips the centre, where both offsets are zero. `row as i32` converts the position before adding, so the sum can be -1 even though `row` is a `usize`.

That conversion is the whole trick. At the top-left corner, `row - 1` on a `usize` would try to go below zero and panic, stopping the program, with `attempt to subtract with overflow` in a debug build. Adding `-1` to an `i32` produces an honest negative number, and `get` checks it:

```rust title="src/grid.rs"
pub fn get(&self, row: i32, col: i32) -> Option<u8> {
    if row < 0 || col < 0 {
        return None;
    }
    let row = row as usize;
    let col = col as usize;
    if row >= self.height || col >= self.width {
        return None;
    }
    Some(self.at(row, col))
}
```

The two checks happen before the casts back to `usize`, so a negative position never reaches `at`. The casts are safe after the checks because the values are known to be at least zero. `Option<u8>` carries the answer: `Some(byte)` for a real cell, `None` for the space outside the map.

Because of these checks, an edge cell has fewer neighbours. A corner has two in the four-way case and three in the eight-way case. No special case is needed at the border; asking for a neighbour that does not exist returns `None`, and the loop moves on.

## Count, sum, sort

Two chores come up again and again: how many of each thing, and which one appears most. A `HashMap` maps a key to a value. When the key is a byte, the value can be a count:

```rust
use std::collections::HashMap;

let mut counts: HashMap<u8, u64> = HashMap::new();
for &cell in &grid.cells {
    let tally = counts.entry(cell).or_insert(0);
    *tally += 1;
}
```

`entry(cell)` looks up the key and returns a slot. `or_insert(0)` fills an empty slot with zero and returns a mutable reference to the value. `*tally += 1` writes through that reference. The loop counts every cell without checking whether the key is already present.

A `HashMap` has no order, so print or sort the pairs when order matters. `BTreeMap` is the same idea with sorted keys, at a small cost per operation. To rank counts, move the pairs into a `Vec` and sort:

```rust
let mut pairs: Vec<(u8, u64)> = counts.into_iter().collect();
pairs.sort_by_key(|pair| pair.1);
pairs.reverse();
```

`into_iter()` produces `(key, value)` pairs, and `collect()` gathers them into the vector. `sort_by_key` takes a closure, a small function written in place, and sorts by the value it returns. Here `pair.1` is the count. `reverse()` turns ascending into descending. Sorting costs roughly `n log n` comparisons, where `n` is the number of distinct keys.

## The drill

Open `data/examples/01.txt`. It has a header and a small map:

```text
width=8 height=6
........
.##..3..
.#.#....
..##..7.
....#...
........
```

The header says the body is 8 columns wide and 6 rows tall. The body holds dots, hashes, and two digits. `parse` splits the header on whitespace, the spaces and tabs between words, removes the `width=` and `height=` prefixes, and parses the two numbers. It joins the remaining lines and hands them to `Grid::parse`, then checks that the header agrees with the body.

Part 1 adds the value of every digit cell. A digit byte is not the number it shows: `b'7'` is 55. The lab subtracts `b'0'`, which is 48, to turn `b'7'` into 7:

```rust title="src/bin/drill01.rs"
pub fn part1(grid: &Grid) -> u64 {
    grid.cells
        .iter()
        .filter(|byte| byte.is_ascii_digit())
        .map(|byte| (byte - b'0') as u64)
        .sum()
}
```

Read the chain as four steps. `iter()` visits every byte, `filter` keeps the ones where `is_ascii_digit()` is true, `map` converts each kept byte to a number, and `sum` adds them. The `as u64` widens the byte-sized result to the answer type.

Part 2 counts `#` cells with exactly three `#` neighbours, diagonals included. It walks every row and column, skips anything that is not a hash, then runs the eight offsets from the previous section and counts how many neighbours are also `#`. When the count is three, it adds one to the total.

Run the tests:

```console
$ cargo test --bin drill01
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/bin/drill01.rs (target/debug/deps/drill01-...)

running 2 tests
test tests::drill01_part1_example ... ok
test tests::drill01_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Two lines say `ok`, so the parser and both parts agree with the example. The timing and the path under `target` will differ on your machine, and the two test lines may appear in the other order because tests run side by side. The whole drill is in `src/bin/drill01.rs`; read `parse`, `part1`, and `part2` once more with the map in front of you.

## If something went wrong

**`called Result::unwrap() on an Err value: ParseIntError { kind: InvalidDigit }`** means a header field was not a number. `"8x".parse::<usize>()` sees the `x` and refuses. Print the header with `println!("{header:?}")` and look for a stray character or an empty value. `kind: Empty` means the field had no digits at all.

**`assertion left == right failed: every grid row must have the same length`** comes from `Grid::parse`. `left` is the length of the row it has read; `right` is the width taken from the first row. One row is longer or shorter, often because of a trailing space. Trim the rows or fix the input file.

**`attempt to subtract with overflow`** means a `usize` went below zero. It usually comes from `row - 1` or `col - 1` at the top or left edge. Convert to `i32` before subtracting and let `get` check the result. A release build turns the check off and wraps around instead, which is worse: the program keeps running with a huge index.

**`index out of bounds: the len is 48 but the index is 48`** means an index equal to the length was used. A list of 48 cells has positions 0 through 47. This often follows `row * width + col` when `col` equals `width`; check the position before indexing, or use `get`.

## Save this step

If you typed the drill into your own copy, save the working state with three commands:

```console
$ git status
$ git add -A
$ git commit -m "Read the input and draw the grid"
```

`git status` lists what changed in the project, `git add -A` stages every change for the next commit, and `git commit` records them with a message. Git prints a summary after each command; the branch name and the short hash, the commit's name, are yours and will not match anyone else's. If `status` says `nothing to commit, working tree clean`, you have no changes to save, so skip the commit.

<details>
<summary>Going deeper: bytes, chars, and where the input lives</summary>

`bytes()` gives one `u8` per byte, while `chars()` gives one Unicode character at a time. A character such as `é` or an emoji takes more than one byte, so the two counts differ. Grid puzzles are ASCII, where one byte is one character, so bytes are smaller and simpler. Text from a story needs `chars()` when you count letters as a person would.

`split_ascii_whitespace()` splits on spaces, tabs, and line breaks, and treats only ASCII whitespace as a separator. `split_whitespace()` also knows about Unicode spaces. For a header made of ASCII words, either works; the ASCII version is predictable when the input is raw bytes.

`include_str!` reads a file while compiling and stores the text in the binary, so the program needs no file at run time. `std::fs::read_to_string` reads at run time and returns a `Result`, because a missing file is a real possibility. Tests use the first; a solver uses the second so the same binary can read today's input.

A `HashMap` hashes each key on every lookup. Hashing a single byte is cheap, but a plain `Vec<u64>` with 256 slots, one per byte value, is cheaper and needs no hashing at all. Reach for the map when keys are sparse or are not small numbers.

</details>

Next: [walk the grid and search the graph](/part-2/walk-the-grid/).
