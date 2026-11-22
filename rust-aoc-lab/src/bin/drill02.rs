//! Drill 2: flood fill and the shortest path on an unweighted grid.

use std::collections::VecDeque;

use rust_aoc_lab::grid::Grid;

const EXAMPLE: &str = include_str!("../../data/examples/02.txt");

/// The grid, the start index, and the exit index.
pub type Input = (Grid, usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').expect("a start cell");
    let exit = grid.find(b'E').expect("an exit cell");
    (grid, start, exit)
}

/// The number of steps from `start` to every reachable cell.
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

/// The number of non-wall cells reachable from the start, not counting the start.
pub fn part1(input: &Input) -> u64 {
    let (grid, start, _exit) = input;
    let dist = distances(grid, *start);
    dist.iter()
        .enumerate()
        .filter(|&(index, &steps)| index != *start && steps != usize::MAX)
        .count() as u64
}

/// The shortest number of steps to the exit, or zero when it cannot be reached.
pub fn part2(input: &Input) -> u64 {
    let (grid, start, exit) = input;
    let dist = distances(grid, *start);
    if dist[*exit] == usize::MAX {
        0
    } else {
        dist[*exit] as u64
    }
}

fn main() {
    let input = parse(EXAMPLE);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drill02_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 24);
    }

    #[test]
    fn drill02_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 8);
    }
}
