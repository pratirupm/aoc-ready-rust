---
title: "Tests and your first commit"
description: "Write a test that states what the code should do, run it, then save the working state in Git."
---

A program that ran once is not the same as a program you can trust. You need a way to ask, at any moment, whether the behavior still holds. A test is that question, written in code, with the expected answer attached.

We will add a test to the project, watch it pass, break it on purpose, and read the failure. Then we save the working state in Git, so the next change starts from somewhere safe.

## A test is a claim the computer checks

A **test** is a small function that states what should happen and then checks it. The check is an **assertion**: a claim that must be true, such as "this sum is 12". If the claim is false, the test fails and names the two values that disagreed.

The book uses tests as the oracle. An oracle is the thing you ask when you are not sure what is true. The test holds the expected value, so you do not have to remember it, and the computer repeats the check every time you ask. When a later change breaks something, the test says which behavior broke.

## Add a test beside the code

Rust keeps tests in the same file as the code they check, inside a module. A **module** is a named box of code; `mod tests` creates one called `tests`, and the full name of a test inside it starts with `tests::`. The `#[cfg(test)]` line tells Rust to build this box only when tests are running.

```rust title="src/main.rs"
fn checksum(values: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for value in values {
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_adds_every_value() {
        assert_eq!(checksum(&[3, 4, 5]), 12);
    }

    #[test]
    fn checksum_of_nothing_is_zero() {
        assert_eq!(checksum(&[]), 0);
    }

    #[test]
    fn checksum_is_positive() {
        assert!(checksum(&[3, 4, 5]) > 0);
    }
}
```

`checksum` takes `&[u32]`, a borrowed list of whole numbers, and adds them. `use super::*` brings the items above the module into view, so the tests can call `checksum`. `#[test]` marks a function as a test; without it, the runner ignores the function even if its name starts with `test`.

`assert_eq!` checks that two values are equal and prints both if they differ. `assert!` checks a condition and reports the condition itself when it is false, so use it when no second value would make the claim clearer. A test function takes no parameters and returns nothing.

## Run the tests

`cargo test` builds a second program, the test runner, and calls every marked function. Passing tests print `ok`. A test that panics is recorded as `FAILED`, and the runner continues with the rest.

```console
$ cargo test
   Compiling checksum v0.1.0 (...)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/main.rs (target/debug/deps/checksum-...)

running 3 tests
test tests::checksum_adds_every_value ... ok
test tests::checksum_of_nothing_is_zero ... ok
test tests::checksum_is_positive ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The summary line is the one to read first: `3 passed; 0 failed` means every test ran and agreed. The folder in the `Running` line and the letters after `checksum-` differ on your machine. The order of the three `test` lines can change too, because tests run side by side.

## When a test disagrees

Change the expected value in the first test from `12` to `13` and run again. The test now makes a claim that is false:

```console
$ cargo test
   Compiling checksum v0.1.0 (...)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/main.rs (target/debug/deps/checksum-...)

running 3 tests
test tests::checksum_is_positive ... ok
test tests::checksum_of_nothing_is_zero ... ok
test tests::checksum_adds_every_value ... FAILED

failures:

---- tests::checksum_adds_every_value stdout ----

thread 'tests::checksum_adds_every_value' (...) panicked at src/main.rs:15:9:
assertion `left == right` failed
  left: 12
 right: 13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::checksum_adds_every_value

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin checksum`
```

`left` is the first value in the comparison, which is what `checksum` returned. `right` is the second value, the one the test expected. Read them in that order: the code produced 12, the test demanded 13. The file, line, and column after `panicked at` point at the assertion; your line will differ if your file is arranged differently.

A test result is not a compiler error. Here the program built and ran; the test disagreed. When the build itself fails, no test runs at all, and you fix the compiler message first. Set the expected value back to `12` before moving on.

## Look inside a value with dbg!

`dbg!` prints an expression and its value, then gives the value back, so the program keeps going. It also prints the file, line, and column of the call, which saves you from guessing where the output came from.

```rust
fn main() {
    let total = dbg!(checksum(&[3, 4, 5]));
    println!("{total}");
}
```

The first line prints `[src/main.rs:2:17] checksum(&[3, 4, 5]) = 12`, and the second prints `12`. `dbg!` needs the value to implement `Debug`, the same mark that lets `{:?}` print a struct. Remove the call when the question is answered; it is a flashlight, not a permanent part of the code.

## Git keeps a history of snapshots

Git is a program that records changes to a folder over time. A **repository** is that folder plus a hidden `.git` folder where the history lives. The **working tree** is the set of files as you see them in your editor, including changes you have not saved in Git yet.

A **commit** is a saved snapshot with a message and a short name called a hash. The **staging area** is a holding area between the working tree and the next commit: you choose which changes go in, then commit that choice.

Cargo may start the history when it creates a project. If it did not, run `git init` in the project folder:

```console
$ git init
Initialized empty Git repository in (...)/checksum/.git/
```

`git init` creates the hidden folder and prints its path; `(...)` stands for the folder where your project lives. Running it again is harmless: it prints `Reinitialized existing Git repository` and keeps the history.

Check what Git sees:

```console
$ git status
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.gitignore
	Cargo.lock
	Cargo.toml
	src/

nothing added to commit but untracked files present (use "git add" to track)
```

The `On branch main` line names the current branch, the default line of history. **Untracked** means Git has noticed the file but is not saving it yet. `Cargo.lock` records the exact versions Cargo used, and `src/` is the folder holding your code.

Before the first commit, tell Git who you are:

```console
$ git config --global user.name "Your Name"
$ git config --global user.email "you@example.com"
```

Nothing prints when either command succeeds. Use the name and address you want in the history; they do not have to match any account. The `--global` part means these settings apply to every repository on this computer.

The `target` folder holds build output and should never be committed. Cargo usually writes a `.gitignore` for you. If the file is missing, create it with this line:

```text title=".gitignore"
target/
```

The trailing slash means "this folder". Git will skip `target` and everything inside it, so `git status` stays readable even after a build.

## Stage the change and commit it

`git add` copies the current state of the files you name into the staging area. A dot means "everything in this folder that is not ignored", which is the usual choice for a first commit.

```console
$ git add .
$ git status
On branch main

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
	new file:   .gitignore
	new file:   Cargo.lock
	new file:   Cargo.toml
	new file:   src/main.rs
```

The files now sit under "Changes to be committed". `git commit` turns that list into a snapshot and attaches your message:

```console
$ git commit -m "Add the first passing test"
[main (root-commit) d3b2983] Add the first passing test
 4 files changed, 25 insertions(+)
 create mode 100644 .gitignore
 create mode 100644 Cargo.lock
 create mode 100644 Cargo.toml
 create mode 100644 src/main.rs
```

The `-m` part is the message. Write it as a short command about what changed, not as a diary entry. `root-commit` means this is the first commit in the repository, and `d3b2983` is the start of its hash. The hash and the counts above come from my copy of the project; yours will differ.

Ask for the history:

```console
$ git log --oneline
d3b2983 Add the first passing test
```

`--oneline` asks for one line per commit, with the short hash first. As you work, the daily loop is the same: change, run `cargo test`, then `git add`, `git commit -m "..."`. A commit is a place you can return to, and a message is the note you leave for your future self.

## If something went wrong

**`error: could not find Cargo.toml`** means the terminal is in the wrong folder. `cd` into the project, the one holding `Cargo.toml`, and run the command again.

**`assertion left == right failed`** means the test ran and disagreed. Compare the two values character by character. Numbers and text differ in type as well as value, so `12` and `"12"` are not the same answer.

**`Author identity unknown`** appears when Git has no name or address for the commit. Run the two `git config` commands above, then commit again.

**`fatal: not a git repository`** means `git init` has not run in this folder, or the terminal is somewhere else. Run `git status` again from the project folder.

The test holds the behavior, and the commit holds the working state. Next we read a real puzzle input and turn its text into a grid.

Next: [read the input, draw the grid](/part-1/read-the-input/).
