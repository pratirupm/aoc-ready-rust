---
title: "Walk the grid, search the graph"
description: "Breadth-first search, depth-first search, flood fill, and the tools that answer connection questions."
---

A grid is a graph wearing a costume. A graph is a set of points, called vertices, joined by lines, called edges. On a map, every cell is a vertex and every pair of neighbouring cells is an edge. The walls remove vertices and edges; the open cells keep them.

Once you see the graph, the questions change shape. Which cells are reachable from here? How many steps to the exit? Are two areas connected? How many separate areas are there? Each question has a small answer that does not depend on where the graph came from: a grid, a maze, or a list of roads.

Today we walk the map with a queue and with a stack. Then we meet two tools for connection questions that do not need a walk at all.

## What you need to know

A `VecDeque` is a queue, a line where items wait their turn, with two open ends; it is pronounced "deck". `push_back` adds to the back and `pop_front` removes from the front, both without moving the other items. A plain `Vec` can push at the back cheaply, but removing from the front shifts every remaining item, so a queue uses a `VecDeque`.

`pop_front` returns an `Option`, because the queue may be empty. That leads to a loop shape worth memorising:

```rust
use std::collections::VecDeque;

let mut queue: VecDeque<usize> = VecDeque::new();
queue.push_back(0);
while let Some(index) = queue.pop_front() {
    println!("{index}");
}
```

`while let` keeps looping as long as the pattern matches. Each round calls `pop_front`; `Some(index)` binds the value and runs the body, and `None` ends the loop. `Option` is the type that is either `Some(value)` or `None`; the previous day used it for positions outside the grid.

`usize::MAX` is the largest number a `usize` can hold, 18,446,744,073,709,551,615 on a 64-bit machine. It makes a useful marker for "not visited", because no real distance will ever reach it.

A `dist` array is a `Vec<usize>` with one entry per cell. Entry `i` holds the number of steps from the start to cell `i`, or `usize::MAX` when the cell has not been reached. A `Vec` also works as a stack: `push` adds to the end and `pop` removes the most recent item, which is the last-in, first-out order.

## Breadth-first search

Breadth-first search, usually shortened to BFS, visits a graph in rings. First the start, then every cell one step away, then every cell two steps away, and so on. It needs a queue, a distance array, and one rule about when to record a visit.

Here is the whole search from the drill:

```rust title="src/bin/drill02.rs"
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

`vec![usize::MAX; grid.cells.len()]` creates one entry per cell, all marked unvisited. The start gets distance zero and enters the queue. Every later cell enters only once, because the guard skips a wall or a cell whose distance is no longer `usize::MAX`.

`neighbors4(index)` returns the flat indices of the up-to-four cells beside `index`, and it already leaves out anything outside the grid. The body then rejects walls and repeats. When a cell passes both checks, its distance is one more than the distance of the cell that found it, and it joins the back of the queue. The index taken from the front always has a real distance, because only a cell with a recorded distance ever enters the queue.

Why is the first distance the shortest? Picture the queue as a line ordered by distance. The start, at distance zero, is alone at the front. Its open neighbours join at distance one. Only after every distance-one cell has been taken from the front do the distance-two cells behind them get their turn. A cell is written the first time any neighbour reaches it, and that neighbour is at the smallest distance possible. Any later path would need at least one more step.

The distance array doubles as the visited list. A cell with a real distance has been queued already, so the second check also prevents the same cell from entering twice and the queue from growing without end. On a map with no walls, the queue can hold a whole ring of cells at once; that is normal, and it is why the queue is a list rather than one saved position.

This version assumes every step costs the same. When steps have different costs, the rings stop lining up, and the next day replaces the queue with a priority queue, one that always hands back the smallest item.

## Flood fill is BFS without the counting

A flood fill asks a smaller question than BFS: which cells can be reached at all? The distances do not matter, only the set. The paint-bucket tool in an image editor is a flood fill: click one region and every connected pixel changes colour.

The drill's first part counts the reachable non-wall cells, leaving out the start itself. Inside `part1`, the search is the same call, and the counting is a pass over the result:

```rust title="src/bin/drill02.rs"
let dist = distances(grid, *start);
dist.iter()
    .enumerate()
    .filter(|&(index, &steps)| index != *start && steps != usize::MAX)
    .count() as u64
```

`enumerate()` pairs each entry with its position, so the closure receives `(index, &steps)`. The pattern `|&(index, &steps)|` unpacks that pair and copies both numbers out. The filter keeps a cell only when it is not the start and its distance is not the marker. `count()` adds one per kept cell.

`start` is borrowed from the destructured input, so it is a `&usize`; `*start` reads the number it points at. The ground floor's `*cell` example uses the same mark.

The result is the size of the connected region around the start, minus the start. Walls split the map into separate regions; a cell in another region keeps the marker and never enters the count.

The natural way to write a flood fill is recursion, a function that calls itself. Here is the shape, using the grid's `get` and `set`:

```rust
fn flood(grid: &mut Grid, row: i32, col: i32) {
    if grid.get(row, col) != Some(b'.') {
        return;
    }
    grid.set(row as usize, col as usize, b'#');
    flood(grid, row - 1, col);
    flood(grid, row + 1, col);
    flood(grid, row, col - 1);
    flood(grid, row, col + 1);
}
```

The function stops at a wall or at the edge, because `get` returns something other than `Some(b'.')`. It paints the current cell so the next call cannot return to it, then visits the four neighbours. That is correct, and it is short. This version accepts only `.` as open; the drill's search also accepts `S` and `E`, so its check is `cell != b'#'`.

It is also fragile. Every call waits for its four children to finish, so a long corridor or a large open field leaves one waiting call per cell, and each waiting call takes memory. The program's call stack has a fixed size, and when it runs out the process aborts with `fatal runtime error: stack overflow, aborting`. A queue or a stack that you control lives in the heap, the pool of memory for values that grow, and cannot overflow that way. The lab uses the queue form everywhere.

## Depth-first search without recursion

Depth-first search, or DFS, follows one path until it cannot continue, then backs up to the last fork. It uses a stack instead of a queue. A `Vec` is a stack if you add and remove at the end:

```rust
let mut seen = vec![false; grid.cells.len()];
let mut stack: Vec<usize> = Vec::new();
stack.push(start);

while let Some(index) = stack.pop() {
    if seen[index] {
        continue;
    }
    seen[index] = true;
    for next in grid.neighbors4(index) {
        if grid.cells[next] != b'#' {
            stack.push(next);
        }
    }
}
```

`stack.pop()` takes the most recently pushed index, so the walk dives into the newest neighbour and only tries the older ones after that path ends. `seen` is a list of `bool`, one per cell. The check at the top skips an index that was reached twice, and the assignment records the visit.

A cell may be pushed more than once before it is popped, because two neighbours can both find it. The `seen` check turns the duplicates into harmless no-ops. Marking cells when they are pushed is another valid style; it keeps the stack smaller but needs the check before every push.

DFS answers reachability, connected regions, and cycle questions; a cycle is a path that returns to where it started. It does not answer shortest distance, because the first path it finds may be long. Use BFS when distance matters and DFS when only the set of reachable cells matters.

Removing recursion also makes it possible to stop and resume the search. The whole state is the stack and the `seen` list, so a function can return between steps without losing its place.

## Two more graph tools

Some questions are about membership rather than paths: are these two cells in the same region, and in what order can these tasks run? Two small tools answer them without a walk.

A `Dsu`, short for disjoint-set union and also called union-find, keeps a collection of groups and merges them. Each item starts alone. `union(a, b)` joins the groups holding `a` and `b`, and `find(a)` returns the representative, a stand-in number for the group:

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
    ...
    true
}
```

`parent` is a list where each item points at a parent, and a root points at itself. `find` walks to the root and, along the way, points each item at its grandparent, which keeps the chains short. `union` finds both roots and links one to the other, returning `false` when they were already together. That return value is also how a cycle is detected while adding edges.

One use is counting islands. Give each cell a number, union every pair of adjacent floor cells, then count the distinct roots among floor cells. Another is Kruskal's algorithm for the cheapest set of roads that connects every place: sort the edges by cost and union each one, skipping an edge whose ends are already connected.

A topological sort puts tasks in an order where every task comes after its prerequisites. It uses a count called the indegree, the number of edges arriving at a vertex, and a queue of vertices with none left. Given `count` vertices and a list of `(from, to)` edges:

```rust
let mut graph: Vec<Vec<usize>> = vec![Vec::new(); count];
for &(from, to) in &edges {
    graph[from].push(to);
}
let mut indegree = vec![0usize; count];
for &(_, to) in &edges {
    indegree[to] += 1;
}
let mut queue: VecDeque<usize> = VecDeque::new();
for node in 0..count {
    if indegree[node] == 0 {
        queue.push_back(node);
    }
}
let mut order: Vec<usize> = Vec::new();
while let Some(node) = queue.pop_front() {
    order.push(node);
    for &next in &graph[node] {
        indegree[next] -= 1;
        if indegree[next] == 0 {
            queue.push_back(next);
        }
    }
}
```

Every vertex with no prerequisites starts in the queue. Taking one out appends it to the order and removes its outgoing edges by lowering the indegree of each vertex the edge points to. A successor that reaches zero has all its prerequisites done and joins the queue. If the final order is shorter than the number of vertices, some vertices never reached zero, which means the edges contain a cycle.

## The drill

Open `data/examples/02.txt`. It is a maze with a start, an exit, walls, and floor:

```text
#########
#S..#...#
#.#.#.#.#
#.#...#.#
#.#####.#
#.......#
####E####
```

`S` is the start, `E` is the exit, `#` is a wall, and `.` is open floor. Both `S` and `E` are walkable. `Grid::parse` keeps every line as a row, and `parse` finds the two special cells with `find`:

```rust title="src/bin/drill02.rs"
pub type Input = (Grid, usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').expect("a start cell");
    let exit = grid.find(b'E').expect("an exit cell");
    (grid, start, exit)
}
```

The return type is a tuple, a fixed group of values in parentheses, holding three things: the grid, the flat index of `S`, and the flat index of `E`. `Input` is a type alias for that tuple, so the function signature stays short and both parts can name the same type. `find` returns an `Option`, and `expect` unwraps it with a message that says what was missing. Flat indices are used instead of row and column pairs because the distance array is indexed the same way.

Part 1 runs BFS from `S` and counts the reachable cells that are not the start and not the marker. `E` is reachable, so it counts as an ordinary cell here. Part 2 runs the same search and reads the distance at the exit; if the exit is walled off, the marker survives and the part returns zero.

Run the tests:

```console
$ cargo test --bin drill02
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/bin/drill02.rs (target/debug/deps/drill02-...)

running 2 tests
test tests::drill02_part1_example ... ok
test tests::drill02_part2_example ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both lines say `ok`. Your timing and the path under `target` will differ, and the two test lines may print in either order. The code is in `src/bin/drill02.rs`; the distance function is the piece to read until the queue order feels natural.

## If something went wrong

**`error[E0502]: cannot borrow grid as mutable because it is also borrowed as immutable`** appears when a loop reads the cells and the body calls `set` on the same grid. The read-only loop holds a borrow for its whole run, and Rust will not hand out a second, mutable borrow at the same time:

```console
$ cargo build
error[E0502]: cannot borrow `grid` as mutable because it is also borrowed as immutable
...
   = help: consider using an index-based loop instead, or collecting modifications into a separate collection
```

Collect the positions to change in a `Vec`, let the read-only loop end, then change them in a second loop. A loop over `0..grid.cells.len()` also works, because reading a cell copies the byte and leaves no borrow behind.

**`index out of bounds: the len is 48 but the index is 48`** means a hand-written neighbour index fell outside the grid. `neighbors4` only returns safe indices, but `grid.idx(row + 1, col)` does not check anything. Use `get` for positions that might be outside, or compare against `width` and `height` before calling `idx`.

**`thread 'main' has overflowed its stack`**, followed by `fatal runtime error: stack overflow, aborting`, means recursive flood fill went too deep. Replace the recursive calls with a `Vec` stack or a `VecDeque` queue. The search stays the same; only the place that remembers pending cells changes.

**`attempt to subtract with overflow`** means a `usize` was decremented at zero, usually `row - 1` or `col - 1` on the top or left edge. Keep the position in `i32` while adding offsets and let `get` reject the negative result. In release builds the value wraps to a huge number instead of panicking, so the mistake can survive until an index check fails.

## Save this step

The search works and the tests agree. Save the state before trying a change:

```console
$ git status
$ git add -A
$ git commit -m "Walk the grid with breadth-first search"
```

`git status` shows which files changed, `git add -A` stages every change in the project, and `git commit` records the snapshot with your message. Branch names and short hashes are local to your machine, so they will not match another reader's. When `status` reports `nothing to commit, working tree clean`, there is nothing new to save.

<details>
<summary>Going deeper: many sources, two costs, and a tighter visited list</summary>

**Multi-source BFS** starts with several cells in the queue at distance zero instead of one. The rings then measure the distance to the nearest source, which answers questions such as "how far is every cell from the closest exit?"

**0-1 BFS** handles edges that cost either zero or one. It keeps a `VecDeque`, pushes a zero-cost neighbour to the front with `push_front`, and a one-cost neighbour to the back with `push_back`. The queue stays ordered by cost, and the search still runs in time proportional to the cells and edges.

**A visited bitset** stores one bit per cell in a `Vec<u64>`. Cell `i` lives in word `i / 64`, one 64-bit number, at bit `1u64 << (i % 64)`. Testing and setting are a shift and a mask. A `Vec<bool>` is already packed this way behind the scenes; the hand-rolled version makes the packing visible and copies in whole words.

**A cycle check for topological sort** is one comparison: if `order.len()` is less than the number of vertices, the remaining vertices form at least one cycle, and no valid order exists.

</details>

Next: [weigh the cost and order the queue](/part-3/weigh-the-cost/).
