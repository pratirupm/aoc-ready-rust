---
title: "Your first Rust program"
description: "Create a Cargo project, open it in Zed, and print a line."
---

Cargo is Rust's build tool: a program that creates projects, compiles them, and runs them. It came with Rust, and we will use it for everything we build.

A **project** is a folder with source code and a settings file. Cargo makes the folder, writes the settings, and adds one short program for us to read.

Then we run that program, change one line, and run it again. That loop of editing, running, and reading is how the rest of the book works.

## Make the project

On the last page you made an empty folder named `rust-aoc-lab`. Cargo wants to create that folder itself, and it will not write into one that already exists. Remove the empty folder first:

```console
$ cd ~/book-work
$ rmdir rust-aoc-lab
```

`rmdir` removes an empty folder. It refuses if anything is inside, which makes it a safe way to undo a practice folder.

Now let Cargo create the project:

```console
$ cargo new rust-aoc-lab --bin
    Creating binary (application) `rust-aoc-lab` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`cargo new` creates the project, and `rust-aoc-lab` is its name. The `--bin` part asks for a **binary**: a program you can run, as opposed to a library, which is code used by other programs. Cargo also starts a Git repository here; Git is a tool that saves versions of a folder over time, and we come back to it later.

Move into the new folder and open it in Zed:

```console
$ cd rust-aoc-lab
$ zed .
```

The dot `.` means "this folder". If `zed` is not found, [Meet Zed](/start/meet-zed/) shows how to install its command. On a Linux Flatpak without the wrapper, run `flatpak run dev.zed.Zed .` instead.

## What Cargo made

The folder now holds two things worth knowing:

```text
rust-aoc-lab/
├── Cargo.toml
└── src/
    └── main.rs
```

`Cargo.toml` holds the settings. The `src` folder holds source code, the text we write. The file `main.rs` is the program itself, and Cargo expects it at that exact place.

Open `Cargo.toml` and you will see:

```toml title="Cargo.toml"
[package]
name = "rust-aoc-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`name` is the project name. `version` is the project's own version number, which starts at `0.1.0` by habit. `edition` names the version of the Rust language the code is written for, and `2024` is the current one. `[dependencies]` lists outside libraries; it stays empty for now.

## The program Cargo wrote

Open `src/main.rs`:

```rust title="src/main.rs"
fn main() {
    println!("Hello, world!");
}
```

`fn` starts a **function**: a named block of code that does one job. The name here is `main`, and it is special. Running the program starts at the first line of `main`. The empty parentheses `()` mean the function takes no inputs.

The braces `{}` mark the **body**, the lines that belong to the function. `println!` prints one line of text. The `!` means it is a **macro**, a command that writes code for you while the program is compiled. For now, read `println!("...")` as "print a line".

The text inside double quotes is a **string literal**: a string is text, and a literal is written exactly as it appears. The semicolon `;` ends the instruction. Most lines inside a function body end with one.

## Run it

```console
$ cargo run
   Compiling rust-aoc-lab v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/rust-aoc-lab`
Hello, world!
```

Cargo did two jobs. **Compiling** means translating the source text into a program the computer can run. **Running** means starting that program. The `(...)` hides the folder path on this machine, and your times will differ.

The last line is the program's output. The program itself now lives at `target/debug/rust-aoc-lab`.

The `target` folder holds build output: large, regenerated, and never edited by hand. `debug` is a **profile**, a set of build settings. The debug profile compiles quickly and keeps extra information for finding mistakes.

If nothing changed since the last run, Cargo skips the compile and prints only the run:

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-aoc-lab`
Hello, world!
```

That is the same program, started from the copy Cargo already built.

## Change the message

In Zed, replace the line between the braces, then save the file:

```rust title="src/main.rs"
fn main() {
    println!("Rust is ready for December.");
}
```

Run it again:

```console
$ cargo run
   Compiling rust-aoc-lab v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/rust-aoc-lab`
Rust is ready for December.
```

Cargo noticed the change and compiled a new copy. Everything between the quotes is text, so you can put any message there. Keep the quotes and the semicolon in place.

## Build without running

```console
$ cargo build
   Compiling rust-aoc-lab v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

There is no program output because Cargo only built the binary. Use `cargo build` when you want to check that the code compiles without starting it. `cargo run` also builds first, so the two commands share the same work under `target`.

## If something went wrong

**A missing semicolon**

```text
error: expected `;`, found `println`
```

A line is missing its semicolon. The compiler points at the end of the previous line and offers to add one.

**A misspelled macro name**

```text
error: cannot find macro `printline` in this scope
```

The name is wrong. Check the spelling of `println`, including the lowercase letters and the `!` at the end.

**Single quotes around text**

```text
error[E0762]: unterminated character literal
```

Text needs double quotes. Rust uses single quotes for one character, which we meet on the next page.

**The destination already exists**

```text
error: destination `/home/you/book-work/rust-aoc-lab` already exists
```

`cargo new` found a folder with that name; your path will show your own folders. If it is the empty folder from the last page, run `rmdir rust-aoc-lab` and try again.

**A missing `Cargo.toml`**: you are in the wrong folder. Run `cd ~/book-work/rust-aoc-lab` and try again.

**`cargo` is not found**: Rust is not on the terminal's path. Close the terminal, open a new one, and run `cargo --version`. If that fails, return to [Set up your workbench](/start/setup/).

The program prints a line, and you know how to change it. Next, we store values in it and teach it to make a decision with [values and decisions](/ground-floor/values-and-decisions/).
