---
title: "Structs, enums, and errors"
description: "Group values with a struct, list the choices with an enum, and handle a value that may be missing or may fail."
---

A single value only goes so far. A point on a grid has two numbers. A step has one direction out of four. A reading can be absent, and a line of input can be nonsense. Rust gives us four tools for these shapes: the `struct`, the `enum`, and two types that say a value may not be there.

We will build a `Point` that knows where it is, a function that returns a word or nothing, and a function that returns a number or an error. Each piece is small. Together they cover most of what the rest of the book asks you to read.

## A struct groups values under one name

A `struct` is a type that groups named values. Each value is a **field**: a name and a type, written on its own line. The `struct` line names the new type, and the braces hold the fields.

```rust title="src/main.rs"
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
```

`Point` is now a type with two fields, both `i32`, a whole number that fits in 32 bits. The line above it, `#[derive(...)]`, asks Rust to write three useful things for us. `Debug` lets `{:?}` print the struct while we work. `Clone` adds a `.clone()` method that makes a full copy. `Copy` makes a plain assignment copy the value instead of moving it, which the next page explains.

To create a value of this type, write the type name and give every field a value. The order does not matter, but none may be left out.

```rust
let mut here = Point { x: 0, y: 0 };
here.x += 3;
println!("{here:?}");
```

The variable is `mut` because we change `here.x`. A dot reaches a field, so `here.x` reads it and `here.x += 3` changes it. `{here:?}` prints the whole value in the debug form.

## Methods live in an impl block

An `impl` block is where methods live. A **method** is a function that belongs to a type, so it is called with a dot and can read the value it was called on. The first parameter says how much access the method needs.

```rust
impl Point {
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
```

`&self` means the method borrows the point to read it. `&mut self` means it borrows the point to change it. Inside the body, `self` is the value the dot was called on, so `here.manhattan()` runs with `self` set to `here`. The return type follows the arrow, and the last expression is the return value.

`abs()` turns a negative number positive. A point at `(3, -4)` is seven steps from `(0, 0)`: three across and four down.

## An enum lists the choices

An `enum` is a type with a fixed set of **variants**. Each variant is one possible value, written with the enum's name and `::`. If a direction can only be north, south, east, or west, an enum says so in one place.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn step(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    }
}
```

`match` takes a value and runs the arm whose pattern fits. Every variant needs an arm; Rust refuses to build the program if one is missing. That check is the point of an enum. If we add a fifth direction later, the compiler shows every `match` that needs a new line.

`step` returns a tuple, a pair of values in parentheses. Writing `let (dx, dy) = step(Direction::East);` unpacks the pair into two variables.

## When the value may be missing

Some questions have no answer. A name may not be in a map, and a line may have no first word. Rust does not use a special number for "missing". It gives us `Option<T>`, a type with exactly two shapes: `Some(value)` when there is a value, and `None` when there is not.

A `HashMap` is a collection that stores values under keys, like a dictionary. Its `get` method looks up a key and returns an `Option`, because the key may not be there.

```rust
use std::collections::HashMap;

let mut ages: HashMap<String, u32> = HashMap::new();
ages.insert(String::from("Ada"), 36);

match ages.get("Ada") {
    Some(age) => println!("Ada is {age}"),
    None => println!("no entry for Ada"),
}
```

`get` returns the age wrapped in `Some`, and the first arm binds it to `age`. A missing key gives `None`, and the second arm handles that. The compiler will not let us treat the `Option` as a plain number, so the missing case is decided before the program runs. `u32` is a whole number that cannot be negative, and `String::from` builds an owned piece of text from a literal.

Our own function can return an `Option` too. This one takes a line of text and gives back its first word:

```rust
fn first_word(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed.find(' ') {
        Some(index) => Some(&trimmed[..index]),
        None => Some(trimmed),
    }
}
```

The return type, `Option<&str>`, says: a borrowed piece of text, or nothing. `&str` is text the function borrows rather than owns, and the next page makes that difference concrete. `trim` removes spaces from both ends. `find(' ')` gives the position of the first space as an `Option`, because a line with one word has no space. `&trimmed[..index]` is the text from the start up to that position.

## When a step can fail

Missing is not the only kind of trouble. A function can be handed bad input, and the reason for the failure matters. `Result<T, E>` has two shapes: `Ok(value)` for success and `Err(error)` for failure. The two type names inside say what a success carries and what a failure carries.

```rust
fn damage(text: &str) -> Result<u64, String> {
    let value: u64 = match text.trim().parse() {
        Ok(number) => number,
        Err(_) => return Err(format!("not a number: {text}")),
    };
    if value > 100 {
        return Err(format!("too large: {value}"));
    }
    Ok(value)
}
```

`parse()` turns text into a number and returns a `Result`, because the text may not be a number. The `_` pattern in `Err(_)` means "any error"; we only need to know that one happened. `format!` builds a `String` from a template, the same way `println!` builds a line. `return` leaves the function early with `Err(...)`. A `String` owns text that can grow, which the next page covers properly.

To get the number out, we have to say what to do with the error. `unwrap()` returns the value inside `Ok` or `Some` and panics on `Err` or `None`. `expect("message")` does the same, but puts our message in the panic. A **panic** is Rust's way of stopping the program when something is wrong.

```rust
let speed = damage("42").unwrap();
let fuel = damage("lots").expect("fuel should be a number");
```

The first line gives `42`. The second stops the program with our message, because `"lots"` is not a number. `unwrap` is fine while the input is known and we are still learning. `expect` is better when a future reader needs to know what we assumed.

The `?` operator makes the early return shorter:

```rust
fn total(first: &str, second: &str) -> Result<u64, String> {
    let a = damage(first)?;
    let b = damage(second)?;
    Ok(a + b)
}
```

Each `?` means: if this value is `Err`, return it from the whole function now; if it is `Ok`, hand me the value inside. It only works inside a function that returns a `Result` or an `Option`, because it needs a failure to hand back. `total("40", "2")` gives `Ok(42)`, and `total("40", "lots")` gives the same error as `damage("lots")`.

## Put the pieces together

Here is one program that uses the point, the optional word, and the result. Add the struct, its `impl` block, and the two functions above `main`, then run it.

```rust title="src/main.rs"
fn main() {
    let mut here = Point { x: 0, y: 0 };
    here.move_by(3, -4);
    println!("{here:?} is {} steps away", here.manhattan());

    println!("{:?}", first_word("north east"));
    println!("{:?}", first_word("   "));

    let speed = damage("42").expect("42 is a number");
    println!("speed: {speed}");
}
```

The four lines appear in order: the moved point and its distance, the first word, the empty answer, and the parsed number.

```text
Point { x: 3, y: -4 } is 7 steps away
Some("north")
None
speed: 42
```

`Some("north")` and `None` are printed by `{:?}` because we asked for the debug form of an `Option`. A plain `{}` would not compile for this type, and the error would tell you to use `{:?}` instead.

## If something went wrong

**A missing field** stops the build before anything runs. The message names the field and points at the value you wrote:

```console
$ cargo build
...
error[E0063]: missing field `y` in initializer of `Point`
 --> src/main.rs:7:13
  |
7 |     let p = Point { x: 1 };
  |             ^^^^^ missing `y`
...
```

Add the missing field and build again. The carets point at the struct name, and the word after them is the field Rust wants.

**An unknown variant** means the name after `::` is not in the enum. It may be a spelling mistake, or a variant you expected but never wrote:

```console
$ cargo build
...
error[E0599]: no variant, associated function, or constant named `Up` found for enum `Direction` in the current scope
 --> src/main.rs:8:24
  |
1 | enum Direction {
  | -------------- variant, associated function, or constant `Up` not found for this enum
...
8 |     let d = Direction::Up;
  |                        ^^ variant, associated function, or constant not found in `Direction`
...
```

Compare the name with the enum's variants. Rust lists the right spelling in the message only when it is close to something real, so read the enum itself.

**`unwrap` on `None`** compiles and then stops at run time. The message names the call and the value:

```console
$ cargo run
...
thread 'main' (...) panicked at src/main.rs:3:22:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Nothing after that line runs. Replace `unwrap` with a `match`, or with `expect` that says what you assumed, so the missing case has a plan. The number in parentheses is the thread id and changes on every run.

The point knows its distance, the enum covers every direction, and the two result types carry either a value or a reason. Next we look at the `&` and `&mut` marks in those signatures.

Next: [borrowing without fear](/ground-floor/borrowing-without-fear/).
