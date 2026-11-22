# House style

The rules every page in this book follows. Short paragraphs, one idea at a
time, nothing to trip over. `bun run lint` enforces the hard rules; the rest is
craft, checked by reading each page once as a person who has never written a
program, never used Rust, never used Git, and never opened a terminal.

This guide is adapted from the house style of *Learn Rust with Tests* and
*Learn Rust with DSA*. The audience is younger in experience, so the
explanations are slower and every new word is defined where it first appears.

## Who this book is for

Readers who have never written a program. They may have used a computer for
years without opening a terminal. They have never used Rust, Git, or an editor
like Zed. They want to solve Advent of Code puzzles in Rust, and they have a
short runway before December.

Every page is written for a smart, motivated person meeting all of this for the
first time. Nothing is "obvious". Nothing is "easy". If a word is new, define
it in the sentence where it appears.

## The voice

- **"We" build together; "you" type and decide.** Never "I".
- **Plain words first.** Jargon only when the build needs it, defined right
  where it first appears.
- **Short sentences, short paragraphs** (2–4 sentences). If a paragraph needs a
  fifth sentence, it probably wants to be two paragraphs.
- **Warm, never condescending.** If something looks strange, say so and explain
  it. "This looks like a lot — it's three lines" beats silence.
- Contractions are fine. Light humor is fine. Exclamation marks, rarely.
- Do not use "just", "simply", "obviously", or "of course". They make a stuck
  reader feel stupid.
- Do not use "easy" to describe a puzzle, a step, or a language feature. Say
  "small", "short", "gentle", or "familiar".
- Name the thing before you use it. "A `Vec` is a growable list" comes before
  `Vec::new()`.

## Page anatomy

Every teaching page has this shape:

1. **Opening: 2–4 short paragraphs.** What we're building, why it matters, what
   it will feel like when it works. No lists, no meta ("in this chapter you
   will learn…"), no time estimates.
2. **What you need to know:** a short section that introduces the new Rust
   words and types before the code uses them. This can be a list of three or
   four definitions with one example each.
3. **Sections (`##`): 200–800 words each.** One idea per section. Headings say
   what the section does, in plain words.
4. **"If something went wrong"** where the page can produce errors: the real
   messages and the fix, as plain text. This is a reference, not a story.
5. **"Save this step"** where the page produces a working state: a short Git
   sequence. Show `status`, `diff`, or `add` only when it teaches something;
   always commit.
6. **"Going deeper": optional depth** at the end, wrapped in
   `<details>…</details>`. Skippable without missing the build. This is the only
   place background theory lives.
7. **A one-line closing** that points at what's next — never "chapter N".

Target length:

- **Ground floor pages:** 600–1,400 words of prose. Short, focused, one new
  idea each.
- **The seven days:** 1,200–4,000 words of prose per page. `bun run stats`
  checks pages under `part-`.
- **Field guide pages:** length follows the reference. Tables and code blocks do
  not count as prose.

## Hard rules (enforced by `bun run lint`)

- **No CI vocabulary:** CI, workflow, gate, pipeline. The book does not teach
  continuous integration. Say "the daily loop" or "the routine".
- **No test-state checkpoint vocabulary:** "red test", "green test",
  "checkpoint". A test *fails* or *passes*. "Tag" means a Git tag.
- **No chapter-number cross references** ("chapter 7", "as we saw in chapter
  3"). Use "later in the book", "in a moment", "as we just saw". "Day 1" is
  allowed as the name of a part of the ramp.
- **No time estimates** and no "you build / answer key" meta blocks. Do not
  write "7 days", "two hours", or "a 5-minute exercise". "Seven days" as a name
  is fine.
- **No invented output.** Every command and every line of output comes from a
  real run on the pinned toolchain. The setup page and the lab are the source.
- **No Advent of Code puzzle text, puzzle inputs, or answer values.** Refer to a
  puzzle as "2019 Day 18" and link to `https://adventofcode.com/2019/day/18`.
  Teach the pattern with an original drill. This is a copyright and courtesy
  rule, not a suggestion.
- **No external crates in the core lab.** `std` only. Mention crates in the
  field guide when they solve a real problem, never in the first path.

## Code presentation

- **File quotes carry a title:** ` ```rust title="src/main.rs" `. The title bar
  is how the reader knows what file the block belongs to.
- **Terminal blocks show the `$` prompt.** Commands are `$`-prefixed; output
  follows on the lines below, exactly as it appeared. One block per step.
- **Git output is real too.** Hashes, branch names, and paths will differ on the
  reader's machine; say so in the sentence after the block. Never invent a hash.
- **Elisions are a lone `...` line** inside a block, or `...` inline. Never
  silently drop lines.
- **Keep blocks small** — aim for 25 lines or fewer. Quote the interesting part,
  not the whole file.
- **Every code block is followed by at least one sentence** saying what to look
  at or what should happen next.
- **Prefer explicit types in teaching code.** `let mut total: u64 = 0;` teaches
  more than `let mut total = 0;` the first time.
- **Prefer `for` and `while` loops over iterator chains** in the core path.
  Iterator chains belong in "Going deeper".
- **No lifetimes, generics, or custom traits** in the core path. If a page
  needs one, it goes in `<details>` with a plain-language explanation.

## Asides

- `:::tip` — advice that makes life easier.
- `:::note` — context that prevents confusion.
- `:::caution` — something that will bite, with the fix.
- `<details>` — "Going deeper" depth, at page end only.

Use at most a handful per page. If everything is highlighted, nothing is.

## Vocabulary

| Avoid | Prefer |
| --- | --- |
| instantiate, invoke, utilize, leverage | create, call, use |
| prior to, in order to | before, to |
| refactor | clean up the code |
| execute | run |
| repo | repository, project |
| red test, green test | a failing test, a passing test |
| checkpoint | commit, save the state |
| easy, trivial | small, short, gentle, familiar |
| desugar, monomorphize | (leave it for "Going deeper", and define it) |

## Checklist for every page

- [ ] Opening is plain-language, short, and free of meta
- [ ] Every new programming or Rust word is defined at first use
- [ ] One idea per section; headings say what the section does
- [ ] Every code block has a following "what to look at" sentence
- [ ] Errors get their real message and a plain-language fix
- [ ] Git words are defined at first use
- [ ] No banned vocabulary (`bun run lint` is clean)
- [ ] No chapter numbers, no CI talk, no time estimates
- [ ] No Advent of Code puzzle text, inputs, or answers
- [ ] "Going deeper" is genuinely skippable
- [ ] Length within the target for the page type
- [ ] Code and transcripts still match the lab

## Tooling

```
bun run dev          # local site
bun run build        # production build
bun run lint         # prose rules
bun run stats        # prose word counts
bun run links        # internal links in the built site
bun run check        # Astro check and prose lint
```
