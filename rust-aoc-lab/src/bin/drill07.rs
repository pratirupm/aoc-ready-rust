//! Drill 7: one measured walk, then count every shortest walk.

use std::collections::VecDeque;

use rust_aoc_lab::grid::Grid;

const EXAMPLE: &str = include_str!("../../data/examples/07.txt");
const MOD: u64 = 1_000_000_007;

pub type Input = (Grid, usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').expect("a start cell");
    let exit = grid.find(b'E').expect("an exit cell");
    (grid, start, exit)
}

fn distances(grid: &Grid, start: usize) -> Vec<usize> {
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

/// The length of the shortest walk from start to exit.
pub fn part1(input: &Input) -> u64 {
    let (grid, start, exit) = input;
    let dist = distances(grid, *start);
    if dist[*exit] == usize::MAX {
        0
    } else {
        dist[*exit] as u64
    }
}

/// The number of different shortest walks from start to exit.
pub fn part2(input: &Input) -> u64 {
    let (grid, start, exit) = input;
    let dist = distances(grid, *start);
    if dist[*exit] == usize::MAX {
        return 0;
    }

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
    fn drill07_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 4);
    }

    #[test]
    fn drill07_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2);
    }
}
