---
title: "Simulate, then skip ahead"
description: "Follow rules exactly, detect a repeated state, and jump to the interesting moment."
---

Some puzzles hand you a rule and a clock. A particle moves three cells every second. A guard turns right whenever it meets a wall. A small machine reads a list of numbers and changes its own memory. You could work out the answer with a pencil, but the question asks for the state at tick ten or tick a trillion, and the pencil gives up long before that.

A program is a machine you can run. You write the rules once, set a counter, and step the clock. That works until the clock is measured in trillions.

The trick that saves those puzzles is small. The machine cannot keep finding new states forever, because it only has so much room. A state that comes back starts a loop. Once you know the loop, you can jump over almost all of it.

We will build a tiny simulation: particles on a circular track, moving at fixed speeds. First we run it exactly, one tick at a time. Then we run it with a memory of states, find the loop, and read the answer at a time far beyond any loop we could count to.

## What you need to know

A `struct` is a type that groups named values. Each value inside it is a **field**: a name and a type. The lab's parsed input is one struct with four fields.

```rust title="src/bin/drill05.rs"
#[derive(Debug, Clone)]
pub struct Input {
    pub track: i64,
    pub steps: u64,
    pub target: u64,
    pub particles: Vec<(i64, i64)>,
}
```

`Input` is the new type. `track`, `steps`, `target`, and `particles` are its fields, and `pub` means other files may read them. The line above the struct asks Rust to write two helpers for us. `Debug` lets `{:?}` print the whole value while we work, and `Clone` adds `.clone()`, which makes a full copy. `#[derive(...)]` is how we ask for those helpers without writing them by hand.

`i64` is a whole number that can be negative, from about minus nine quintillion to about nine quintillion. `u64` is a whole number that cannot be negative, from zero to about eighteen quintillion. The `64` counts bits: more bits means a wider range. A **bit** is one yes-or-no slot in the machine's memory.

A `Vec` is a growable list. `Vec<(i64, i64)>` is a list of pairs, and a pair like `(0, 3)` is a **tuple**, two values kept together in a fixed order. Here the first value of the pair is a position and the second is a velocity, the distance moved each tick.

A `HashMap` is a dictionary: you store a value under a key, and later look it up by that key. The key can be almost any type with a sensible notion of equality, including a whole `Vec` of pairs.

```rust
use std::collections::HashMap;

let mut seen: HashMap<Vec<(i64, i64)>, u64> = HashMap::new();
```

This map uses a complete state, the list of pairs, as its key, and a `u64` time as its value. `new()` makes an empty map, and `mut` is there because adding an entry changes it. Rust turns each key into a number so lookup stays fast; the everyday word for that is hashing.

## Write the rules in order

A simulation is a rule applied to a state, again and again. The rule must be written exactly as the puzzle states it, including the order of events within one tick. Read the puzzle twice before typing. Most wrong simulations are not wrong code; they are code for a slightly different rule.

In our drill, one tick moves every particle by its own velocity, and the position wraps around a circular track. The lab writes that tick as a small function called `step`.

```rust title="src/bin/drill05.rs"
fn step(positions: &[(i64, i64)], track: i64) -> Vec<(i64, i64)> {
    positions
        .iter()
        .map(|&(position, velocity)| ((position + velocity).rem_euclid(track), velocity))
        .collect()
}
```

Read the signature first. `positions` is a borrowed slice, written `&[(i64, i64)]`: a view of a list of pairs that the function may read but not change. `track` is the length of the circle, and the arrow promises a new `Vec` of pairs.

The body uses an iterator chain, a compact way to say "for each item, make a new item." `.iter()` visits each pair in turn. `.map(...)` builds one new value for each visit. The part in the vertical bars is a **closure**, a small unnamed function; `&(position, velocity)` unpacks a borrowed pair into its two numbers. Each new pair holds the moved position and the same velocity. `.collect()` gathers all the new pairs into one `Vec`.

Here is the same function written with a `for` loop, where every step is on its own line.

```rust
fn step_plain(positions: &[(i64, i64)], track: i64) -> Vec<(i64, i64)> {
    let mut next: Vec<(i64, i64)> = Vec::new();
    for &(position, velocity) in positions {
        let moved = (position + velocity).rem_euclid(track);
        next.push((moved, velocity));
    }
    next
}
```

`next` starts empty, `push` adds one pair at the end, and the finished list is returned. The two versions do the same work. Choose the loop while the rule is still new to you.

The important detail is not the loop style. It is that `step` reads the old list and builds a new one. It never changes the list it was given. If we moved particles in place, the second particle might see the first particle's new position, and one tick would become a mixture of two ticks. This drill has no collisions, so both orders happen to agree. A puzzle with collisions does not.

Order also decides ties. When two particles land on the same position, this drill's checksum counts both. A different puzzle might say the first one wins, or that neither moves. That sentence belongs in your code as an explicit `if`, not as an accident of which particle the loop reached first.

Part 1 runs the rule for a small, given number of ticks.

```rust title="src/bin/drill05.rs"
pub fn part1(input: &Input) -> u64 {
    let mut positions = input.particles.clone();
    for _ in 0..input.steps {
        positions = step(&positions, input.track);
    }
    checksum(&positions)
}
```

`input.particles.clone()` copies the starting list so the input stays untouched; `input` is borrowed with `&Input`, which means we may read it but not change it. The range `0..input.steps` counts from zero up to but not including `steps`, so the loop body runs exactly `steps` times. The underscore in `for _` says the counter itself is not needed.

`checksum` adds up the positions.

```rust title="src/bin/drill05.rs"
fn checksum(positions: &[(i64, i64)]) -> u64 {
    positions.iter().map(|&(position, _)| position as u64).sum()
}
```

The underscore in the pattern ignores the velocity. `as u64` converts each position to the unsigned type, and `sum()` adds the whole list. The conversion is safe because a wrapped position is always zero or more.

## A state is a value

The state of this machine is the whole list of pairs. Not one particle's position: all of them, together, at one moment. The list at time zero is `input.particles`, and each call to `step` produces the list at the next time.

Keeping velocity beside position looks wasteful while the speed never changes. It is there because a state should carry everything the next tick needs. If a later rule slows a particle down, the pair still has the information. A state is a value you can copy, print, compare, and store.

That is why the list works as a `HashMap` key. Two states are equal when every pair is equal, and Rust already knows how to compare and hash a `Vec` of pairs.

Copying the state for a key is fine here: three pairs are a few dozen bytes, and the loop is short. A bigger puzzle might have a state of a thousand cells, or a whole grid. Copying that every tick could cost more than the simulation saves. There you would store a hash of the state, or pack the state into a single number, and accept a small chance of a false match. Here we choose the plain version.

The wrap-around uses a method called `rem_euclid`, which returns the non-negative remainder.

```rust
let track: i64 = 11;
let position: i64 = 9;
let velocity: i64 = 5;

let wrapped = (position + velocity).rem_euclid(track);
```

Nine plus five is fourteen, which is one full lap past eleven, so the particle sits at three. On a negative move the difference matters: `(0 - 2).rem_euclid(11)` is nine, the place two steps behind zero. The ordinary `%` operator would give minus two, which is not a place on the track at all. The next page looks closely at why the two operators disagree.

## Find the loop

The track has eleven places and the drill has three particles, so there are only 11 × 11 × 11 = 1,331 possible arrangements of positions. By tick 1,332 the simulation must have shown the same arrangement twice. This is the pigeonhole idea: more pigeons than holes means two pigeons share a hole. With finitely many states and an endless clock, a repeat is certain.

A repeat is also permanent. `step` looks only at the current state, so the same state always produces the same next state. Once time 40 and time 51 hold equal states, time 41 and time 52 hold equal states too, and the whole stretch from 40 to 51 repeats forever. The gap, eleven ticks in this made-up case, is the **cycle**.

To find the gap, `part2` remembers the first time it saw each state.

```rust title="src/bin/drill05.rs"
let mut states = vec![input.particles.clone()];
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

`vec![value]` makes a one-element `Vec`; here it holds the state at time zero. `states` keeps every state in order, so `states[time]` is the state at that time. `seen` maps a state to the first time it appeared, and time zero is entered before the loop starts.

Each pass computes `next` from the current state, then advances the clock. `if let Some(&first_time) = seen.get(&next)` asks the map for `next`. If the key is present, the pattern copies the stored `u64` out of the reference and runs the block. If not, the block is skipped and the new state is stored with its time.

Inside the block, `cycle` is the distance between the two visits. `index` is the time whose state matches the target time, and the function returns that state's checksum immediately.

The formula deserves a slow read. Suppose a state first appears at time 2 and the cycle is 5 ticks. Then times 2, 7, 12, 17, and so on all share a state. You want time 1,000,000. Subtract the start of the loop: 999,998 ticks after time 2. How far into the cycle is that? The remainder after dividing by 5 is 3. Add the start back: time 5.

```rust
let first_time: u64 = 2;
let cycle: u64 = 5;
let target: u64 = 1_000_000;

let index = first_time + (target - first_time) % cycle;
```

`index` is 5. Time 1,000,000 and time 5 differ by 999,995 ticks, which is 199,999 full cycles, so they hold the same state. Subtracting `first_time` first is what makes the arithmetic honest: the loop begins at time 2, not at time zero. Writing `target % cycle` would measure from the wrong starting line.

The `while` condition is a safety net. If no repeat arrives before the target, the loop stops at the target and falls through to the last line of the function, which returns the state at that time. For this drill the repeat always arrives early.

The memory cost is small: `states` holds at most 1,331 lists before a repeat is forced. On a bigger machine the cycle could be millions of states long, and this approach would need millions of copies. That is still usually less work than trillions of ticks.

## The drill

The example file is four short lines.

```text title="rust-aoc-lab/data/examples/05.txt"
track=11 steps=8 target=1000000000000
0 3
4 -2
7 1
```

The first line holds settings. The track is eleven places around, part 1 asks for eight ticks, and part 2 asks for the state at one trillion ticks. Each line after that is one particle: a starting position and a velocity. The first particle starts at zero and moves three places per tick.

Part 1 follows the rule exactly for eight ticks, then adds up the final positions. Part 2 asks for that same sum at tick 1,000,000,000,000, which no loop could reach in a human lifetime.

The code is in `rust-aoc-lab/src/bin/drill05.rs`. `parse` reads the header by looking for the prefixes `track=`, `steps=`, and `target=`, then reads the particle lines into pairs. `unwrap()` takes the number out of a parse result and panics if the text is not a number, which is a fine trade for an input we control. `main` prints both parts.

Run the two tests from inside `rust-aoc-lab/`.

```console
$ cargo test --bin drill05
   Compiling rust-aoc-lab v0.1.0 (...)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/bin/drill05.rs (target/debug/deps/drill05-...)

running 2 tests
test tests::drill05_part1_example ... ok
test tests::drill05_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both tests pass, which means the exact path and the skipping path agree with the recorded answers. Your compile time and the finished time will differ; the counts and the `ok` marks are the part that matters. The `(...)` stands for a folder path and a file suffix that change from machine to machine.

## If something went wrong

**`error[E0502]: cannot borrow 'positions' as mutable because it is also borrowed as immutable`**: you tried to `push` into the list while a `for` loop was walking it. Build a new list, push there, and assign it to `positions` after the loop ends.

**`index out of bounds: the len is 12 but the index is 1000000000000`**: the code tried `states[input.target as usize]` on a list that stopped at the first repeat. Either return the checksum from inside the repeat block, as `part2` does, or re-run the simulation forward to the target when no repeat was found.

**The number is right for part 1 but wrong for part 2**: check the loop bounds. `0..input.target` runs exactly `target` times and leaves you at time `target`. `0..input.target - 1` runs one tick fewer, and `0..=input.target` runs one tick more. The difference is a single tick, and a single tick changes positions.

**A loop that never ends**: the key in `seen` probably contains something that changes every tick, such as the time. Then no state ever matches, and the map grows until memory runs out. The key must hold only the repeating part of the state. A stopping condition like `while time != target` can also skip its target forever if `time` jumps by two. Use `<` when you mean "until we pass it."

**`attempt to add with overflow`**: a `u64` clock counted past its maximum. In a debug build Rust stops with that message; in a release build the number wraps to zero, and a loop that waits for a huge time may spin forever. Stop at the target instead of counting to it.

**A state with a negative position**: `%` keeps the sign of the left side, so `-2 % 11` is `-2`. Use `rem_euclid(11)` to land in `0` through `10`. If a negative number is cast to `usize` and used as an index, the message can look absurd, with an index near 18 quintillion; that is the negative value seen as an unsigned number, and the real fix is at the wrap step.

## Save this step

Save the practice you did in your own project. `git status` shows which files changed, `git add .` stages everything in the current folder, and `git commit` records the staged work with a message.

```console
$ git status
$ git add .
$ git commit -m "Practice the cycle-skipping simulation"
```

The commit hash and the branch name will differ on your machine. A commit is a saved state for your code, the same idea as a saved state for the machine you spent this page simulating.

<details>
<summary>Going deeper: machines, shortcuts, and rings</summary>

Many Advent of Code puzzles from 2019 hand you a small computer, and the year leaned on the idea so often that it is remembered as the machine year. The computer has a list of numbers as memory, a pointer called the instruction pointer, and a loop that reads one number, decides what it means, and changes memory. That shape is often called a fetch, decode, run loop, and it looks like this.

```rust
let mut memory: Vec<i64> = vec![1, 0, 0, 0, 99];
let mut ip: usize = 0;

while ip < memory.len() {
    let opcode = memory[ip];
    match opcode {
        1 => { /* add two numbers */ }
        2 => { /* multiply two numbers */ }
        99 => break,
        _ => panic!("unknown opcode"),
    }
    ip += 1;
}
```

`memory` is the machine's whole world, and the numbers in it are a made-up program that begins with an add and ends with a halt. The pointer `ip` is an index into that list. Each pass fetches the number there, matches it against the known opcodes, and moves the pointer. This sketch moves one slot at a time to keep the shape visible; a real machine also steps over the operands that follow each opcode. The full machine has more opcodes and more rules, but the skeleton does not change. A puzzle that runs such a machine for a huge number of steps can be handled with the same state-and-cycle idea: the machine's whole memory is the state.

Our drill has a shortcut. The velocity never changes, so a particle's position at time `t` is `(start + velocity * t).rem_euclid(track)`, with no loop at all. Try it on paper for the first particle at time eight. The shortcut is real, and it is worth seeing. It also stops working the moment a rule changes a velocity, which is why the general tool is the one to keep.

When a puzzle calls a list circular, you do not have to shift a `Vec` on every removal. Store the index of the next item instead: `let mut next: Vec<usize>`. To remove an item, point the item before it at the item after it. Two writes replace a whole shift, and the ring stays a ring.

</details>

The machine now runs exactly and jumps over its own repetition. Next: [do the math, keep the intervals](/part-6/do-the-math/).
