# Rust Before December

A ground-floor Rust primer and a seven-day practice ramp for Advent of Code.

This is an independent study guide. It is not affiliated with or endorsed by
Advent of Code. Puzzle text and personal puzzle inputs are not reproduced here;
each reference links to the official puzzle page.

## Run the site

```console
$ bun install
$ bun run dev
```

The site runs at `http://localhost:4321`.

## Check the book

```console
$ bun run check
$ bun run snippets
$ bun run lint
$ bun run stats
$ bun run build
$ bun run links
```

## Run the practice lab

```console
$ cd rust-aoc-lab
$ cargo test
```

## Layout

```
src/content/docs/
├── start/          who the book is for, setup, Zed
├── ground-floor/   programming and Rust from zero
├── part-1…part-7/  the seven-day ramp
└── appendix/       the field guide and reference
rust-aoc-lab/       the zero-dependency practice crate
```
