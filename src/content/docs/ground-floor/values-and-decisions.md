---
title: "Values and decisions"
description: "Store values, choose a path, and read a type."
---

A program that only prints is a sign. To do work, it needs to remember values and choose between paths.

We give a value a name, tell Rust what kind of value it is, and change it when the work needs that. Then `if` and `match` decide which lines run.

Every number on this page is written into the code. Reading values from a person or a file comes later.

## Give a value a name

A **variable** is a name for a value the program keeps while it runs. The word `let` creates one:

```rust
fn main() {
    let answer: i32 = 41;
    println!("{}", answer);
}
```

`let` introduces the name. `answer` is the name, `: i32` is its type, `=` stores the value on the right, and `41` is the value. The semicolon ends the instruction. `println!` fills the `{}` with the value and prints `41`.

By default the name cannot be changed after it is set. Add `mut`, short for **mutable**, meaning able to change:

```rust
fn main() {
    let mut total: i32 = 0;
    total = total + 1;
    total = total + 1;
    println!("{}", total);
}
```

The program prints `2`. A single `=` is **assignment**: it stores the value on the right into the name on the left. `total = total + 1` reads the old value, adds one, and stores the result back. Two equals signs, `==`, mean something different and appear in a moment.

A second `let` with the same name creates a new variable that hides the first:

```rust
fn main() {
    let answer: i32 = 41;
    let answer: i32 = answer + 1;
    println!("{}", answer);
}
```

This is called **shadowing**, and the program prints `42`. The first `answer` still exists, but the name now points at the second one. Shadowing differs from `mut`: `mut` changes one variable's value, while shadowing makes a new variable and gives it the old name.

## Types say what a value can be

A **type** is the kind of value a name can hold: a whole number, a true-or-false value, or text. Rust needs the type while compiling, before the program runs, so it can reserve the right space and reject operations that make no sense.

An **integer** is a whole number with no decimal point. A **signed** integer may be negative; an **unsigned** integer is zero or larger. A **bit** is a single 0 or 1, and the number in a type's name is how many bits it uses. More bits mean a wider range: the set of values it can hold.

| Type | Holds | Example |
| --- | --- | --- |
| `i32` | a signed whole number from -2,147,483,648 to 2,147,483,647 | `let n: i32 = -7;` |
| `u32` | an unsigned whole number from 0 to 4,294,967,295 | `let n: u32 = 7;` |
| `usize` | an unsigned whole number used for lengths and indexes | `let i: usize = 0;` |
| `i64` | a much wider signed whole number | `let n: i64 = 9_000_000_000;` |
| `u64` | a much wider unsigned whole number | `let n: u64 = 9_000_000_000;` |
| `bool` | `true` or `false` | `let ready: bool = false;` |
| `char` | one character, in single quotes | `let grade: char = 'A';` |
| `&str` | borrowed text that cannot grow | `let name: &str = "Ada";` |
| `String` | owned text that can grow | `let name: String = String::from("Ada");` |

`usize` is the type Rust uses for every length and index. On most computers it is 64 bits, but the language lets it follow the machine, which is why it has its own name.

A **character** is one letter, digit, or symbol. `char` uses single quotes, so `'A'` is a character and `"A"` is text. Text itself is a **string**.

The `&str` type is a borrowed view of text, and a plain string literal like `"Ada"` has that type. A `String` owns its characters: it can grow, shrink, and outlive the line that made it. `String::from("Ada")` builds an owned copy of the text. The next pages show how to pass both around; for now, use `&str` for fixed labels and `String` when text is built as the program runs.

Integers have a limit. When an arithmetic result goes past that limit while the program runs, a debug build stops with `attempt to add with overflow`. A release build, made with `cargo run --release`, wraps around to the other end instead.

Neither result is useful, so use `i64` or `u64` for the large sums and answers in this book. They are 64-bit versions with room to spare.

## Choose a path with if

An `if` runs a block of code only when a **condition** is true. A condition is an expression, a piece of code that produces a value, and that value must be a `bool`. The braces hold the code for each path, and `else` holds the path taken when the condition is false:

```rust
fn main() {
    let cups: i32 = 3;

    if cups > 2 {
        println!("That is plenty.");
    } else {
        println!("Brew another pot.");
    }
}
```

This prints `That is plenty.` Change `3` to `1` and it prints the other line. The comparison operators are `==` for equal, `!=` for not equal, `<` for less than, `<=` for less than or equal, `>` for greater than, and `>=` for greater than or equal.

You can combine conditions. `&&` means "and": both sides must be true. `||` means "or": at least one side must be true. `!` means "not" and flips a `bool`, so `!ready` is true when `ready` is false.

Parentheses group a longer condition when the order matters. An `if` can also have `else if` in the middle, for more than two paths. Rust checks them from top to bottom and runs the first branch whose condition is true.

## Match a value against patterns

A `match` compares one value with several **patterns** and runs the first one that fits. Each line inside is an **arm**, written as a pattern, then `=>`, then the code to run, then a comma:

```rust
fn main() {
    let cups: i32 = 3;

    match cups {
        0 => println!("The pot is empty."),
        1 => println!("One cup left."),
        _ => println!("There is still coffee."),
    }
}
```

Only one arm runs. The underscore `_` is a **wildcard**: it matches any value and works like "everything else". Put it last to catch the values you did not list.

The same form works on a `bool`:

```rust
fn main() {
    let ready: bool = false;

    match ready {
        true => println!("Start the day."),
        false => println!("Coffee comes first."),
    }
}
```

A `bool` has only two values, so the two arms cover every case. Rust checks that a `match` covers all possible values; the compiler complains if an arm is missing. An `if` and `else` could do this job too, but `match` puts every possibility in one place, which helps when the list grows.

## Print a value

`{}` prints a value in its usual form: numbers as digits, text without quotes. `{:?}` prints the **debug** form, which is made for inspecting values while you work. Debug adds quotes around text and brackets around lists.

```rust
fn main() {
    let name: &str = "Ada";
    let cups: i32 = 3;

    println!("{} drank {} cups.", name, cups);
    println!("{:?}", name);
}
```

The first line prints `Ada drank 3 cups.` The second prints `"Ada"`, with the quotes included. Both forms are useful; when a value looks wrong, try `{:?}` first.

## A whole program

This program stores a number and a `bool`, chooses a message with `if`, chooses another with `match`, and prints both values:

```rust
fn main() {
    let cups: i32 = 3;
    let ready: bool = false;

    if cups > 2 && !ready {
        println!("That is plenty of coffee.");
    } else {
        println!("Brew another pot.");
    }

    match cups {
        0 => println!("The pot is empty."),
        1 => println!("One cup left."),
        _ => println!("There is still coffee."),
    }

    match ready {
        true => println!("Start the day."),
        false => println!("Coffee comes first."),
    }

    println!("cups = {}, ready = {:?}", cups, ready);
}
```

It prints four lines. `cups > 2 && !ready` is true only when there are more than two cups and `ready` is false, so the first line is `That is plenty of coffee.` Then the two `match` blocks print one line each, and the last line shows `cups = 3, ready = false`.

Change `cups` to `0` and run it again. Read the new lines and match each one to the arm that produced it.

## If something went wrong

**Mismatched types**

```text
error[E0308]: mismatched types
```

The value on the right does not fit the type on the left. The message names both, such as `expected i32, found &str`. Either remove the quotes or change the type.

**Assigning to a name that cannot change**

```text
error[E0384]: cannot assign twice to immutable variable `cups`
```

The name was created with `let` and then given a new value. Add `mut` after `let`, as the compiler's help line suggests.

**One equals sign in a condition**

```text
error[E0308]: mismatched types
```

The condition used `=` instead of `==`. `=` stores a value, while `==` compares two. Change the condition to `cups == 3`; the full message says `expected bool, found ()`.

**An unused name**: a variable was created and never read. Remove it, or start its name with an underscore if you are keeping it for later.

The program can now remember values and pick a path. Next, we keep many values in one place and repeat work with [lists, loops, and functions](/ground-floor/lists-loops-and-functions/).
