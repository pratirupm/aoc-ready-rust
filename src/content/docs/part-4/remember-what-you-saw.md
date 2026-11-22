---
title: "Remember what you have seen"
description: "Memoisation, bottom-up dynamic programming, and counting paths."
---

Some questions come back. You count the orders in which a staircase can be climbed, one step or two or three at a time, and the count for a short climb keeps appearing inside the count for a longer one. Writing the function straight from the description works, but it solves the same small question again and again.

On forty steps, the same two-step ending is counted thousands of times. The work grows faster than the staircase. Most of it is repetition.

The cure is a notebook. The first time a small question is answered, write the answer down. The next time the question appears, read the notebook instead of working it out again. In a program, the notebook is a table, and the small question is called a subproblem: a smaller version of the question we actually asked.

When the notebook sits beside a function that calls itself, the trick is called memoisation. When we fill the table in order from the smallest subproblem upward, it is called dynamic programming. Today we count orders of steps, orders with a restriction, and shortest paths through a grid.

## What you need to know

**Recursion is a function that calls itself.** Each call gets its own copy of the parameters, and each copy holds a smaller number than the one before. The function must move toward a stopping point on every call.

```rust
fn countdown(n: u64) {
    if n == 0 {
        return;
    }
    println!("{n}");
    countdown(n - 1);
}
```

`countdown(3)` prints 3, then calls `countdown(2)`, which calls `countdown(1)`, which calls `countdown(0)`. The bare `return` leaves the function early, before any printing.

**A base case is the input that stops the recursion.** For `countdown`, the base case is `n == 0`: it answers without calling itself. Every recursive function needs at least one, and every call must get closer to it. Without a base case, the calls never stop.

**A `Vec<u64>` table holds one answer per subproblem.** The index is the subproblem's number, and the value is its answer. `vec![0u64; 6]` builds six slots, all holding zero.

```rust
let mut table: Vec<u64> = vec![0u64; 6];
table[0] = 1;
table[5] = table[0] + 7;
assert_eq!(table[5], 8);
```

Slot 0 holds 1 and slot 5 holds 8, while the untouched slots still hold 0. A zero can mean "answer is zero" or "not filled yet", so read the code carefully when zero is a possible answer.

**A `HashMap` is a dictionary.** It maps a key to a value. `insert` adds a pair, and `get` looks one up, returning an `Option` because the key may be absent.

```rust
use std::collections::HashMap;

let mut memo: HashMap<u64, u64> = HashMap::new();
memo.insert(5, 13);
let found: Option<u64> = memo.get(&5).copied();
assert_eq!(found, Some(13));
```

`get` takes a reference to the key, hence `&5`, and `.copied()` turns the `Option<&u64>` into an `Option<u64>` so the answer is an ordinary number. A `HashMap` earns its keep when the keys are pairs, or when only a few of all possible keys are ever used.

**`Option` means "a value or nothing".** It is either `Some(value)` or `None`. `HashMap::get` uses it for a missing key, and the memo table below uses it for a slot that has not been filled. `if let Some(answer) = ...` runs the body only when a value is present.

**`u64` counters grow quickly.** A count of orders can pass four billion long before the input looks large. A `u64` reaches 18,446,744,073,709,551,615, which is roomy but not endless. Rust stops with an overflow panic in a debug build and wraps around in a release build, so the type has to be chosen with the growth in mind.

## The same subproblem twice

Take a staircase and allow steps of 1, 2, or 3. A sequence is an ordered list of steps, so 1 then 2 and 2 then 1 are different sequences even though both reach 3. Let `ways(n)` be the number of sequences whose steps add up to `n`.

The empty sequence is one sequence, so `ways(0) = 1`. For any larger `n`, look at the first step. It is 1, 2, or 3. If it is 1, the rest of the sequence must add up to `n - 1`, and there are `ways(n - 1)` ways to choose that rest. The same holds for a first step of 2 and of 3. The three groups do not overlap, because their first steps differ, so the counts add:

```text
ways(n) = ways(n - 1) + ways(n - 2) + ways(n - 3)
```

A rule that expresses a subproblem in terms of smaller subproblems is called a recurrence. Terms for negative totals are skipped, and `ways(0) = 1` is the base case. Here is the call tree for `ways(4)`, with the arrows marking a call that reaches the base case:

```text
ways(4)
├─ ways(3)
│  ├─ ways(2)
│  │  ├─ ways(1) → ways(0)
│  │  └─ ways(0)
│  ├─ ways(1) → ways(0)
│  └─ ways(0)
├─ ways(2)
│  ├─ ways(1) → ways(0)
│  └─ ways(0)
└─ ways(1) → ways(0)
```

Count the boxes. `ways(2)` appears twice and `ways(1)` appears four times, and each repeat redoes the same small sum. At `ways(40)` the duplication is enormous, even though there are only forty-one distinct subproblems.

The fix is to compute each subproblem once and keep the answers in a table. The lab fills the table from the bottom up:

```rust title="rust-aoc-lab/src/bin/drill04.rs"
fn ways_any(target: u64) -> u64 {
    let target = target as usize;
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;

    for total in 1..=target {
        for step in 1..=3 {
            if total >= step {
                dp[total] += dp[total - step];
            }
        }
    }

    dp[target]
}
```

The table has one slot per total, from 0 through `target`; the `+ 1` makes room for slot 0. `1..=target` is an inclusive range, so the loop visits every total in order. By the time it reaches `total`, the three smaller slots it reads are already final, which is why one pass is enough. `target as usize` converts the total into the type used for indexes.

The name `dp` is short for dynamic programming, the table itself. The two loops do not care which step is considered first, because addition does not care about order.

## Top-down and bottom-up

The table can be filled in two directions. The version above is bottom-up: start at the smallest subproblem and walk upward, so every dependency is ready before it is needed. The other version starts from the question we actually asked and works downward, writing an answer the first time it is computed.

```rust
fn ways(total: usize, memo: &mut Vec<Option<u64>>) -> u64 {
    if total == 0 {
        return 1;
    }
    if let Some(answer) = memo[total] {
        return answer;
    }
    let mut sum: u64 = 0;
    for step in 1..=3 {
        if total >= step {
            sum += ways(total - step, memo);
        }
    }
    memo[total] = Some(sum);
    sum
}
```

`memo` is a notebook passed down every call. `Some(answer)` means the subproblem is already solved, so the function returns the note. After the loop, it writes the new answer before returning it. The first call is `ways(target, &mut memo)`, with a table of `target + 1` empty slots.

Top-down mirrors the recurrence almost word for word, and it visits only the subproblems reachable from the target. That matters when most of the table would stay empty, as in a search over cells where many positions are walls. Bottom-up visits every slot, but each visit is a loop step with no function call, and it cannot run out of stack.

A function call is not free: the machine keeps a frame for each active call, holding its parameters, its local variables, and where to return. The frames live on the stack, a region of memory with a fixed size. A recursion that goes thousands of calls deep can fill it. The program then stops with `thread 'main' has overflowed its stack` and `fatal runtime error: stack overflow`. If a bottom-up table can express the same recurrence, use it for deep inputs. The example targets here are small enough that either shape works.

## The key is the whole future

Part 2 forbids two 3s in a row. The recurrence now needs to know how the sequence ended. A total of 5 reached by 2 then 3 may not take another 3, while the same total reached by 3 then 2 may. The history matters through one bit: was the last step a 3?

So the subproblem is no longer a total. It is a pair: the total, and whether the last step was a 3. That pair is the key. Two histories with the same key must have exactly the same options left; otherwise the key has thrown away something the future needs.

The table grows a second dimension. Each slot holds two counters, one for each value of `last_was_three`.

```rust title="rust-aoc-lab/src/bin/drill04.rs"
let mut dp = vec![[0u64; 2]; target + 1];
dp[0][0] = 1;

for total in 0..=target {
    for last_was_three in 0..2 {
        let current = dp[total][last_was_three];
        if current == 0 {
            continue;
        }

        for step in 1..=3usize {
            if step == 3 && last_was_three == 1 {
                continue;
            }
            if total + step > target {
                continue;
            }
            let next_state = if step == 3 { 1 } else { 0 };
            dp[total + step][next_state] += current;
        }
    }
}
```

`[0u64; 2]` is a fixed array: a row of exactly two counters, with the length written into its type. `vec![...; target + 1]` makes one such pair per total. `dp[0][0] = 1` says the empty sequence reaches total 0 and does not end in a 3; `dp[0][1]` stays 0 because no sequence ends in a 3 before taking any step.

The loop reads a state, skips it when its count is zero, and pushes its count forward into the states one step away. A step of 3 is refused when `last_was_three` is 1. The `next_state` line records whether the step that was taken is a 3. Pushing forward like this is the same sum as the recurrence, only written from the other side.

At the end, the answer is `dp[target][0] + dp[target][1]`, because a finished sequence may or may not end in a 3, and both kinds are valid. That sum is the one place where the two states are merged.

This is the general lesson, and it reaches beyond staircases. If a fact about the past changes which moves are legal next, that fact belongs in the key. In the key-and-door puzzle from [Day 18 of 2019](https://adventofcode.com/2019/day/18), the set of keys held decides which doors open, so the key becomes a cell plus a set. In a puzzle with a one-use boost, the key records whether the boost is spent. The table and the loop stay the same shape; only the key grows.

## Counting paths

The same idea counts shortest paths. Take the grid walk that measures distance from the start. A breadth-first search — the queue walk that visits cells in rings of equal distance — fills `dist`, where `dist[index]` is the number of steps from the start to that cell, or `usize::MAX` when the cell cannot be reached.

Now add a second table, `ways`, where `ways[index]` counts the shortest paths from the start to that cell. The start has one shortest path: the empty one. Every other count is a sum over the cells that lead into it along a shortest path.

```rust title="rust-aoc-lab/src/bin/drill07.rs"
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
```

`order` collects the reachable cells and sorts them by distance. For each cell, the inner loop looks for neighbours exactly one step farther from the start. Every shortest path to `index` can gain one more step and become a shortest path to such a neighbour, so the count at `index` is added to it. The function returns the count stored for the exit cell.

The distance test is what keeps the count honest. A neighbour at the same distance would make a loop, and a neighbour two steps away would not be part of a shortest path. Only a connection that climbs by exactly one belongs.

Sorting by distance makes the additions safe. Every cell that feeds `index` has a smaller distance, so it appears earlier in `order` and its count is already final when `index` is processed. The sorted order is a dependency order: each cell comes after the cells it depends on.

`MOD` is `1_000_000_007`, and the percent sign takes the remainder after division. The underscores in the number are digit separators, ignored by Rust. Taking the remainder at every addition keeps the counter from overflowing; the puzzle that asks for this reports the remainder, not the full count.

Counting paths with a plain depth-first search — one that follows a route as far as it can before backing up — would be a mistake. It would walk into the same cell from many routes and recount every suffix, which is the duplication from the staircase all over again. The distance table turns the grid into a ladder, and one pass up the ladder counts everything once.

## The drill

The example is a short list of target totals, one per line. `parse` reads the file into a `Vec<u64>`.

```text title="rust-aoc-lab/data/examples/04.txt"
12
20
30
40
```

Part 1 sums `ways_any` over the four targets: every sequence of 1s, 2s, and 3s that adds up to each target. Part 2 sums `ways_no_double_three`, with the extra rule that no two 3s may sit next to each other. Both functions live in `rust-aoc-lab/src/bin/drill04.rs`.

The tests hold the expected totals for the example, so they are the oracle. Run them from inside `rust-aoc-lab/`:

```console
$ cargo test --bin drill04
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/bin/drill04.rs (target/debug/deps/drill04-...)

running 2 tests
test tests::drill04_part1_example ... ok
test tests::drill04_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The hash in the file path and the timings differ on your machine, and the two tests may print in either order because they run in parallel. Two `ok` lines mean both totals matched. On a first run you will also see a `Compiling` line above the `Finished` line.

Open `src/bin/drill04.rs` and read `ways_any` before `ways_no_double_three`. The first is a single row of counts; the second is the same loop with a second index. The difference between them is the whole idea of today's page.

## If something went wrong

**`thread 'main' has overflowed its stack`.** A recursive function went too deep, and the stack ran out of room for frames. For a counting recurrence, rewrite the function as a bottom-up loop over a `Vec`. If recursion is truly needed, the state space has to be small enough for the stack, or the work has to move to an explicit `Vec` used as a stack.

**Part 2 counts sequences with two 3s in a row.** The memo key forgot the last move. A one-dimensional table indexed only by total cannot tell a sequence ending in 3 from one that does not, so it permits the forbidden step. Add the second index, `last_was_three`, and write to `dp[total + step][next_state]`.

**`attempt to add with overflow`.** The counter passed the top of `u64`, or a smaller type was used for a total. Debug builds panic with that message; release builds wrap around and give a smaller, wrong count. Use `u64` throughout, and if the puzzle asks for a remainder, apply `%` at every addition. A modulus near one billion keeps every intermediate sum inside a `u64`.

**`error[E0499]: cannot borrow `*memo` as mutable more than once at a time`.** The code kept a reference into the table alive across a recursive call. `let slot = &mut memo[total];` borrows the table mutably, and passing `memo` to the next call asks for a second mutable borrow while the first is still alive. Copy the number out, drop the reference, recurse, then write back through a fresh index. The `if let` version above copies the `Option<u64>` and releases the borrow before the recursive call.

**The table is one slot short.** A table for totals 0 through `target` needs `target + 1` slots. With `vec![0u64; target]`, the final index is out of bounds and Rust stops with `index out of bounds`. Check every `vec![...; n]` against the largest index the loop writes.

## Save this step

Git keeps the history. `git status` lists what changed, `git add .` stages everything under the current folder, and `git commit` saves a snapshot, called a commit. Run these from the project root:

```console
$ git status
$ git add .
$ git commit -m "Count orders with a memo table"
```

The commit hash and branch name will differ on your machine; the message is ours. Saving here keeps the weighted search from the previous day untouched while the counting code grows beside it.

<details>
<summary>Going deeper: key sets, tours, and remainders</summary>

A set of keys fits in one integer. Number the keys from 0, and let bit `k` be 1 when key `k` is held. A bit is one binary digit, and `1u32 << 3` is 8, a mask with only bit 3 set. Adding a key is `mask | bit`, and testing for one is `mask & bit != 0`. The state becomes a cell plus a mask, and the visited table has one row per mask. With ten keys that is 1,024 rows, which is why the trick only suits puzzles with a modest number of keys.

The travelling salesman problem asks for the cheapest tour that visits every city once and returns to the start. A table indexed by `mask` and `last` holds the cheapest way to visit the cities in `mask` and stop at `last`. Each entry takes the smallest value over the possible previous city, adding the cost of the final hop. The table has `2^n * n` entries for `n` cities, so the method suits around fifteen to twenty cities and no more. It is dynamic programming over subsets, with the same key discipline as the staircase.

`Vec<Option<u64>>` and `HashMap<u64, u64>` solve the same job with different costs. The `Vec` is faster and smaller when keys are dense whole numbers, because the key is the index and no hashing happens. The `HashMap` shines when keys are pairs, strings, or a sparse scattering over a huge range. A pair such as `(total, last)` can still use a `Vec` by computing one index, `total * 2 + last`, which is what the two-dimensional table does under the hood.

A modulus such as `1_000_000_007` is common in counting puzzles because it is prime, meaning it has no divisors except 1 and itself, and it fits in 32 bits. The modulus is the number you divide by, and remainders after dividing by a prime are well behaved under multiplication as well as addition, which matters when counts are combined rather than summed. Apply the remainder after each operation, not at the very end: the point is to keep every intermediate value small. When a puzzle wants the exact count, as this drill does, leave the modulus out and make sure `u64` is wide enough.

</details>

Counting is now a habit: solve each small question once, and put the answer where the next question can find it. Next, we let a machine run for a while and learn to jump over the part that repeats: [Simulate, then skip ahead](/part-5/simulate-then-skip/).
