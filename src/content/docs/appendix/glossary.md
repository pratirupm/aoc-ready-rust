---
title: "Glossary"
description: "The programming, Rust, Git, and puzzle words used in the book, in plain language."
---

Every word here is defined where it first appears on a page. This list is for the moment a word comes back and you want the short version. The entries are grouped by first letter, and a family of closely related words shares one entry.

Skip around. Nothing in the book depends on reading this list in order.

## A

**Advent of Code** — A yearly set of programming puzzles, one two-part puzzle per day, starting on December 1. Events from 2015 to 2024 ran to December 25; the 2025 event ran for twelve days. Answers are checked against your own input, and each puzzle has a page on adventofcode.com.

**argument** — A value you pass to a function when you call it. In `length(&line)`, `&line` is the argument.

**assertion** — A claim a test checks, such as "these two values are equal". In Rust, `assert_eq!` and `assert!` make the claim and panic when it is false.

## B

**boolean** — A value that is either `true` or `false`. Conditions in `if` and `while` are booleans.

**borrow** — To use a value through a reference without taking ownership. A borrow ends when the reference is last used.

**branch** — A named line of commits in Git that can move forward on its own. The first branch is usually called `main`.

## C

**cargo** — Rust's build tool and package manager. It creates projects, compiles them, runs tests, and fetches libraries.

**character** — A single piece of text, written in single quotes in Rust: `'a'`, `'7'`, `' '`. The type is `char`.

**clippy** — A tool that reads Rust code and suggests cleaner or safer ways to write it. Run it with `cargo clippy`.

**clone** — An explicit full copy of a value, made with `.clone()`. It costs time and memory because the data is duplicated.

**command** — A word or short phrase you type in a terminal to ask the computer to do something, such as `cargo test` or `git status`.

**commit** — A saved snapshot of a Git repository, with a message and a hash. Also the act of making that snapshot.

**compile** — To turn source code into a program the computer can run. Rust compiles before anything runs, so many mistakes appear at this step.

## D

**derive** — An attribute such as `#[derive(Debug, Clone, Copy)]` that asks Rust to write common code for a type. Each name in the list adds a trait implementation.

**dynamic programming** — A way to solve a problem by solving smaller versions once and reusing their answers. It usually needs a table or a cache to hold those answers.

## E

**enum** — A type whose value is one of a fixed set of variants. `Direction::North` is one value of the `Direction` enum.

**error** — A report that something did not work. Rust uses `Result` for errors a caller can handle and a panic for errors that stop the program.

**example** — The small sample input printed with a puzzle, together with its expected answer. Tests use the example; the real input is much larger.

**expression** — Code that produces a value, such as `3 + 4` or a function call. An expression can be used wherever a value is needed.

## F

**field** — One named value inside a struct. `Point` has the fields `x` and `y`.

**file and folder** — A file is a named container of data on a disk, such as `src/main.rs`. A folder, also called a directory, is a container that holds files and other folders.

**for** — A loop that visits each item of a list or range once. `for value in &values` takes each value in turn.

**function** — A named block of code that can take values and return a value. In Rust a function starts with `fn`; `main` is the one that runs first.

## G

**graph** — A set of things and the connections between them. Puzzles use graphs for maps of rooms, pipes, or one-way paths.

**grid** — A rectangle of cells arranged in rows and columns. A grid is often stored as a list of rows, with each row a list of characters or numbers.

**`.gitignore`** — A file that lists paths Git should not track. Build output such as `target/` belongs there.

## H

**hash map (`HashMap`)** — A collection that stores values under keys and finds them quickly. `ages.get("Ada")` returns an `Option` because the key may be absent.

## I

**if** — A choice: run a block when a condition is true, and optionally another block when it is false.

**impl** — A block that attaches methods or trait implementations to a type. `impl Point { ... }` holds the methods of `Point`.

**index** — A position in a list, counted from zero. `grid[0]` is the first row.

**input** — The text a puzzle gives you to solve. It is different for every person, so answers cannot be shared.

**integer** — A whole number with no decimal part. Rust names integers by size and sign: `i32` and `i64` can be negative, `u32` and `u64` cannot, and the number after the letter is the bit width.

**interval** — A range with a start and an end, such as a time span or a set of covered positions. Puzzles often ask where intervals overlap.

**iteration** — One pass through the body of a loop. A loop that runs five times has five iterations.

## L

**let** — The word that creates a variable. `let mut total: u32 = 0;` creates a changeable `total` with an explicit type.

**list** — An ordered collection of values you can walk through. In Rust the growable list is `Vec`.

**loop** — A block that repeats. Rust has `for` for each item, `while` while a condition holds, and `loop` for repeating until a `break`.

## M

**main** — The function where a Rust program starts. Every binary needs one.

**match** — A check that compares a value with patterns and runs the first arm that fits. `match` on an enum must cover every variant.

**memoisation** — Saving the result of a function for each input so a repeat call can return the saved answer. It turns repeated work into a lookup.

**method** — A function attached to a type and called with a dot, such as `here.manhattan()`. Its first parameter is `self`, `&self`, or `&mut self`.

**module (`mod`, `use`, `pub`)** — A module is a named box of code, declared with `mod`. `use` brings a path into the current file, and `pub` makes an item visible outside its module.

**modulo** — The remainder after division, written `%`. `17 % 5` is `2`, and modulo is how a position wraps around the end of a row.

**move** — To pass ownership of a value from one variable to another. After a move, the old variable can no longer be used.

**mut** — A mark on a variable or reference that allows change. Without `mut`, the value is read-only.

## O

**Option (`Some`, `None`)** — A type for a value that may be missing. `Some(value)` carries a value; `None` says there is none, so the missing case must be handled.

## P

**panic** — Rust's way of stopping the program when something is wrong. A panic prints a message and a location, and a test records it as a failure.

**parameter and argument** — A parameter is a name in a function's definition; an argument is the value passed when the function is called. In `fn length(text: &str)`, `text` is the parameter, and `&line` is the argument.

**part one and part two** — The two questions of an Advent of Code day. Solving part one unlocks part two, which uses the same input with a harder twist.

**path** — The location of a file or folder, such as `src/main.rs` or `/home/you/project`. A path can be relative to the current folder or absolute.

**program** — A set of instructions a computer can run. The source code is the text; the compiled program is what runs.

## Q

**queue** — A collection where the first item in is the first item out. Searches use a queue to visit places in the order they are found.

## R

**remote** — A copy of a Git repository on another computer or service, such as GitHub. `git push` sends commits to a remote.

**repository** — A project folder together with the history Git keeps in its hidden `.git` folder.

**Result (`Ok`, `Err`)** — A type for a step that can fail. `Ok(value)` carries a success and `Err(error)` carries the reason for failure.

**return value** — The value a function hands back to the code that called it. In Rust it is the last expression in the body, unless `return` leaves early.

**run** — To start a program and let it do its work. `cargo run` compiles first when needed, then runs the result.

**rust-analyzer** — The tool an editor uses to understand Rust code. It provides completion, inline errors, and the ability to jump to a definition.

**rustc** — The Rust compiler. `cargo` calls it for you, but `rustc main.rs` can compile a single file directly.

**rustfmt** — The tool that formats Rust code to one agreed shape. Run it with `cargo fmt`.

## S

**search** — The process of visiting places in a graph or grid until you find a target. Breadth-first and depth-first are two orders for the visiting.

**source code** — The text a programmer writes, before it is compiled. Rust source files end in `.rs`.

**stack** — A collection where the last item in is the first item out. A depth-first search uses a stack, and the call stack holds the functions currently running.

**staging area** — The holding area where Git keeps the changes chosen for the next commit. `git add` puts changes there.

**statement** — A line of code that performs an action, usually ending with a semicolon. `let x = 5;` is a statement.

**string (`String`, `&str`)** — Text. A `String` owns a growable buffer; a `&str` borrows a piece of text owned by something else, such as a literal or a `String`.

**struct** — A type that groups named fields under one name. `struct Point { x: i32, y: i32 }` defines one.

## T

**terminal** — A program where you type commands and read text output. It is also called a shell or command line.

**test** — A function that states a behavior and checks it. A passing test means the check held; a failing test means the code and the claim disagree.

**trait** — A named set of behaviors a type can promise to provide. `Debug` and `Copy` are traits, and `#[derive(...)]` writes their code for you.

**type** — The kind of value a variable holds, such as a whole number or text. Rust checks types before the program runs.

## U

**unwrap** — A method that takes the value out of `Ok` or `Some` and panics on `Err` or `None`. It is useful when the value is known to be there.

**usize** — An unsigned whole number the size of a memory address. It is the type of list lengths and indices, so `0..values.len()` counts with it.

## V

**value and variable** — A value is a piece of data such as `42`, `"north"`, or a `Point`. A variable is a name that holds a value, and it is immutable in Rust unless marked `mut`.

**variant** — One member of an enum. `Direction::East` is a variant of `Direction`.

**Vec** — Rust's growable list, written `Vec<T>` where `T` is the type of its items. `Vec::new()` starts empty and `push` adds an item.

## W

**while** — A loop that repeats as long as a condition stays true. The condition is checked before each iteration.

**working tree** — The files in a repository as they are on disk right now, including edits that are not staged or committed.

## Symbols

**`&` and `&mut`** — `&` makes a shared reference, which may read but not change. `&mut` makes an exclusive reference, which may change; a value can have many `&` readers or one `&mut` writer at a time.

**`?`** — An operator after a `Result` or `Option` that returns early on `Err` or `None`. It only works in a function that returns one of those types.
