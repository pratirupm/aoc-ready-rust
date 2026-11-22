---
title: "About this book"
description: "How the book was built, what it is not, and where the puzzles live."
---

*Rust Before December* is an independent study guide. It is not affiliated with, endorsed by, or sponsored by Advent of Code. The puzzles, the story, and the calendar belong to Eric Wastl and the Advent of Code project.

This book does not reproduce puzzle text, personal puzzle inputs, or puzzle answers. When a page names a past puzzle, it gives the year and the day and links to the official page. The drills in `rust-aoc-lab` are original: small puzzles written to practise one technique at a time, with their own inputs and tests.

## How the code was checked

Every Rust example that claims to run was run. The lab passes `cargo test`, `cargo fmt`, and `cargo clippy --all-targets -- -D warnings` on Rust 1.99.1. The command output quoted in the chapters comes from those runs; timings, paths, and commit hashes differ on your machine.

The site passes `bun run check`, `bun run lint`, `bun run stats`, `bun run build`, and `bun run links`. The prose linter is strict about vocabulary, code-block length, and tone, which is why some sentences read the way they do. The word-count report flags any part page outside a readable range.

## Where the puzzles live

- [Advent of Code 2026](https://adventofcode.com/2026)
- [The 2025 about page](https://adventofcode.com/2025/about), which explains the shorter 2025 format and the end of the global leaderboard
- [The Rust Book](https://doc.rust-lang.org/book/)
- [The Rust standard library](https://doc.rust-lang.org/std/)

The setup page uses the same tools the rest of the book uses: `rustup`, Cargo, Git, Zed, `rust-analyzer`, `rustfmt`, and `clippy`. The version numbers there are real, and the page says plainly that yours will differ.

## Type and tools

The site and the code blocks use 0xProto by 0xType, released under the SIL Open Font License. The license text is in `public/fonts/OFL.txt`. The site is built with Astro and Starlight. The lab uses only the Rust standard library, so there is nothing to install beyond Rust itself.

## Thank you

Thank you for starting at the beginning. The first program is the hard one; after that, the loop is the same: read, type, run, read the message, change one thing, and try again. The puzzles will be there when you are ready.

[Back to the start](/start/who-this-is-for/).
