---
title: "Lists, loops, and functions"
description: "Keep many values, repeat work, and write a function."
---

A puzzle rarely hands you one number. It hands you rows, lines, or a long run of readings. A program needs a place to keep all of them.

Rust's growable list holds values in order and lets you add more. Loops visit the values one at a time. A function gives a name to a piece of work so the program can use it again.

We will build a list, walk it, and write a function that adds it up.

## A list you can grow

A **list** holds several values of one type, in order. Rust calls its list a **vector**, written `Vec`, and a vector can grow or shrink while the program runs. The type `Vec<i32>` is a list of `i32` values; the part in angle brackets says what the list holds.

```rust
fn main() {
    let mut readings: Vec<i32> = Vec::new();
    readings.push(3);
    readings.push(8);
    readings.push(2);

    println!("len = {}", readings.len());
    println!("first = {}", readings[0]);
    println!("{:?}", readings);
}
```

`Vec::new()` creates an empty list, and the annotation says what it will hold. The list needs `mut` because `push` changes it. Each `.push(value)` adds one value to the end. `.len()` gives the **length**, the number of values, and its type is `usize`.

`readings[0]` reads one value by its **index**, the position number. Indexes start at zero, so the first value is at index `0`, the second at `1`, and the last at `length - 1`. Asking for a position that does not exist is an **out-of-bounds** index, and Rust stops the program with a **panic**: an immediate stop with a message. The block above prints:

```text
len = 3
first = 3
[3, 8, 2]
```

`{:?}` prints the whole list in brackets because lists do not have a usual display form.

When the values are known up front, the macro `vec!` builds the list in one line:

```rust
fn main() {
    let readings: Vec<i32> = vec![3, 8, 2];
    println!("{}", readings.len());
}
```

This is the same list without the three `push` calls, and it prints `3`.

## Visit every item with for

A **loop** repeats a block of code. One trip through the block is an **iteration**. The `for` loop visits each value in a list, one iteration per value:

```rust
fn main() {
    let readings: Vec<i32> = vec![3, 8, 2];

    for value in &readings {
        println!("{}", value);
    }
}
```

Read the first line as "for each value in readings". The `&` means the loop **borrows** the list: it looks at the values without taking them away, so `readings` is still usable after the loop. The braces hold the body, the lines that run on each iteration. This prints `3`, then `8`, then `2`.

Sometimes the position matters. A **range** written `0..n` counts from `0` up to, but not including, `n`:

```rust
fn main() {
    let readings: Vec<i32> = vec![3, 8, 2];

    for i in 0..readings.len() {
        println!("{}: {}", i, readings[i]);
    }
}
```

With a length of three, `i` takes the values `0`, `1`, and `2`, and the last line printed is `2: 2`. `i` is a `usize`, the same type as the length, because it is used as an index.

## Repeat with while and loop

A `while` loop checks a condition before every iteration and stops when the condition becomes false:

```rust
fn main() {
    let mut count: i32 = 3;

    while count > 0 {
        println!("{}", count);
        count = count - 1;
    }
    println!("Liftoff!");
}
```

This prints `3`, `2`, `1`, then `Liftoff!`. The counter needs `mut` because the body changes it. If the body never changes `count`, the condition stays true and the loop never ends; Ctrl-C stops it.

A `loop` repeats with no condition at all. Use it when the stopping point sits in the middle of the body. The keyword `break` leaves the loop immediately:

```rust
fn main() {
    let mut total: i32 = 0;

    loop {
        total = total + 1;
        if total == 3 {
            break;
        }
    }
    println!("{}", total);
}
```

The loop adds one each time and stops when `total` reaches `3`, so the program prints `3`. Without `break`, it would run forever.

## Write a function

A **function** is a named block of code that does one job. It can take inputs, called **parameters**, and give back an output, called a **return value**. We have used one function all along: `main`.

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(21));
}
```

`fn` starts the definition, `double` is the name, and `n: i32` is one parameter named `n` with type `i32`. The arrow `-> i32` says the function returns an `i32`. Calling the function means writing its name with a value, as in `double(21)`; the value you pass is the **argument**. This program prints `42`.

The body is one line, `n * 2`, with no semicolon. That detail matters.

An **expression** produces a value. A **statement** is an instruction ending in a semicolon; it does something and throws away any value. The last expression in a function body is the function's return value, so it stays without a semicolon.

If you add a semicolon after `n * 2`, the function returns nothing while the arrow promised an `i32`, and the compiler rejects it. The keyword `return` also works: `return n * 2;` leaves the function at once. Use it for an early exit from inside an `if`; otherwise the last expression is the usual style.

## Take a borrowed list

A function can take a list. The type `&[i32]` means a **slice**: a view into a run of `i32` values. Read it as "a borrowed list of `i32`". The `&` means the function borrows the list, so the caller keeps it and can use it again after the call.

```rust
fn sum(values: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for i in 0..values.len() {
        total += values[i];
    }

    total
}
```

`values.len()` works on a slice. The line `total += values[i]` is shorthand for `total = total + values[i]`. The last line, `total`, has no semicolon, so its value becomes the return value. A slice is the usual way to hand a function a list it only needs to read; the page on [borrowing without fear](/ground-floor/borrowing-without-fear/) explains the rules behind the `&`.

## Put it together

This program builds a list, passes it to `sum`, and prints the result:

```rust
fn sum(values: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for i in 0..values.len() {
        total += values[i];
    }

    total
}

fn main() {
    let readings: Vec<i32> = vec![3, 8, 2];
    println!("{}", sum(&readings));
}
```

`main` creates the vector and calls `sum(&readings)`. The `&` lends the list to the function; after the call, `main` still owns it. `sum` walks the indexes, adds each value, and returns the total. The program prints `13`.

Try the list `vec![10, 20, 12]` and check that the output becomes `42`.

## If something went wrong

**Mismatched types**

```text
error[E0308]: mismatched types
```

A value's type does not match what the surrounding code expects. A common case is assigning a whole `Vec<i32>` where one `i32` belongs; pass the list to `sum`, or index one value out of it.

**An index with no value**

```text
thread 'main' (1291234) panicked at src/main.rs:3:28:
index out of bounds: the len is 3 but the index is 3
```

The list has three values, at indexes `0`, `1`, and `2`, and the code asked for index `3`. Remember that the last index is always `length - 1`. The number in parentheses on the first line is a thread id and will differ on your machine.

**A function with no return value**

```text
error[E0308]: mismatched types
```

The function promised a return type but its body produced nothing. Check that the last expression has no semicolon, or add an explicit `return`. The full message includes `expected i32, found ()`; `()` is the **unit** type, the type of nothing.

**An unexpected closing brace**: a brace is missing or one too many. Count the opening and closing braces, starting from the function that contains the mistake.

Lists, loops, and functions are enough for many small programs. Next, we group related values into one type and handle failure with [structs, enums, and errors](/ground-floor/structs-enums-and-errors/).
