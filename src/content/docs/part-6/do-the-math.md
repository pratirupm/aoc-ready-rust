---
title: "Do the math, keep the intervals"
description: "Integer widths, modular arithmetic, merging intervals, and compressed coordinates."
---

Some Advent of Code answers do not come from a search. No path to walk, no state to simulate, no table to fill. The answer is a piece of arithmetic: how many numbers a set of ranges covers, when three cycles line up, how large a shape is on a grid too big to draw.

Those puzzles are won or lost before the first line of real logic. A number is stored in a box of fixed width, and a value that does not fit the box wraps or stops the program. A remainder with the wrong sign sends a coordinate to the wrong place. An interval counted with the wrong endpoint is off by one, quietly, and the test disagrees much later.

The arithmetic we need is small: pick a width, understand the remainder, merge ranges, shrink a big grid, and find a common multiple. Each idea is one short function. Together they cover a surprising number of the later puzzles.

We will learn the types first, then the four tools, then run a drill that merges intervals and aligns cycles.

## What you need to know

Rust has several integer types, and the name says the width. A wider type holds a wider range and takes more memory. The common ones are these.

| Type | Holds | Rough range |
| --- | --- | --- |
| `i32` | whole numbers, positive or negative | −2.1 billion to 2.1 billion |
| `i64` | whole numbers, positive or negative | −9.2 quintillion to 9.2 quintillion |
| `u64` | whole numbers, zero or positive | 0 to 18.4 quintillion |
| `i128` | whole numbers, positive or negative | about −1.7 × 10³⁸ to 1.7 × 10³⁸ |

The `i` types are signed, which means they have a minus sign. The `u` types are unsigned, which means they start at zero. `usize` is a special unsigned type used for lengths and indexes; it is as wide as the machine's memory addresses, usually 64 bits.

Two operators look alike and are not. `%` is the remainder, and its sign follows the left-hand number. `rem_euclid` is the non-negative remainder, always from zero up to one less than the right-hand number.

```rust
let a: i64 = -3;
let b: i64 = 7;

let remainder = a % b;
let wrapped = a.rem_euclid(b);
```

The first value is −3 and the second is 4. Both are correct answers to different questions. We will use each one on purpose.

A range is written with two dots. `0..8` counts 0 through 7 and stops before 8. `0..=8` counts 0 through 8 and includes it. The `=` is the difference between eight items and nine. Get in the habit of reading `..=` as "through."

A tuple key is a pair used as a dictionary key: `HashMap<(i32, i32), u32>` maps a row and column to a count. Rust compares the pair field by field, so `(3, 4)` and `(4, 3)` are different keys.

## Pick a width and stay there

Every drill in the lab returns `u64`, because an answer is a count or a total and counts do not go negative. Inside the function, use the type that fits the work. Coordinates are often `i64`, because a step can move left or up into negative numbers. A sum is often `u64`. A product of two large values may need `i128`.

The width is a promise, and Rust enforces it. An `i32` cannot hold ten billion, so it does not pretend to.

```rust
let width: i32 = 100_000;
let height: i32 = 100_000;
let area: i64 = width as i64 * height as i64;
```

The underscores are separators, there to help your eye count zeros. `as i64` converts each value before the multiplication. The conversion matters: 100,000 times 100,000 is 10,000,000,000, which is past the `i32` limit of about 2.1 billion. Writing `width * height` and converting the result afterwards does not help, because the multiplication happens first, inside `i32`, and the program stops with an overflow.

This is not a rare corner. A million distances, each up to a million, sum to 10¹², which fits `u64` without strain. Multiply two numbers near the `u64` limit and the true product needs about 128 bits, which is what `i128` is for. A range of numbers in the billions, as in [Day 5 of 2023](https://adventofcode.com/2023/day/5), can pass the `i32` limit when you add an endpoint to a length, even though each number looked fine on its own.

The safe habit is to widen before the arithmetic, not after. Choose the type by asking what the largest intermediate value could be, not what the final answer looks like. Then convert once, at the end, to the return type.

## Modulo is not remainder

Modulo and remainder agree on positive numbers, which is why the difference hides for so long. Seven divided by three leaves one either way. Put a minus sign on the left and they part company.

```rust
let a: i64 = -3;
let b: i64 = 7;

println!("{}", a % b);
println!("{}", a.rem_euclid(b));
```

The first line prints −3 and the second prints 4. `%` keeps the sign of the dividend, the number being divided. `rem_euclid` adds or subtracts copies of the divisor until the answer sits in the range from zero to one less than the divisor, which is what "wrap around" means in everyday language.

That is exactly the job when a position runs off the end of a row, or a direction passes north and should come back around.

```rust
let north: i64 = 0;
let west = (north - 1).rem_euclid(4);
let east = (north + 5).rem_euclid(4);
```

`west` is 3 and `east` is 1, so the four directions behave like a circle with no negative index. With `%`, `west` would be −1, and using it as a `Vec` index would stop the program or read the wrong cell.

A negative remainder is dangerous because it does not look wrong. A position of −1 on a track of length 11 may still add up to a plausible-looking checksum. Nothing panics; the answer is false. When a value can go below zero, reach for `rem_euclid` and say why in a comment.

## Intervals

An interval is a pair of endpoints, and in this book both ends are included. `(3, 5)` covers 3, 4, and 5, three integers. The lab gives the pair a name, `Interval`, and treats it as `(i64, i64)`.

Several intervals can cover the same ground. `(1, 5)` and `(3, 8)` overlap, and their union is everything from 1 to 8. The job is to join them, then count the union without counting any integer twice. Sorting first makes that a single pass. Here is the core of `merge`, after the check for an empty list.

```rust title="src/intervals.rs"
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
```

`sort()` orders the pairs by their start, then by their end, so the next interval always begins at or after the current one. The first pair becomes the interval under construction. `&intervals[1..]` is a view of the list from the second item onward.

For each later pair, the test is `next_start <= end + 1`. If the next start is at or before the first free integer after `end`, the two touch or overlap, and the current interval grows. `end.max(next_end)` keeps the farther end, which handles a pair that is completely inside the current one. If there is a real gap, the finished interval is pushed and a new one begins. The last interval is pushed after the loop, because no later pair arrives to close it.

The `+ 1` is the whole subtlety. Since `end` is covered, the first number not covered is `end + 1`. An interval that starts there continues the run with no hole. Intervals `(1, 3)` and `(4, 6)` cover 1 through 6 with nothing missing, so they merge into `(1, 6)`. Drop the `+ 1` and they stay apart, which is still a correct list of intervals but a misleading picture of the covered ground.

Once the list is merged, counting is short.

```rust title="src/intervals.rs"
pub fn total_length(intervals: &[Interval]) -> i64 {
    intervals.iter().map(|&(start, end)| end - start + 1).sum()
}
```

Each merged interval contributes `end - start + 1`, not `end - start`, because both endpoints belong to it. The sum counts every covered integer exactly once. `intervals::gaps` uses the same merged list in the other direction: give it a low and a high, and it returns the holes between the intervals, the parts nobody covers.

## Compress the coordinates

A grid of 100,000 rows and 100,000 columns has 10,000,000,000 cells. At one byte per cell that is ten gigabytes, before any bookkeeping. A puzzle can describe such a grid with a few thousand lines and still expect an answer in under a second. The grid is a decoy; the interesting places are the listed ones.

Coordinate compression keeps only the rows and columns that can change the answer. Sort the listed coordinates, remove duplicates, and replace each real coordinate with its rank in that short list.

```rust
let mut xs: Vec<i64> = vec![0, 50, 50, 900];
xs.sort();
xs.dedup();
```

After the two calls, `xs` is `[0, 50, 900]`. `sort()` puts equal values side by side, and `dedup()` removes a value when it equals the one before it. A grid that spanned 901 columns now has three coordinate values to track.

The catch is that one compressed column is not one real column. `0` and `50` mark the edges of a strip that is 50 wide, and any area or count has to be multiplied by that real width. Compression shrinks the bookkeeping, not the numbers. It is worth the extra care when the alternative is ten gigabytes.

Two puzzles from recent years lean on this habit. [Day 9 of 2025](https://adventofcode.com/2025/day/9) asks about shapes on a very large plane where only the listed points matter, and [Day 5 of 2023](https://adventofcode.com/2023/day/5) keeps its large values in ranges rather than in a grid; in both, the answer lives at the endpoints, not at every integer in between.

## GCD and LCM

The greatest common divisor of two numbers is the largest number that divides both. Euclid's method finds it by repeated remainder: divide, keep the remainder, swap, repeat until the remainder is zero.

```rust title="src/num.rs"
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
```

For 48 and 18, the remainders run 12, then 6, then 0, and the answer is 6. The parameters are `mut` because the loop changes them. The function returns `a` when `b` reaches zero, and that leftover `a` is the divisor both numbers share.

The least common multiple is the smallest number that both divide into. It follows from the gcd: divide one number by the gcd, then multiply by the other.

```rust title="src/num.rs"
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}
```

The zero check avoids a division by zero. The order matters: `a / gcd(a, b) * b` divides before it multiplies, so the intermediate value stays as small as the final answer. Multiplying first can overflow even when the answer fits.

For 4 and 6, the gcd is 2, so the lcm is 4 / 2 × 6 = 12. Check it: multiples of 4 are 4, 8, 12; multiples of 6 are 6, 12. The first shared value is 12.

That is why cycles align at the lcm. If one light blinks every 4 ticks and another every 6, both blink together at tick 12, then 24, then 36. Any common time must be a multiple of each period, so the first one is the least common multiple.

Lists have more than two periods, and the lab folds over them.

```rust title="src/num.rs"
pub fn lcm_many(values: &[u64]) -> u64 {
    values.iter().fold(1, |total, &value| lcm(total, value))
}
```

`fold` carries a running value, starting at 1, and combines it with each item. After the first item the total is that item's lcm with 1; after the second it is the lcm of both; and so on. The result is the first time every cycle is back at its start.

## The drill

The example file has two sections.

```text title="rust-aoc-lab/data/examples/06.txt"
intervals:
10-20
15-25
30-40
50-50
cycles:
4
6
10
```

The `intervals:` line starts a list of ranges, one per line, with the start and end joined by a dash. The `cycles:` line starts a list of periods, one number per line. `parse` reads the section header, then sends each line to the right list.

Part 1 merges the four intervals and returns the number of integers they cover together. The first two overlap, so they become one interval; the last two sit apart and stay separate. The answer counts every covered integer once, which is why the merge has to happen before the addition.

Part 2 returns the first positive time at which every cycle is back at its start. The three periods in the file are 4, 6, and 10, so the answer is their least common multiple, taken across the whole list.

The code is in `rust-aoc-lab/src/bin/drill06.rs`. `part1` clones the interval list, calls `intervals::merge`, then `intervals::total_length`, and converts the `i64` total to `u64` for the return type. `part2` hands the cycle list to `num::lcm_many`. `main` prints both.

Run the tests from inside `rust-aoc-lab/`.

```console
$ cargo test --bin drill06
   Compiling rust-aoc-lab v0.1.0 (...)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/bin/drill06.rs (target/debug/deps/drill06-...)

running 2 tests
test tests::drill06_part1_example ... ok
test tests::drill06_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both tests pass, so the merge and the common multiple match the recorded answers. Compile times differ from machine to machine, and the `(...)` hides a folder path and a file suffix that change too.

## If something went wrong

**`attempt to multiply with overflow`**: two `i32` values were multiplied and the true product did not fit. Convert to `i64` or `i128` before the multiplication, not after. The same message can come from an addition that passes the type's limit.

**A negative remainder gives a strange answer**: `%` keeps the sign of the left side, so a wrapped position can be −1 instead of 10. Replace it with `rem_euclid`, and check any place where a coordinate is allowed to go below zero.

**The interval count is one short per interval**: `end - start` leaves out the last integer. Use `end - start + 1`, and keep the touch test as `next_start <= end + 1`. Both `+ 1`s come from the same fact: the end is included.

**`memory allocation of 10000000000 bytes failed`**: the program tried to build the giant grid for real. The process aborts, and no amount of waiting helps. Compress the coordinates first, or work with the rectangles and ranges directly instead of their cells.

**`attempt to divide by zero`**: `lcm` was handed a zero and reached `a / gcd(a, b)` with both values zero. Keep the zero check before the division; without it, a zero in the second position can also turn a real answer into zero without any error at all.

## Save this step

Save your interval helpers and your number helpers in your own project. `git status` shows the changed files, `git add .` stages them, and `git commit` records the work with a message.

```console
$ git status
$ git add .
$ git commit -m "Add interval and number helpers"
```

The hash and branch name will differ on your machine. A commit is a saved state for your code, and it is cheap to make before trying a risky change.

<details>
<summary>Going deeper: four tools with long names</summary>

The **Chinese remainder theorem** says that remainders modulo several numbers with no shared factor pin a value down modulo their product, and it shows how to rebuild that value. It is the many-modulus version of the cycle alignment above, and it appears whenever a puzzle gives you the same unknown under different clocks.

The **shoelace formula** computes the area of a polygon from the coordinates of its corners, by adding cross products around the boundary. **Pick's theorem** relates that area to the lattice points inside and on the edge: area equals interior plus half the boundary minus one. Given any two of the three counts, the third follows.

**Gaussian elimination** solves several linear equations by removing one variable at a time, then substituting back. It is the systematic version of the elimination you may have done in school, and it costs time proportional to the cube of the number of variables, which is fine for a handful and hopeless for a million.

A **3D rotation** is a 3 × 3 table of numbers applied to a point's three coordinates. Composing rotations means multiplying the tables, and wrapping an angle uses the same `rem_euclid` idea with 360 as the modulus.

</details>

The arithmetic is in place, and the big grids have shrunk to their interesting lines. Next: [put it together](/part-7/put-it-together/).
