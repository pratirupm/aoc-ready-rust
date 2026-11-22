---
title: "How this book works"
description: "The loop of reading, typing, running, fixing, and saving; the ground floor; the seven days; and the paths through them."
---

A page in this book is not finished when you have read it. It is finished when you have typed its code, run it, and seen the result on your own screen. That sounds plain until you try it: the compiler notices things a reader glides past.

The rhythm is the same everywhere. You read a short explanation, type the code into a file, run the command the page shows, and look at what comes back. Most of the time it works. When it does not, the message tells you where to look.

December will feel the same. A puzzle appears, you write something, it fails, you read the failure, and you try again. Practising that loop now matters more than memorising syntax.

## The loop

**Read.** Read the section once without typing. Let the shape of the idea settle, and notice which words are new. Every new programming word is defined where it first appears.

**Type.** Type the code by hand rather than copying it. The slowness is useful: your hands meet the punctuation, brackets, and semicolons that Rust cares about.

**Run.** Use the command in the console block. `cargo` is Rust's build tool; `cargo run` builds and starts the program, and `cargo test` builds it and runs the tests.

**Read the error.** Start at the top. The first line names the problem, the arrow points to a file, line, and column, and the caret marks the text Rust is unhappy about. Fix the first error before reading the rest; one mistake often produces a pile of messages that vanish together.

**Fix.** Change the smallest thing you can: a missing semicolon, a name spelled differently, a type that needs a number instead of text. Run again.

**Test.** When the program compiles, run the drill's tests. A passing test says your code agrees with the example; a failing one says which part disagrees. The test is the oracle: it does not care how you feel about your code.

**Commit.** A commit is a saved snapshot in Git, with a short message describing what works. Saving after each working step means a bad experiment can be undone without losing the good work before it.

## The ground floor

The ground floor is a prerequisite day in seven short pages. It starts with opening a terminal and ends with a test and a first commit. In between you meet a first Rust program, values and decisions, lists and loops and functions, structs and enums and errors, and borrowing.

No page assumes the one before it was plain sailing. If you have never programmed, read them in order and type every example. If you have programmed before, skim the explanations but still do the small programs; Rust's rules differ enough to surprise you.

The ground floor is not an entrance exam. You cannot fail it. Move slowly, break the code on purpose, and put it back together.

## The seven days

After the ground floor, each day adds one family of puzzle techniques and one drill. Day 1 is parsing and flat grids. Day 2 is searching: breadth-first search, depth-first search, and flood fill. Day 3 is weighted paths with Dijkstra's algorithm and a priority queue. Day 4 is remembering results so you do not repeat work. Day 5 is simulation and cycle detection. Day 6 is integer widths, modular arithmetic, intervals, and compression. Day 7 is triage, part-two patterns, and a short mock puzzle.

A day opens with a worked example, not a copied puzzle. The example is original and small enough to trace by hand. We build the technique on it, run it, then hand you a drill with its own tests. The drill is where the idea becomes yours.

The page ends with a commit and a short look back. If the day was hard, the field guide waits with a second explanation.

## Two paths through a day

The minimum path is read the page, type the worked example, complete one drill, and commit. It gives you one working piece of code and one saved state for each day.

The full path adds the remaining drills, the stretch exercises, and the mock on Day 7. It is for a day with energy to spare. The book is a floor, not a ceiling; a reader who takes the minimum path for a week is still learning.

If you miss a day, take the minimum path and move on. Finishing the month with momentum matters more than finishing every exercise in order.

## Three helpers on every page

Most pages end with the same three helpers. **If something went wrong** lists the real messages a step can produce and the plain fix. It is a reference, so skip to it when your screen matches. **Save this step** shows the Git commands for the state you have reached.

**Going deeper** is folded away in a `<details>` block. It holds background not needed for the next page: a little theory, another way to write the idea, a note on why Rust chose a rule. You can skip every one and still finish the book.

The helpers are not decoration. They are where you will look when a page is not enough.

## The field guide

The field guide at the back is built for December. It has a tour of eleven years of puzzles, an index of patterns, a twelve-day playbook for the event, a Rust error decoder, a cheat sheet, and a glossary. It is also for the days when the main page is hard.

When a puzzle resists, look up its pattern rather than its answer. The guide names the technique the shape suggests, which is the part that carries to the next puzzle. An unfamiliar compiler message gets a plain explanation.

That is the whole design: a loop you can repeat, a floor you can stand on, and a guide for when the floor tilts.

Ready? [Set up your workbench](/start/setup/).
