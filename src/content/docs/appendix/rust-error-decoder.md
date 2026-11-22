---
title: "Rust error decoder"
description: "Fourteen real compiler messages, what they mean, and the smallest change that fixes them."
---

Rust's compiler is talkative. A first program can produce a screen of text for one missing word, and the temptation is to read the whole screen as a verdict. It is closer to a set of directions: here is the line, here is the column, here is what the compiler expected, and here is a suggestion.

This page decodes fourteen messages that a beginner meets while building a puzzle project. Each entry shows the real first line, the arrow, the caret, and the important help or note. The file names and line numbers come from small scratch programs, so yours will differ. The words around the code are the part that transfers.

Read the first error first. A pile of errors often comes from one mistake: a missing `let` makes every later use of the name fail, and a bad type makes every call that touches it complain. Fix the first message, run the compiler again, and watch the pile shrink. The rest of this page follows that habit.

## How to read a Rust error

Here is a complete message for a moved `String`:

```console
$ rustc e01.rs
error[E0382]: borrow of moved value: `first`
 --> e01.rs:4:16
  |
3 |     let second = first;
  |                  ----- value moved here
4 |     println!("{first} {second}");
  |                ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
```

Read it in four passes. The first line names the family of mistake and often the exact name: `borrow of moved value: first`. The arrow gives the file, line, and column where the problem is reported, in the form `--> file:line:column`.

The quoted source line and the caret `^^^^^` show the spot. A label beside the caret says what is wrong there. Other lines may carry labels too: in this message, the dashes under `first` mark where the value moved, and the caret marks where it was used too late.

The last part is the help or note. A `help:` is a suggested change. A `note:` explains a rule or points at a library definition. The compiler is not always right about the fix, but it is usually right about the location.

The code in brackets, `E0382`, is a label you can search. Run `rustc --explain E0382` for a longer explanation with examples. The number is stable across projects, which makes it useful in a search engine and in a conversation.

Two families of message appear in this book. A compile error starts with `error[E....]` and a number, and the program never runs. A panic appears after the program starts, as `thread 'main' panicked at ...`, because a value was missing or an index was outside its list. The first family is the compiler refusing to build. The second is the program stopping while it runs.

A recent compiler prints a process id in parentheses after `main` in a panic line, as in `thread 'main' (12345) panicked`. This page writes `(...)` in that spot; the number changes on every run. The note about `RUST_BACKTRACE` that follows a panic is optional and can be ignored while you read the message.

## Moving and borrowing

### E0382: use of moved value

A `String` owns a piece of text on the heap. Assigning it to another name moves the ownership; the first name is no longer valid. Rust refuses to let two names own one allocation, because both would try to free it at the end of the block.

The fix depends on what you meant. If you need two independent copies, call `.clone()`, which copies the text and gives the second name its own allocation. If the second name only needs to read the text, pass a reference with `&`, or borrow it at the use site.

```rust title="before"
let first = String::from("day one");
let second = first;
println!("{first} {second}");
```

The assignment moved the text, and the `println!` then tried to borrow the empty name.

```rust title="after"
let first = String::from("day one");
let second = first.clone();
println!("{first} {second}");
```

Now each name owns a separate `String`. Cloning costs time and memory, so borrow when a read is enough.

### E0502: cannot borrow as mutable because it is also borrowed as immutable

A shared borrow (`&`) promises that the value will not change while the borrow lives. A mutable borrow (`&mut`) asks for the opposite promise: no one else may read or write. Rust will not grant both at once.

The grid version of this error appears when a loop holds a reference to a row and then calls a method that changes the whole grid. The reference is still alive, so the change is refused.

```rust title="before"
let mut row = vec![1, 2, 3];
let first = &row[0];
row.push(4);
println!("{first}");
```

`first` borrows the first cell; `push` needs to change the vector's length, which could move the cells.

```rust title="after"
let mut row = vec![1, 2, 3];
let first = row[0];
row.push(4);
println!("{first}");
```

Copying the number ends the borrow, because `i32` is `Copy`. For a grid, copy the byte or the index, finish the read, then change the grid.

### E0499: cannot borrow as mutable more than once at a time

Two `&mut` references to the same value would let two places change it without coordination, so Rust allows only one at a time. The second borrow is rejected while the first is still used.

```rust title="before"
let mut total = 0;
let a = &mut total;
let b = &mut total;
*a += 1;
*b += 1;
```

The first borrow is still used by `*a`, so the second cannot start.

```rust title="after"
let mut total = 0;
{
    let a = &mut total;
    *a += 1;
}
let b = &mut total;
*b += 1;
```

The braces end the first borrow before the second begins. When the two changes are independent, use one borrow and change the value twice instead.

## Types and names

### E0308: mismatched types

Every value has one type, and an assignment must keep it. `usize` is the type Rust uses for positions and lengths; its width matches the machine. `u32` is a fixed-width unsigned integer. They are different types even when the number is small.

```rust title="before"
let index: usize = 3;
let value: u32 = index;
```

The compiler will not guess that you meant to convert the width.

```rust title="after"
let index: usize = 3;
let value: u32 = index as u32;
```

`as` converts and truncates when the number does not fit, so use it only when you know the value is small. When the value comes from input, `u32::try_from(index).unwrap()` checks the range and stops with an error instead of wrapping silently.

### E0277: the trait bound is not satisfied

A trait is a promise about what a type can do. `HashMap::get` promises to accept a borrowed form of the key type. For a key of `String`, the borrowed form is `str`, so a lookup takes a `&str` such as `"start"`.

```rust title="before"
let value = seen.get(&0u64);
```

The map's key is a `String`, and `String` does not promise to borrow as `u64`.

```rust title="after"
let value = seen.get("start");
```

A string literal is a `&str`, which is the borrowed form the map expects. When a trait bound looks strange, read the `help:` line: it often names the type that is implemented instead.

### E0425: cannot find value in this scope

A name must be introduced before it is used. `let` creates a binding, a name tied to a value. Without it, the assignment tries to change a name that does not exist.

```rust title="before"
fn main() {
    total = 5;
    println!("{total}");
}
```

There is no `total` to assign to.

```rust title="after"
fn main() {
    let total = 5;
    println!("{total}");
}
```

A typo produces the same message with a different name, and the `help:` line often suggests the binding you meant. Check the spelling before you add a new line.

### E0433: failed to resolve

A path with `::` walks through modules and crates. A module is a named box of code, usually one file, and `mod` declares one. If the first name in the path is unknown, the path cannot start.

```rust title="before"
fn main() {
    let grid = grid::Grid::parse("..#\n...");
}
```

The file that holds `Grid` exists, but nothing in this file declares `grid`.

```rust title="after"
mod grid;

fn main() {
    let grid = grid::Grid::parse("..#\n...");
}
```

`mod grid;` tells Rust to read `src/grid.rs` and make its public items reachable under `grid::`. In a project with several crates, the same message can mean a missing `use` or a dependency that is not listed.

### E0061: this function takes two arguments but one was supplied

A function's signature is a promise about how many values it takes and what types they have. The call must keep that promise. The caret shows the first missing argument, and the dashes show where it belongs.

```rust title="before"
fn idx(row: usize, col: usize) -> usize {
    row * 10 + col
}

fn main() {
    println!("{}", idx(2));
}
```

`idx` needs a row and a column.

```rust title="after"
fn idx(row: usize, col: usize) -> usize {
    row * 10 + col
}

fn main() {
    println!("{}", idx(2, 3));
}
```

When the types are right but the count is wrong, the fix is a missing value. When the order is wrong, the compiler may report a type mismatch instead.

### E0599: no method named ...

A method call such as `numbers.sorted()` looks for a method on the type of `numbers`. `Vec` has `sort`, not `sorted`, so the lookup fails. The `help:` line often names the closest match.

```rust title="before"
fn main() {
    let numbers = vec![3, 1, 2];
    numbers.sorted();
}
```

The name is close, but method names are exact.

```rust title="after"
fn main() {
    let mut numbers = vec![3, 1, 2];
    numbers.sort();
}
```

Two changes matter here: the name is `sort`, and `sort` needs a mutable binding because it changes the list in place. Forgetting `mut` produces a different message, `E0596`, which says the value is not mutable. A method that comes from a trait can also fail this lookup when the trait is not imported; the `help:` line then suggests the `use` line.

## Numbers and panics

### attempt to subtract with overflow

An unsigned integer cannot go below zero. In a debug build, Rust checks the subtraction and stops the program. In a release build the same line wraps around to a very large number, which is worse because no message appears.

```rust title="before"
fn main() {
    let mut count: usize = 0;
    count -= 1;
    println!("{count}");
}
```

The debug build checks the subtraction and stops at the line that goes below zero.

```console
$ rustc e10.rs
$ ./e10
thread 'main' (...) panicked at e10.rs:3:5:
attempt to subtract with overflow
```

The panic is a guard, not a crash in the operating-system sense: it stops this program with a message.

```rust title="after"
fn main() {
    let count: usize = 0;
    let next = count.saturating_sub(1);
    println!("{next}");
}
```

`saturating_sub` stops at zero. When negative values are meaningful, use `i64` instead and check the sign; when zero is a bug, test for it before subtracting and print the state.

### index out of bounds

A `Vec` has a length, and an index is valid from `0` to `length - 1`. The bracket lookup checks at runtime and panics on the first invalid index. The message gives both numbers, which is often enough to see the off-by-one.

```rust title="before"
let cells = vec![b'.', b'#', b'.'];
println!("{}", cells[3]);
```

Three cells occupy indices `0`, `1`, and `2`, so `3` is one past the end.

```rust title="after"
let cells = vec![b'.', b'#', b'.'];
let cell = cells.get(3).copied().unwrap_or(b'#');
println!("{}", cell as char);
```

`get` returns `Option<&u8>`: `Some` inside the list, `None` outside it. The `unwrap_or` supplies a wall byte when the index is outside, which is the usual choice for a grid neighbour. A bounds check with `if index < cells.len()` is the other plain fix.

### called `Option::unwrap()` on a `None` value

`Option` is a value that may hold something (`Some`) or nothing (`None`). `unwrap` says "I know this is `Some`"; when it is not, the program panics. A search that finds no match returns `None`.

```rust title="before"
let cells = vec![b'.', b'#'];
let start = cells.iter().position(|&cell| cell == b'S').unwrap();
```

There is no `S` in this list, so `position` returned `None`.

```rust title="after"
let cells = vec![b'.', b'#'];
if let Some(start) = cells.iter().position(|&cell| cell == b'S') {
    println!("{start}");
} else {
    println!("no start cell");
}
```

`if let` handles both cases without a panic. In a puzzle parser, an `expect("a start cell")` is also fair: a missing start means the input is wrong, and a named panic is easier to read than a silent zero.

### called `Result::unwrap()` on an `Err` value

`Result` is a value that may hold a success (`Ok`) or a failure (`Err`). `parse` returns a `Result`, and the `Err` carries a reason. A string with a stray letter is not a number, so the conversion fails.

```rust title="before"
let width: usize = "12x".parse().unwrap();
```

The text is not a number, so the conversion returns an error that `unwrap` turns into a panic.

```console
$ rustc e13.rs
$ ./e13
thread 'main' (...) panicked at e13.rs:2:38:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

`InvalidDigit` names the cause: a character that is not a digit.

```rust title="after"
let width: usize = match "12x".parse() {
    Ok(value) => value,
    Err(error) => {
        println!("bad width: {error}");
        return;
    }
};
```

The `match` turns the failure into a message and leaves `main` early. Trimming the text before parsing fixes the common case of a trailing space or newline, but a real letter should still be reported.

### thread 'main' has overflowed its stack

Every function call uses a stack frame, a small block of memory for arguments and local values. Recursion adds a frame per call. With no base case, or with a base case the input never reaches, the frames run past the stack and the program aborts.

```rust title="before"
fn depth(index: usize) -> usize {
    1 + depth(index + 1)
}

fn main() {
    println!("{}", depth(0));
}
```

The compiler even warns `function cannot return without recursing` before the program runs.

```console
$ rustc e14.rs
$ ./e14
thread 'main' (...) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

A recursive walk needs two limits: a base case and a `seen` table so it never revisits a cell. A grid search that forgets `seen` can recurse forever around a cycle.

```rust title="after"
fn walk(index: usize, seen: &mut Vec<bool>, grid: &Grid) {
    if seen[index] {
        return;
    }
    seen[index] = true;
    for next in grid.neighbors4(index) {
        walk(next, seen, grid);
    }
}
```

The `seen` check ends a path that loops back, and the end of the grid ends the neighbour list. For a very deep grid, an explicit `Vec` stack avoids recursion altogether.

The same four passes work on every message: read the first line, follow the arrow, look at the caret and its label, then read the help. When the message is a panic, the panic line gives the file and line, and the value after the colon gives the cause. A path that differs from this page is not a new species of error; it is the same species in your folder.

Next: [the cheat sheet](/appendix/cheat-sheet/) keeps the working snippets in one place.
