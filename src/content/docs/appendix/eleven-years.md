---
title: "The eleven years at a glance"
description: "What each Advent of Code event from 2015 to 2025 asked for, and which years to practise."
---

Eleven Decembers are a lot of puzzles. The table below compresses each event into five cells: the story, the kind of work, the idea that year rewarded, the days people found hardest, and the Rust mistake that year made loud.

Read it as a map, not a scoreboard. A year with a hard reputation can still be the right place to start, and a year with a gentle reputation can still contain one day that takes you a week.

The section on practice turns the map into an order. If you only read one part of this page before December, read that one.

## The table

Each year links to its own page on the Advent of Code site. The days in the "hardest" column are the ones most often named in that year's discussions, in rough order.

| Year | Theme | Signature | Hardest days | Rust trap |
| --- | --- | --- | --- | --- |
| [2015](https://adventofcode.com/2015) | Santa, elves, weather machine. Theme, not mechanical. | Invert the problem; bounded brute force is fine (Day 19 backwards, Day 24 subset partition). | 22, 19, 24, 13, 9, 20, 7 | Recursive string rewriting and debug-speed divisor loops. |
| [2016](https://adventofcode.com/2016) | Easter Bunny stole the stars. Theme, not mechanical. | Canonicalise equivalent states and recognise known maths (Day 11 pairs, Day 19 Josephus, Day 15 CRT). | 11, 22, 14, 24, 19, 23 | Fat `HashSet` keys, MD5 in a debug build, mutating an instruction vector. |
| [2017](https://adventofcode.com/2017) | Inside the computer. Mechanical on the machine days. | The input is a program; understand it, do not run it (Day 23 loop replacement, Day 16 permutation cycle). | 23, 3, 21, 24, 18 | `usize` jump underflow, `i64` overflow, `%` on negatives. |
| [2018](https://adventofcode.com/2018) | Chronal time travel. Partly mechanical (ElfCode). | Simulate literally, then compress the state (Day 15, Day 23 octree). | 15, 23, 17, 22, 24, 19 | Mutating a grid while reading neighbours, recursive water depth, `Vec::insert` cost. |
| [2019](https://adventofcode.com/2019) | Space, Intcode. Mechanical: the VM is the substrate for half the month. | Build one Intcode machine and extend it; invert scale (Day 16 tail only, Day 22 affine map). | 18, 22, 20, 25, 16, 14 | Negative or huge addresses, `i128` products, unpacked state keys. |
| [2020](https://adventofcode.com/2020) | Tropical island. Theme, not mechanical. | Part two is a scale change; use arrays, indices, and rings (Day 15, 20, 23). | 20, 19, 23, 22, 13, 18 | `Vec::remove` in a million-item list, `HashMap` for 30 million turns, recursive regex depth. |
| [2021](https://adventofcode.com/2021) | Submarine. Theme, not mechanical. | Enumerate the symmetry group and treat geometry as intervals (Day 19 rotations, Day 22 boxes). | 19, 22, 23, 24, 18, 15 | `f64` rotations, `i64` coordinates, a `HashMap` per cell, `Rc<RefCell<_>>` trees. |
| [2022](https://adventofcode.com/2022) | Jungle expedition. Theme, not mechanical. | Compress the graph before searching; draw the cube net (Day 16 bitmask DP, Day 22 wrapping). | 16, 19, 22, 17, 20, 24, 11 | Multiply before modulo, a fat memo key, `Vec::insert` in a list. |
| [2023](https://adventofcode.com/2023) | Snow production. Partly mechanical (Day 25 min-cut). | Find the closed form (Day 5 intervals, Day 18 shoelace, Day 21 quadratic, Day 24 linear algebra). | 24, 21, 12, 17, 20, 23, 25 | `i32` range overflow, memo table layout, field order in a derived `Ord`, deep DFS, `f64` cross products. |
| [2024](https://adventofcode.com/2024) | Chief Historian, chronospatial computer. Two mechanical devices. | Memoise the recursion, reverse the machine (Day 21 depth/from/to, Day 17 quine, Day 24 adder). | 21, 24, 17, 20, 16, 14, 23 | O(n²) obstacle re-simulation, stone vectors as memo keys, `u32` length overflow, a `Vec` delta key. |
| [2025](https://adventofcode.com/2025) | North Pole base, twelve days. Light theme. | Part two is the real puzzle; model, do not code (Day 10 integer linear system, Day 12 area bound). | 10, 12, 9, 11, 8 | `rem_euclid`, iterating huge ranges, `f64` distances, allocating a 100k × 100k grid, floating-point elimination. |

Read a row across, not down. The theme tells you what the story is about, and almost nothing about the code. The signature tells you what the year rewarded. The hardest column tells you where the wall is. The trap tells you which line the compiler will complain about, or which line it will not.

"Mechanical" means the puzzle hands you a machine and asks you to build or drive it: a made-up computer in 2019, an assembly language in 2017, a min-cut in 2023. The story is still there, but the real work is inside the machine. "Theme, not mechanical" means the story changes and the work stays general: grids, searches, simulations, arithmetic.

The trap column is worth a second look, because half of those mistakes are silent. `usize` underflow panics in a debug build and wraps in a release build. An `i32` that overflows does the same. A `HashMap` with a fat key still returns the right answer, only later than you wanted. A `Vec::remove` in a long list is correct and slow. Learning to hear the difference between "wrong" and "late" is part of the skill.

The signature column is the most useful one after the first week. "Invert the problem" tells you to turn the machine around. "Find the closed form" tells you to stop looping. "Compress the graph" tells you that the map is too big to walk cell by cell. Each phrase is a question you can ask of a new statement before you write anything.

## Which years to practise

**2025 first.** The newest event was a twelve-day sampler rather than a full month. The stories are light, and part two is usually the real puzzle rather than a harder repeat. If you have never finished a day, start here.

**The first week of 2021.** This is the best short course in the three skills the middle of the month needs: breadth-first search on a grid, Dijkstra when the cost varies, and dynamic programming when the same smaller question keeps returning. The first week is gentler than the reputation of the year.

**The first week of 2024.** The first days are parsing and small simulations, and then the month asks you to memoise a recursion and to read a small machine backwards. That combination is the best rehearsal for the two twists that surprise people most.

**2022 for grids.** The year is generous with maps, paths, and one cube that has to be folded on paper before it can be walked in code. If grids are where you feel slow, this is the year to repeat.

**2023 for arithmetic.** Intervals, a quadratic, a linear system, and a shoelace formula. The year rewards a reader who stops to find the closed form instead of running the loop. Save it for after the first two weeks, when a formula feels like a relief rather than a risk.

**2016 for a full month.** It is the gentlest complete year. The puzzles are small, the stories are warm, and the hardest days are hard for reasons you can name: a state space, a counting game, a set of remainders. If you want the experience of twenty-five days in a row, this is the one.

**2019 only if you want the machine.** Half the month is one made-up computer, Intcode, extended a little each day. It is a wonderful way to learn to build a small interpreter and a poor way to learn variety. Choose it on purpose.

**2018 is the simulation tax.** Many days ask you to follow rules exactly and keep a large state tidy. The reward is that the code is honest and the bugs are findable. The cost is typing, and the middle of the month is heavy.

**2015 and 2017 are old-school.** 2015 is language-independent and full of small tricks; 2017 often hands you a program as the input and asks you to understand it without running it. Both reward patience more than speed.

If two years sound equally good, pick the one whose signature you want to practise. The theme is the least important column; the work is the part you will remember.

## A practice route

The route below pairs a goal with two specific puzzles. Do one from each row, in any order, and you will have touched every skill the middle of the month uses. Every reference names a year and a day only.

| Goal | First puzzle | Then |
| --- | --- | --- |
| Parsing and a clean loop | Day 1 of 2015 | Day 1 of 2022 |
| Grid search and distance | Day 12 of 2022 | Day 15 of 2021 |
| Simulation and state | Day 11 of 2020 | Day 15 of 2018 |
| Intervals and arithmetic | Day 5 of 2023 | Day 13 of 2020 |
| A small machine | Day 2 of 2019 | Day 8 of 2020 |
| Reading a program as input | Day 23 of 2017 | Day 17 of 2024 |

The left column names the skill, the middle column gives a gentle first contact, and the right column gives a second puzzle that uses the same skill with more state. If the first one takes an evening, the second one will take two, and that is the point: the second is where the skill settles.

Do not treat the table as a syllabus to finish. Treat it as a set of doors. One door from each row is enough to make the first week of December feel like a place you have visited.

## What the table does not tell you

Difficulty is personal. A day that fits your way of thinking takes an evening; a day that does not can take a week. The hardest column is a summary of other people's experience, not a prediction of yours.

Completion statistics have survivorship bias. The people who are still solving on Day 20 are the people who found the first nineteen days manageable. A low finish rate says something about the wall and something about who was still climbing.

The last third of the month is usually the wall. The puzzles get longer, the statements get denser, and part two stops being a variation. If you plan to stop, stop at a day you chose, not at the first day that makes you feel behind.

The table also cannot show the shape of a single day. A year with three hard days can still have twenty-two gentle ones, and a year with no famous day can still contain the puzzle that teaches you the most.

One more thing the table cannot show: the same puzzle can be attempted twice. The first attempt teaches the pattern; the second, a month later, teaches the shape of your own mistakes. Returning to a day you solved is one of the cheapest ways to find out whether a skill is yours or the code's.

## The shape of the calendar

Every event from 2015 to 2024 ran for twenty-five days, with one puzzle per day and two parts per puzzle. The 2025 event ran for twelve days. The next event starts on December 1, 2026, and the [2025 about page](https://adventofcode.com/2025/about) explains the shorter format in the organisers' own words.

The calendar is the one part of the year you cannot practise. You can practise the parsing, the search, and the arithmetic, but the feeling of a new page at midnight is new every time. The 2026 event will be the twelfth, and it will contain at least one day that no table on this page predicts.

A puzzle appears at midnight on the east coast of the United States. You do not have to be awake for it. The calendar is a set of doors, not a race, and the people who finish are the ones who keep opening doors after a bad day.

Next, [the pattern index](/appendix/pattern-index/) turns the years into archetypes you can name at the moment the statement lands.
