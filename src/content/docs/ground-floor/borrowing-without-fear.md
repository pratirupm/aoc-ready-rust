---
title: "Borrowing without fear"
description: "See when a value moves, when it copies, and how a reference lets one part of the program read while another part stays in charge."
---

The first wall most people hit in Rust is a message about a value that was moved. It looks like the language is being fussy about a variable you can still see on the screen. The rule behind the message is small, and once it clicks, most of those errors turn into a sentence you can read.

We will watch an integer copy and a `String` move, then pass values to functions without giving them away. After that we will meet the rule that keeps reads and changes apart, and the one place it shows up in a loop over a grid.

## An integer copies; a String moves

Every value lives in memory. Some values are small enough to copy in a single step: a number, a `bool`, a tuple of numbers. When you assign one of those to a new variable, the bits are copied and both variables stay usable.

```rust
let a: i32 = 7;
let b = a;
println!("{a} and {b}");
```

This prints `7 and 7`. Nothing was taken from `a`. An `i32` implements `Copy`, which is a promise that the value can be copied this way without surprises. The `Copy` in the `#[derive(...)]` line on `Point` made the same promise for that struct.

A `String` is different. It owns a buffer of text that lives elsewhere in memory, and the variable holds a pointer to that buffer along with its length. Copying the pointer would leave two owners for one buffer, and both would try to free it. Rust avoids that by **moving** the value: the new variable becomes the owner, and the old name stops working.

```rust
fn main() {
    let name = String::from("north");
    let other = name;
    println!("{name} and {other}");
}
```

The assignment moves the string into `other`. The next line tries to read `name`, and the compiler stops it:

```console
$ cargo build
...
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:4:16
  |
2 |     let name = String::from("north");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let other = name;
  |                 ---- value moved here
4 |     println!("{name} and {other}");
  |                ^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let other = name.clone();
  |                     ++++++++
...
```

Read the message from the bottom up. `name` was moved at line 3, and line 4 borrows it after the move. "Borrow" here means "use the value"; the next section shows the mark that asks for a borrow on purpose.

## A reference lets you look without taking

A reference is a value that points at another value without owning it. You make one with `&`. The owner keeps the value, and the reference is only allowed to look.

```rust
fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let line = String::from("ready");
    let count = length(&line);
    println!("{count}");
}
```

`length` takes `&str`, a borrowed piece of text, and `&line` lends the `String` for the call. `line` is still the owner and still usable afterwards. This prints `5`, the number of bytes in `ready`.

To change a value through a reference, the reference needs to be mutable too: `&mut`. The variable that owns the value must also be declared with `mut`, and the function's parameter says `&mut String`.

```rust
fn add_excitement(line: &mut String) {
    line.push('!');
}

fn main() {
    let mut line = String::from("ready");
    add_excitement(&mut line);
    println!("{line}");
}
```

`push('!')` adds one character to the end. The program prints `ready!`. If you forget the `mut` on `let`, Rust says so before the program runs:

```console
$ cargo build
...
error[E0596]: cannot borrow `line` as mutable, as it is not declared as mutable
 --> src/main.rs:7:20
  |
7 |     add_excitement(&mut line);
  |                    ^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let mut line = String::from("ready");
  |         +++
...
```

The help line shows the exact change: add `mut` after `let`.

## One writer or many readers

Here is the rule behind all of this, in plain words: at any moment, a value may have many readers or one writer, but not both. A reader is a `&` reference. A writer is a `&mut` reference, or the owner changing the value directly.

The reason is what a reader expects. If a function holds `&line` and reads its length, the length must not change underneath it. If a writer could grow the string while a reader looked, the reader might follow an old pointer into memory that has moved. Rust makes that impossible by refusing to hand out the writer until the readers are done.

The same rule protects a whole list. A reader of `grid` expects the rows to stay where they are. A writer that pushes a new row could reallocate the list and leave the reader pointing at the old place.

## A list, a reference, and a change

Indexing a list of numbers gives you a value, not a reference. For a `Vec<u8>`, `grid[1][2]` copies the `u8` out, and the copy has no connection to the list.

```rust
let grid: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
let cell = grid[1][2];
println!("{cell}");
```

This prints `6`. The number was copied, so the list is free to change afterwards.

A loop written as `for row in &grid` does something different: it borrows the whole list for the length of the loop. Changing a cell inside that loop needs a mutable borrow of `grid`, and the two cannot overlap. Here is the classic version, with the change inside the read-only loop:

```rust
fn main() {
    let mut grid: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    for row in &grid {
        for cell in row {
            if *cell == 1 {
                grid[0][0] = 9;
            }
        }
    }
    println!("{grid:?}");
}
```

The `*cell` reads the number out of the reference. The build stops with a message that names both sides:

```console
$ cargo build
...
error[E0502]: cannot borrow `grid` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:17
  |
3 |       for row in &grid {
  |       -          -----
  |       |          |
  |       |          immutable borrow occurs here
  |  _____|          immutable borrow later used here
  | |
4 | |         for cell in row {
5 | |             if *cell == 1 {
6 | |                 grid[0][0] = 9;
  | |                 ^^^^ mutable borrow occurs here
... |
9 | |     }
  | |_____- this for loop borrows `grid` immutably, preventing mutation within its body
  |
  = help: consider using an index-based loop instead, or collecting modifications into a separate collection
...
```

The fix is to finish the reading before the writing. Walk the list by index, write down the cells you want to change, then change them in a second loop. No reference to the list survives the first loop, so the second one may borrow it mutably.

```rust
let mut grid: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
let mut targets: Vec<(usize, usize)> = Vec::new();
for row in 0..grid.len() {
    for col in 0..grid[row].len() {
        if grid[row][col] == 1 {
            targets.push((row, col));
        }
    }
}
for (row, col) in targets {
    grid[row][col] = 9;
}
println!("{grid:?}");
```

The first loop reads through indexing and stores positions. `0..grid.len()` is a range of positions from zero up to, but not including, the number of rows, and `grid[row][col]` reaches one cell. The second loop owns those positions and changes the cells. It prints `[[9, 2, 3], [4, 5, 6]]`. Another fix is to copy the value you need out of the row first, then change the list after the loop ends.

## clone buys a copy

When you truly need two independent strings, `clone()` makes a full copy of the text. The original and the copy can then change separately.

```rust
let name = String::from("north");
let copy = name.clone();
println!("{name} and {copy}");
```

Both names print `north`, and both remain usable. The cost is real: a new buffer is allocated and every byte is copied. For a short line that is nothing, but inside a loop over a large grid it adds up. Reach for `clone()` when the two values must live independently, not as a way to silence a message.

## &str and String

`String` owns its text. It can grow, shrink, and move, and it frees the text when it goes away. A `&str` borrows text owned by something else: a `String`, a literal inside the program, or a slice of either.

```rust
let line = String::from("north east");
let first: &str = &line[..5];
println!("{first}");
let flag: &str = "west";
println!("{flag}");
```

`&line[..5]` borrows the first five bytes of the `String` and prints `north`. A literal such as `"west"` is already a `&str`, so it needs no `String`. Functions that only read text take `&str`, which lets them accept both a `String` through `&line` and a literal directly.

## If something went wrong

**`borrow of moved value`** means the owner changed. Use `&` if the function only reads, `clone()` if you need two owners, or move the later use before the move. The arrow in the message shows both places.

**`cannot borrow as mutable because it is also borrowed as immutable`** means a read-only loop is still open while you change the value. Collect the positions or values you need, end the loop, then change. The compiler's help line points at the same fix.

**`cannot borrow as mutable, as it is not declared as mutable`** means the owner lacks `mut`. Add it after `let`, and check that any function you pass it to takes `&mut` as well.

The rules sound strict because they are doing a job: they keep every read honest. Next we write a test that checks the behavior, and save the result in Git.

Next: [tests and your first commit](/ground-floor/tests-and-your-first-commit/).
