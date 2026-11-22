//! Drill 3: Dijkstra on a grid, then Dijkstra with a direction in the state.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use rust_aoc_lab::grid::Grid;

const EXAMPLE: &str = include_str!("../../data/examples/03.txt");
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub type Input = (Grid, usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').expect("a start cell");
    let exit = grid.find(b'E').expect("an exit cell");
    (grid, start, exit)
}

/// The price of entering a cell. A digit is its value; everything else costs one.
fn entry_cost(byte: u8) -> u64 {
    if byte.is_ascii_digit() {
        (byte - b'0') as u64
    } else {
        1
    }
}

/// The cheapest path when only the entered cell has a price.
pub fn part1(input: &Input) -> u64 {
    let (grid, start, exit) = input;
    let mut dist = vec![u64::MAX; grid.cells.len()];
    let mut heap = BinaryHeap::new();

    dist[*start] = 0;
    heap.push(Reverse((0u64, *start)));

    while let Some(Reverse((cost, index))) = heap.pop() {
        if cost > dist[index] {
            continue;
        }
        if index == *exit {
            return cost;
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

    0
}

/// The cheapest path when a change of direction adds three to the bill.
pub fn part2(input: &Input) -> u64 {
    let (grid, start, exit) = input;
    let cells = grid.cells.len();
    let mut dist = vec![vec![u64::MAX; cells]; 5];
    let mut heap = BinaryHeap::new();

    // Direction 4 means "no direction yet".
    dist[4][*start] = 0;
    heap.push(Reverse((0u64, *start, 4usize)));

    while let Some(Reverse((cost, index, direction))) = heap.pop() {
        if cost > dist[direction][index] {
            continue;
        }
        if index == *exit {
            return cost;
        }

        for next in grid.neighbors4(index) {
            if grid.cells[next] == b'#' {
                continue;
            }

            let (row, col) = grid.row_col(index);
            let (next_row, next_col) = grid.row_col(next);
            let step = (next_row as i32 - row as i32, next_col as i32 - col as i32);
            let next_direction = DIRS.iter().position(|&dir| dir == step).unwrap();

            let turn = if direction != 4 && direction != next_direction {
                3
            } else {
                0
            };
            let next_cost = cost + entry_cost(grid.cells[next]) + turn;

            if next_cost < dist[next_direction][next] {
                dist[next_direction][next] = next_cost;
                heap.push(Reverse((next_cost, next, next_direction)));
            }
        }
    }

    0
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
    fn drill03_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 8);
    }

    #[test]
    fn drill03_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 12);
    }
}
