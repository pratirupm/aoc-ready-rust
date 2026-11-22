---
title: "Weigh the cost, order the queue"
description: "Dijkstra's algorithm, a priority queue, and state that includes more than position."
---

The walk we built treated every move as one step. The queue handed back cells in the order it found them, so the first time it reached the exit, the step count was the smallest possible. That works because every open door in the maze costs the same to walk through.

Today some cells carry a price. A digit costs that many coins to enter, and every other open cell costs one. The old queue still finds a way through, but the first way it finds is not always the cheapest way.

We need a queue that hands back the cheapest unfinished place, not the oldest. That queue is a heap. With it, one small change to the search loop gives us Dijkstra's algorithm, the standard answer to "what is the cheapest route?" on a weighted map, meaning a map whose moves have different prices.

Then we add a twist: some puzzles charge for turning. Now the same cell can be cheap to reach from one side and expensive to leave toward another. The fix is a lesson that outlives this puzzle: put the part of the past that changes the future into the state.

## What you need to know

Five ideas carry today's work. Each one is small on its own.

**A heap is a container with a favourite.** A `BinaryHeap` is a container that always hands back the largest item first. It lives in the standard library, so we need no outside crate. `push` adds an item. `pop` removes the largest item and returns an `Option`: either `Some(item)`, or `None` when the heap is empty. A container that returns items by importance instead of by age is called a priority queue.

```rust
use std::collections::BinaryHeap;

let mut heap: BinaryHeap<u64> = BinaryHeap::new();
heap.push(4);
heap.push(9);
heap.push(1);
assert_eq!(heap.pop(), Some(9));
assert_eq!(heap.pop(), Some(4));
```

The largest value leaves first, whatever order it went in. That is a max-heap, and it is the opposite of what a cheapest-first search wants.

**`Reverse` flips the order.** `Reverse` is a wrapper from `std::cmp` that compares backwards. `Reverse(1)` counts as larger than `Reverse(9)`, so a max-heap full of wrapped values behaves like a min-heap. Wrapping every item is cheaper to write than a new container.

```rust
use std::cmp::Reverse;

let mut heap: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
heap.push(Reverse(4));
heap.push(Reverse(9));
heap.push(Reverse(1));
assert_eq!(heap.pop(), Some(Reverse(1)));
```

Popping now gives the smallest wrapped value first. That is the queue Dijkstra needs, and the wrapper adds nothing else to the search.

**Tuples compare left to right.** A tuple is a fixed group of values written in parentheses. Rust compares two tuples by their first fields, and uses later fields only to break a tie. So `(2, 99)` comes before `(3, 0)` even though 99 is larger than 0.

```rust
let cheap: (u64, usize) = (2, 99);
let dear: (u64, usize) = (3, 0);
assert!(cheap < dear);
```

That ordering lets us store `(cost, index)` in the heap and have cost decide the order. The index only settles equal costs, which keeps two runs on the same input identical.

**`u64` is the type for money.** An unsigned integer holds zero and up, never a negative number. A `u64` uses 64 bits and reaches 18,446,744,073,709,551,615, which is far beyond any total these puzzles ask for. A `usize` is the type for a position or a length; it is as wide as the machine needs for indexing.

```rust
let price: u64 = 9;
let index: usize = 42;
```

Costs are `u64`. If a total would ever pass four billion, a `u32` is too narrow, so we choose the wider type from the start.

**`#[derive(...)]` is an attribute we can skip today.** An attribute is a note to the compiler written above an item. `#[derive(Debug, Clone)]` asks Rust to write some routine code for a type, such as the ability to print it. Today's heap entries are tuples of numbers, and tuples of numbers already know how to compare, so there is nothing to derive.

## A queue that returns the cheapest

The map is a graph in disguise: every open cell is a place, and every step to a neighbour is a connection with a price. The price is charged when you enter a cell, not when you leave it. The start cell is free because the walk begins there.

Keep a price board with one slot per cell. `dist[index]` is the cheapest total we have found so far for standing on that cell. At the beginning the board says 0 under `S` and `u64::MAX` under everything else. `u64::MAX` is the largest number a `u64` can hold, so it works as "no route yet"; a real route will always be smaller.

The heap holds offers, not cells: a pair of `(cost, index)` saying "I can reach this cell for this many coins". We begin with one offer for the start at cost 0. Then the loop repeats a short cycle.

Pop the cheapest offer. If the cost is already worse than the board, ignore it. Otherwise the offer is the best known way to that cell, so look at its four neighbours: skip walls, add the price of entering each open neighbour, and if the new total beats the number on the board, write it down and push a fresh offer.

Why is the first good offer for a cell final? Every price is zero or more. If a cheaper route to that cell existed, the first unfinished cell on that route would already have an offer in the heap with a smaller cost, and we would have popped that first. The argument is short, but it is the reason the loop can trust what it pops. Here is the heart of `part1`:

```rust title="rust-aoc-lab/src/bin/drill03.rs"
let mut dist = vec![u64::MAX; grid.cells.len()];
let mut heap = BinaryHeap::new();

dist[start] = 0;
heap.push(Reverse((0u64, start)));

while let Some(Reverse((cost, index))) = heap.pop() {
    if cost > dist[index] {
        continue;
    }
    if index == exit {
        return cost;
    }
    ...
}
```

`while let` unwraps each `Some` and stops when `pop` returns `None`. The `0u64` spells out the type of the first field, which lets Rust work out the rest of the heap's type without a long annotation. The exit check sits after the stale check so that an old, too-expensive offer can never end the search.

The neighbour loop is the other half, often called the relax step: it lowers the board's number whenever a cheaper route appears.

```rust title="rust-aoc-lab/src/bin/drill03.rs"
for next in grid.neighbors4(index) {
    if grid.cells[next] == b'#' {
        continue;
    }
    let next_cost = cost + entry_cost(grid.cells[next]);
    if next_cost < dist[next] {
        dist[next] = next_cost;
        heap.push(Reverse((next_cost, next)));
    }
}
```

`b'#'` is a byte literal: one ASCII character written with a `b` in front and stored as its numeric code, which is how the grid holds walls. `entry_cost` reads a digit as its value and every other byte as one. A cell may be pushed more than once over the run; the next section explains why that is allowed.

## Lazy deletion

A `BinaryHeap` has no operation for lowering the priority of an item already inside it. Searching the heap for the old entry would cost more code and more time than the alternative. So we do not change the entry. We push a second, cheaper offer for the same cell and leave the first one where it is.

The old offer becomes garbage, or stale: it describes a route that has since been beaten. When it eventually reaches the top, the board already holds a smaller number. One comparison catches it, and `continue` jumps to the next turn of the loop.

```rust
while let Some(Reverse((cost, index))) = heap.pop() {
    if cost > dist[index] {
        continue; // an older, dearer offer for the same cell
    }
    // this is the best offer for `index`
    ...
}
```

We never push a cost below the recorded best, so an equal cost means the offer is current. The guard must come before the neighbour loop and before the exit check. If the exit check came first, a stale offer could return a total that a later route has already beaten.

This trick is called lazy deletion. The word lazy is accurate: the cleanup happens when the item surfaces, not when it becomes useless. Duplicates make the heap bigger, but each successful improvement adds only one entry, and each push or pop on a heap is logarithmic in its size: doubling the entries adds about one comparison. For a grid of a hundred thousand cells, that is a comfortable amount of work.

## The state is more than the cell

Part 2 charges three coins for a change of direction. The first move is free because there is no earlier heading to change from. Now two visits to the same cell are not the same situation: arriving from the left leaves you facing right, and arriving from above leaves you facing down. The options ahead depend on which one you are.

So the place in the search is no longer a cell. It is a cell plus a heading. We call that pair the state. The board grows a second dimension: five rows of cells. Rows 0 to 3 are the four headings, and row 4 means "no heading yet", used only at the start.

```rust title="rust-aoc-lab/src/bin/drill03.rs"
let cells = grid.cells.len();
let mut dist = vec![vec![u64::MAX; cells]; 5];
let mut heap = BinaryHeap::new();

// Direction 4 means "no direction yet".
dist[4][start] = 0;
heap.push(Reverse((0u64, start, 4usize)));

...
let turn = if direction != 4 && direction != next_direction {
    3
} else {
    0
};
```

`vec![vec![u64::MAX; cells]; 5]` builds five rows, each with one slot per cell. The heap entry now carries three fields, so equal costs are settled by cell, then by heading; that only affects the order of exploration, not the total.

`next_direction` comes from `DIRS.iter().position(...)`, which returns the position of the matching offset inside the four-element array. The `unwrap` says a match is certain: the step was computed from two neighbouring cells, so it must be one of the four. A turn costs 3, and a straight move costs 0.

Why can the total change? The cheapest route by entry cost can weave around an expensive cell, and weaving means turning. A straighter route may pay more at the door but never turn. Once each turn carries a price, the second route can win. That is not a quirk of the example; it is what adding a price to a move does to the shape of the cheapest path.

Keys and doors work the same way. In [Day 18 of 2019](https://adventofcode.com/2019/day/18) the set of keys you carry decides which doors open, so the state becomes a cell plus a set. The heap and the board do not change; only the number of rows and the tuple grow. Whenever the past changes which moves are legal, it belongs in the state.

## When BFS is enough

A heap is not always the right queue. Breadth-first search, or BFS, is the queue walk that visits cells in rings of equal distance; sometimes it is all the map needs. The cost of a heap operation grows with the logarithm of its size, and on a plain maze that price buys nothing.

When every move costs the same, use the plain first-in, first-out queue. Cells leave it in the order they were found, which is already the order of increasing step count. That was the earlier walk, and it stays the better choice. When you only need to know whether the exit can be reached, any complete search will do, and a stack or queue is enough.

When every price is 0 or 1, there is a middle path called 0-1 BFS. It keeps a double-ended queue: a move that costs 0 goes on the front, and a move that costs 1 goes on the back. The front of the queue always holds the cheapest unprocessed cell, and no heap is needed. The total work stays proportional to the number of cells.

For prices beyond that, reach for the heap. A useful rule: read the price list first. If it contains one number, use a queue. If it contains 0 and 1, use a double-ended queue. Otherwise use Dijkstra.

## The drill

The example is a five-by-seven map. `S` is the start, `E` is the exit, `#` is a wall, and each digit is the price of entering that cell.

```text title="rust-aoc-lab/data/examples/03.txt"
#######
#S9..E#
#.#...#
#..1..#
#######
```

Part 1 asks for the cheapest total entry cost from `S` to `E`, with no charge for turning. Part 2 asks the same question and adds three coins for every change of direction. Both parts live in `rust-aoc-lab/src/bin/drill03.rs`.

Before you read the code, trace the part 1 route by hand. The expensive `9` sits next to the start, but there is a way around it. Then ask what part 2 does to that route: the detour turns, and each turn now has a price.

The two tests hold the expected totals for the example, so they are the oracle. Run them from inside `rust-aoc-lab/`:

```console
$ cargo test --bin drill03
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/bin/drill03.rs (target/debug/deps/drill03-...)

running 2 tests
test tests::drill03_part1_example ... ok
test tests::drill03_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The hash in the file path and the timings differ on your machine, and the tests may print in the other order because they run in parallel. Two `ok` lines mean both totals matched. On a first run you will also see a `Compiling` line above the `Finished` line.

Open `src/bin/drill03.rs` and read `part1` from the top. The pieces should look familiar now: a board, a heap of offers, a stale guard, and a relax step. Then read `part2` as the same function with one extra field. The only new idea is that the heading sits in the state.

## If something went wrong

**The search returns a total that is too high, or falls through to 0.** A plain `BinaryHeap` pops the largest cost first. Wrap every entry in `Reverse`, and write the heap's type once so the wrapper cannot be forgotten: `BinaryHeap<Reverse<(u64, usize)>>`. In part 2 the tuple has three fields, and the wrapper goes around the whole tuple.

**The exit total is one that a later route beats.** The lazy-deletion guard is missing or comes after the exit check. Compare the popped cost with `dist[index]` and `continue` before doing anything else with the offer. The board is the authority, not the heap.

**`attempt to add with overflow`.** A cost was stored in a type too narrow for the total. A `u32` stops at 4,294,967,295. Debug builds panic with that message; release builds wrap around to a small number and give a wrong total with no warning. Use `u64` for every cost and every total.

**Part 2 returns a number that is too small, or a route that seems impossible.** The state key is missing the heading. If `dist` has one row per cell, a cheap arrival from one side hides a different arrival from another, and turns get priced against the wrong heading. Index the board as `dist[direction][index]`, put the heading in the heap entry, and compare against the same pair.

**The compiler asks for the type of an empty heap.** When the first push contains only plain integer literals, there is nothing to pin down their type, and the compiler stops with `error[E0282]: type annotations needed`. Annotate the binding, or write the first push's fields with a suffix such as `0u64`; either one settles the tuple and the wrapper.

## Save this step

Git records a snapshot of the project, called a commit. `git status` lists what changed, `git add .` stages everything under the current folder, and `git commit` writes the snapshot with a message. Run these from the project root:

```console
$ git status
$ git add .
$ git commit -m "Search by cost with a priority queue"
```

The commit hash and branch name will differ on your machine; the message is the one we chose. Saving here means the equal-cost search still exists in history if the weighted version needs to be undone.

<details>
<summary>Going deeper: A*, custom ordering, and tiny maps</summary>

A* is Dijkstra with a guess. For each cell, add an estimate of the remaining distance to the exit and order the heap by the sum. On a grid where every move costs at least one, the Manhattan distance is a safe estimate: the number of rows apart plus the number of columns apart. It never overstates the remaining cost, so the first time the exit is popped, the total is still the cheapest.

```rust
let h = (row.abs_diff(exit_row) + col.abs_diff(exit_col)) as u64;
let priority = cost + h;
heap.push(Reverse((priority, cost, next)));
```

The heap now sorts by `priority`, while the board still stores `cost`. Ties fall to the smaller `cost`, so the order is repeatable; a heuristic tie-break, one that uses an estimate of the distance left, is a separate choice. A guess that overstates the distance can make A* return a route that is not the cheapest, so the estimate has to stay optimistic.

`Reverse` is enough for today, but a heap can also be given a custom order. That needs a type with its own comparison routine, written with `impl Ord`; `Ord` is a trait, a rulebook that says how values of a type are compared. The routine can compare `other.cost` with `self.cost` to turn a max-heap into a min-heap. Traits and generic types arrive later in the book. Until then, the wrapper is shorter and harder to get wrong.

Tie-breaking is a choice, not a correctness issue. Many cells can share a cost. Ordering equal offers by cell index, as the tuple does, makes a run repeatable; ordering them by the A* estimate usually explores fewer cells. Either way the cheapest total is the same, and only the order of work changes.

For a very small map, a heap can be more machinery than the job needs. A `Vec` of offers plus a scan for the smallest entry finds the same cheapest offer in a few lines, and on a few hundred cells the scan is fast. A bucket queue is another option when prices are small whole numbers: one list per price, processed from the lowest price upward. Both trade the heap's logarithmic order for simpler memory access. Measure before choosing; the heap is the safe default.

</details>

The queue now weighs every step. Next, we stop solving the same small question twice and write each answer down: [Remember what you have seen](/part-4/remember-what-you-saw/).
