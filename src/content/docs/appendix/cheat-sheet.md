---
title: "Cheat sheet"
description: "Short, tested snippets for parsing, grids, search, DP, and modular arithmetic."
---

The snippets here are the working core of the lab, trimmed to the part you reach for on a puzzle morning. The title above a block names the file whose shape it shows. The lab versions, with tests and comments, live in `rust-aoc-lab/src/`, and you can copy them into your own project.

The snippets assume the lab's imports, such as `use rust_aoc_lab::grid::Grid;`. When you copy one into a new project, copy its import line too.

## Reading the input

`fs::read_to_string` returns a `Result`: the whole file as a `String`, or an error. `expect` unwraps the text and stops with the message you give it when the file is missing.

```rust title="src/bin/day01.rs"
use std::fs;

fn main() {
    let input: String = fs::read_to_string("data/inputs/day01.txt")
        .expect("day01 input file");
    let line_count = input.lines().count();
    println!("{line_count} lines");
}
```

`lines` drops the newline at the end of each line, and `count` walks the whole file once.

`parse::blocks` splits on blank lines, and `parse::unsigned` pulls every unsigned integer out of the text in order. A minus sign is treated as a separator, not a sign.

```rust title="src/bin/day01.rs"
use rust_aoc_lab::parse;

let text = "one\n\n12, 345\n";
let blocks: Vec<&str> = parse::blocks(text);
let numbers: Vec<u64> = parse::unsigned(text);
assert_eq!(blocks, vec!["one", "12, 345"]);
assert_eq!(numbers, vec![12, 345]);
```

Both functions return a `Vec`, so you can index the result or loop over it. Use `parse::signed` when negative numbers matter.

A range written `10-20` is two numbers and a separator. `split_once` cuts at the first `-` and gives you the two pieces as an `Option`, which `unwrap` turns into a pair.

```rust title="src/bin/day06.rs"
let (start, end) = line.split_once('-').unwrap();
let interval: (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());
```

The `parse` calls need the type from the tuple annotation, `i64`, because a number's type cannot be guessed from the text alone.

## Grids and search

The `Grid` keeps every cell in one flat `Vec`, row by row. The index of row `r` and column `c` is `r * width + c`, which turns a two-dimensional position into one number.

```rust title="src/grid.rs"
pub struct Grid {
    pub cells: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}
```

A flat index is cheap to store in a queue and quick to compare, which is why the searches below use it instead of a pair.

`neighbors4` returns the indices of the cells above, below, left, and right, skipping any that fall outside the rectangle. `get` takes signed coordinates, so the subtraction at the edge is safe.

```rust title="src/grid.rs"
pub fn neighbors4(&self, index: usize) -> Vec<usize> {
    let (row, col) = self.row_col(index);
    let mut neighbors = Vec::with_capacity(4);

    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if self.get(row as i32 + dr, col as i32 + dc).is_some() {
            let next_row = (row as i32 + dr) as usize;
            let next_col = (col as i32 + dc) as usize;
            neighbors.push(self.idx(next_row, next_col));
        }
    }

    neighbors
}
```

Use `neighbors8` for the diagonals; it is the same loop over `-1..=1` in both directions.

Breadth-first search uses a queue and a distance table. `usize::MAX` means "not reached yet", so the same check both skips walls and stops a cell from entering the queue twice.

```rust title="src/bin/drill02.rs"
use std::collections::VecDeque;

use rust_aoc_lab::grid::Grid;

pub fn distances(grid: &Grid, start: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; grid.cells.len()];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(index) = queue.pop_front() {
        for next in grid.neighbors4(index) {
            if grid.cells[next] == b'#' || dist[next] != usize::MAX {
                continue;
            }
            dist[next] = dist[index] + 1;
            queue.push_back(next);
        }
    }

    dist
}
```

The queue visits cells in rings, so the first time a cell is reached is the shortest path to it. Call the function once, then read `dist[exit]` for the answer or loop over the table to count reachable cells.

Dijkstra replaces the queue with a min-heap. `Reverse` turns the max-heap into a min-heap, and the `(cost, index)` pair orders by cost first. The `cost > dist[index]` check discards stale entries without deleting them, which is the lazy-deletion trick.

Inside `part1`:

```rust title="src/bin/drill03.rs"
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use rust_aoc_lab::grid::Grid;

let (grid, start, exit) = input;
let mut dist = vec![u64::MAX; grid.cells.len()];
let mut heap = BinaryHeap::new();
dist[*start] = 0;
heap.push(Reverse((0u64, *start)));

while let Some(Reverse((cost, index))) = heap.pop() {
    if cost > dist[index] {
        continue;
    }
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
}
```

The `entry_cost` function is the drill's small price table: a digit costs its value, and every other cell costs one. Return `cost` when you pop the exit; the first pop is the cheapest, because every later path costs at least as much.

## Counting and remembering

A memo turns a recursion that repeats itself into one that computes each state once. The `Option` marks a state as not computed yet; after the loop, the answer is stored for the next caller.

```rust
fn ways(target: usize, memo: &mut Vec<Option<u64>>) -> u64 {
    if target == 0 {
        return 1;
    }
    if let Some(value) = memo[target] {
        return value;
    }
    let mut total: u64 = 0;
    for step in 1..=3 {
        if step <= target {
            total += ways(target - step, memo);
        }
    }
    memo[target] = Some(total);
    total
}

let mut memo: Vec<Option<u64>> = vec![None; 21];
let count = ways(20, &mut memo);
```

The table is a `Vec` because the states are small integers from `0` to the target; a `HashMap` is the right shape when the state is a tuple or a string. The recursion meets the same remainders in a different order many times, and the memo turns the second visit into a table read.

Counting shortest paths uses the distance table and then a second table of counts. Sorting the reached cells by distance puts every contributor before the cell that needs it.

Inside `part2`, after the distance table exists:

```rust title="src/bin/drill07.rs"
let mut ways = vec![0u64; grid.cells.len()];
ways[*start] = 1;

let mut order: Vec<usize> = (0..grid.cells.len())
    .filter(|&index| dist[index] != usize::MAX)
    .collect();
order.sort_by_key(|&index| dist[index]);

for index in order {
    for next in grid.neighbors4(index) {
        if dist[next] == dist[index] + 1 {
            ways[next] = (ways[next] + ways[index]) % MOD;
        }
    }
}

ways[*exit]
```

The count at the start cell begins at one because there is one empty walk at the start; each open edge one ring forward adds the count behind it. The modulo keeps the count inside a `u64`; the puzzle states when it wants a remainder.

## Groups, ranges, and numbers

A disjoint-set union keeps groups of numbered items. `find` returns the representative of a group, and `union` joins two groups and reports whether they were separate.

```rust title="src/dsu.rs"
pub fn find(&mut self, mut item: usize) -> usize {
    while self.parent[item] != item {
        self.parent[item] = self.parent[self.parent[item]];
        item = self.parent[item];
    }
    item
}

pub fn union(&mut self, a: usize, b: usize) -> bool {
    let mut root_a = self.find(a);
    let mut root_b = self.find(b);
    if root_a == root_b {
        return false;
    }
    if self.size[root_a] < self.size[root_b] {
        std::mem::swap(&mut root_a, &mut root_b);
    }
    self.parent[root_b] = root_a;
    self.size[root_a] += self.size[root_b];
    self.sets -= 1;
    true
}
```

Path halving in `find` keeps the trees short, and the size check in `union` keeps the smaller tree under the larger one. `Dsu::new(count)` starts with one set per item, and `count()` reports how many sets remain.

Intervals are inclusive pairs, and `merge` joins any two that touch or overlap. Sorting first means one pass is enough: each new interval either extends the current one or starts a new one.

```rust title="src/intervals.rs"
pub fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return intervals;
    }
    intervals.sort();

    let mut merged = Vec::new();
    let (mut start, mut end) = intervals[0];

    for &(next_start, next_end) in &intervals[1..] {
        if next_start <= end + 1 {
            end = end.max(next_end);
        } else {
            merged.push((start, end));
            start = next_start;
            end = next_end;
        }
    }

    merged.push((start, end));
    merged
}
```

`next_start <= end + 1` is what makes `(1, 3)` and `(4, 6)` merge into `(1, 6)`. `gaps` is the same idea in reverse: it returns the holes between the merged intervals inside a low and high bound.

`total_length` walks the merged list once and adds the number of integers in each interval:

```rust title="src/intervals.rs"
pub fn total_length(intervals: &[Interval]) -> i64 {
    intervals.iter().map(|&(start, end)| end - start + 1).sum()
}
```

The `+ 1` is the fence-post correction: an interval from 10 to 20 contains 11 integers, not 10. A call looks like this:

```rust
let merged = intervals::merge(vec![(10, 20), (15, 25), (30, 40)]);
let covered = intervals::total_length(&merged);
```

The `covered` value counts each distinct position once, even when the input intervals overlapped.

`gcd` and `lcm` handle cycles that must line up, and `lcm` divides before multiplying so the intermediate value stays small.

```rust title="src/num.rs"
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}
```

`mod_pow` raises a base to a large power under a modulus, and `rem_euclid` returns a non-negative remainder even when the value is negative:

```rust
let day = num::lcm(4, 6);
let power = num::mod_pow(2, 10, 1000);
let wrapped = num::rem_euclid(-3, 7);
```

Use `rem_euclid` for positions that wrap around a track; the ordinary `%` keeps the sign of a negative value. `lcm_many` folds a list of cycles into one aligned period.

## Time and state

A simulation with a huge target is usually a cycle in disguise. Store each state with the time it first appeared. When the state returns, the cycle length is the difference, and the target time can be reduced to one index inside the cycle.

```rust title="src/bin/drill05.rs"
use std::collections::HashMap;

let mut seen: HashMap<Vec<(i64, i64)>, u64> = HashMap::new();
seen.insert(input.particles.clone(), 0);
let mut time: u64 = 0;

while time < input.target {
    let next = step(&states[time as usize], input.track);
    time += 1;

    if let Some(&first_time) = seen.get(&next) {
        let cycle = time - first_time;
        let index = first_time + (input.target - first_time) % cycle;
        return checksum(&states[index as usize]);
    }

    seen.insert(next.clone(), time);
    states.push(next);
}
```

The `states` vector keeps the history so the answer at the reduced index can still be read; the hash map only stores the first time each state was seen. The modulo maps the target time into the cycle, and the `first_time` offset puts it back at the right index.

The full versions of these snippets are in `rust-aoc-lab/src/`, with a test for each module and a drill that uses every piece.

Next: [the colophon](/appendix/colophon/) records how the book was built and tested.
