---
title: "The pattern index"
description: "A map from puzzle archetype to the page that teaches it and example years."
---

Most Advent of Code puzzles are a familiar shape wearing a new story. The story is what you read first; the shape is what you write. This page names thirty shapes, points at the page that teaches each one, and gives two example days so you can go and look.

The names are not official. They are labels we chose because they fit in a sentence and because they help at the moment a statement lands. Use them to ask a better question than "what do I do?", namely "what kind of thing is this?"

Every example is a year and a day, nothing more. If a row names a day you have not seen, open it on the Advent of Code site and read only the statement. The label will make more sense after you have seen the shape in the wild.

## The thirty archetypes

The middle column says where the book builds the shape. The last column names two days that use it.

| Archetype | One-line definition | Where the book teaches it | Example years and days |
| --- | --- | --- | --- |
| Line aggregation | Read one line at a time, turn it into a number or record, and combine the results. | [Read the input](/part-1/read-the-input/) | Day 1 of 2021 and Day 1 of 2022 |
| Grid turtle | Move a marker cell by cell, turning and stepping as the input says. | [Read the input](/part-1/read-the-input/) and [Walk the grid](/part-2/walk-the-grid/) | Day 12 of 2020 and Day 9 of 2022 |
| Neighbourhood counting | For each cell, inspect the cells around it and count what you find. | [Read the input](/part-1/read-the-input/) | Day 11 of 2020 and Day 11 of 2021 |
| Flood fill/regions | Spread from a cell through everything reachable, then label or measure the region. | [Walk the grid](/part-2/walk-the-grid/) | Day 9 of 2021 and Day 14 of 2017 |
| Uniform-cost BFS | Explore in rings of one step so the first arrival is the shortest path. | [Walk the grid](/part-2/walk-the-grid/) | Day 12 of 2022 and Day 18 of 2024 |
| Dijkstra with augmented state | Find the cheapest path when costs differ and the state carries more than a position. | [Weigh the cost](/part-3/weigh-the-cost/) | Day 15 of 2021 and Day 17 of 2023 |
| A* | Run Dijkstra but order the queue with a guess about the distance still to go. | [Weigh the cost](/part-3/weigh-the-cost/) | Day 17 of 2023 and Day 16 of 2024 |
| Implicit-state BFS (position + mask) | Search states built on the fly, such as a position plus the set of things collected. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 24 of 2016 and Day 18 of 2019 |
| Strict-order simulation | Follow the rules in the stated order and repeat until a whole round changes nothing. | [Simulate, then skip ahead](/part-5/simulate-then-skip/) | Day 15 of 2018 and Day 15 of 2024 |
| Cellular automaton | Update every cell from its neighbours at the same time, tick after tick. | [Simulate, then skip ahead](/part-5/simulate-then-skip/) | Day 18 of 2015 and Day 11 of 2020 |
| Cycle detection | Run until a state repeats, then skip whole cycles instead of replaying them. | [Simulate, then skip ahead](/part-5/simulate-then-skip/) | Day 16 of 2017 and Day 14 of 2023 |
| Linked list/ring buffer | Keep an order that changes in the middle without shifting every later item. | [Simulate, then skip ahead](/part-5/simulate-then-skip/) | Day 23 of 2020 and Day 20 of 2022 |
| Interval merge/sweep | Sort ranges by a boundary, then combine or measure them in one pass. | [Do the math](/part-6/do-the-math/) | Day 5 of 2023 and Day 20 of 2016 |
| Range translation/splitting | Push whole ranges through a rule and split only at the boundaries. | [Do the math](/part-6/do-the-math/) | Day 5 of 2023 and Day 22 of 2021 |
| Modular arithmetic/CRT | Work with remainders, and combine cycles with the Chinese remainder theorem. | [Do the math](/part-6/do-the-math/) | Day 13 of 2020 and Day 15 of 2016 |
| LCM of parallel cycles | When independent cycles repeat, the system repeats at their least common multiple. | [Do the math](/part-6/do-the-math/) | Day 12 of 2019 and Day 8 of 2023 |
| Ordered hash map | Remember values by key and walk the keys in the order you need. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 7 of 2015 and Day 14 of 2020 |
| Expression parser | Turn text with brackets and operators into a tree, then evaluate the tree. | [Do the math](/part-6/do-the-math/) | Day 18 of 2020 and Day 18 of 2021 |
| Grammar/recursive matching | Match nested rules by letting one rule call another, with a table to avoid repeats. | [Do the math](/part-6/do-the-math/) | Day 19 of 2020 and Day 12 of 2023 |
| VM/interpreter | Read a list of instructions and change registers or memory as you go. | [Simulate, then skip ahead](/part-5/simulate-then-skip/) | Day 2 of 2019 and Day 8 of 2020 |
| Assembly reverse engineering | Read a program's instructions, work out what it computes, and replace the slow part. | [Put it together](/part-7/put-it-together/) | Day 23 of 2017 and Day 17 of 2024 |
| Topological order | Put every dependency before the thing that needs it. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 7 of 2018 and Day 5 of 2024 |
| Union-find | Join groups and ask whether two items share a group. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 14 of 2017 and Day 25 of 2018 |
| Tree traversal | Walk a hierarchy from the root, or build it from parent links. | [Walk the grid](/part-2/walk-the-grid/) | Day 7 of 2015 and Day 7 of 2022 |
| Sequence DP | Build answers for longer prefixes out of answers for shorter ones. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 21 of 2021 and Day 19 of 2024 |
| Subset sum/partition | Choose items whose sizes hit a target exactly. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 17 of 2015 and Day 24 of 2015 |
| Bitmask DP/TSP | Use the bits of one integer to record which items are used, then search over those masks. | [Remember what you have seen](/part-4/remember-what-you-saw/) | Day 16 of 2022 and Day 9 of 2015 |
| Binary search on the answer | Guess a number, test whether it works, and halve the range. | [Do the math](/part-6/do-the-math/) | Day 14 of 2019 and Day 5 of 2023 |
| Computational geometry | Compare directions, distances, and intersections with care for rounding. | [Do the math](/part-6/do-the-math/) | Day 19 of 2021 and Day 24 of 2023 |
| Polygon area (shoelace/Pick) | Get a shape's area from its corners, and count the lattice points inside. | [Do the math](/part-6/do-the-math/) | Day 10 of 2023 and Day 18 of 2023 |

Read a row from left to right. The definition is the part to memorise; the page is where the code lives; the days are where the shape appeared for real.

Some of the names overlap on purpose. A flood fill is a breadth-first search with a different question, and a cellular automaton is a strict-order simulation where the order is "everyone at once". The index is a set of handles, not a taxonomy.

## What the table leaves out

Most puzzles combine two archetypes. A grid plus a mask, a search plus an interval, a parser plus dynamic programming. The table gives one name per row because a name is easier to carry than a paragraph.

The archetype describes the state, not the code. Two days can share a row and need different containers: one a `VecDeque`, one a `BinaryHeap`, one a `HashMap`. The row tells you what changes from step to step; the page tells you which tool the book used.

A day missing from the table is not a day without a pattern. Some of the best days are mostly reading: a long statement, a small rule, and a tidy loop. Reading is a skill, and it does not need an entry here to be worth practising.

The example days are hints, not limits. If a day named here feels wrong when you read it, trust the statement. The label is a starting question, and the statement is the authority.

## When two archetypes fit

Say the state out loud before choosing. A state is the smallest set of facts that changes from one step to the next. If the position alone repeats, a plain breadth-first search is enough. If the position repeats with a direction, the key grows. If the position repeats with a set of collected items, a mask joins the key.

Then count the states. Two archetypes can both be correct, and the one with fewer states is easier to test. A search with a million states is a morning; a search with a billion states is a different algorithm.

Finally, expect the pair. A first part that fits one row often has a second part that adds a second row. The new row is usually the one that changes the state, not the one that changes the story.

## How to use this during the event

Read the statement once for the story. Then read it again and write down what one line of input looks like and what the two questions ask for.

Name the archetype. Say it as a sentence: "this is a uniform-cost BFS with a mask", or "this is an interval merge with a translation". If two names fit, write both and circle the one that describes the state.

Open the page for the primary archetype. Copy the shape, not the puzzle: the parse function, the tests, the loop, and the data structures. Rename the fields, keep the skeleton, and get the example passing before you touch the real input.

Adapt the shape. A grid search becomes a graph search when the neighbours come from a rule instead of a map. A simulation becomes a cycle skip when the state repeats. The page teaches one concrete version; the adaptation is yours.

If the statement refuses a name, write the example by hand and look for the thing that repeats. The repetition is usually the archetype. And if the day still refuses, that is not a gap in the index; it is the puzzle doing its job.

Next, [the twelve-day playbook](/appendix/twelve-day-playbook/) turns this index into a plan for the first twelve days of December.
