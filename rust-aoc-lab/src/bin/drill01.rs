//! Drill 1: read a header, build a grid, count digits and crowded cells.

use rust_aoc_lab::grid::Grid;

const EXAMPLE: &str = include_str!("../../data/examples/01.txt");

/// Read the `width=… height=…` header and the grid that follows.
pub fn parse(input: &str) -> Grid {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header = lines.next().expect("a header line");

    let mut parts = header.split_whitespace();
    let width: usize = parts
        .next()
        .unwrap()
        .strip_prefix("width=")
        .unwrap()
        .parse()
        .unwrap();
    let height: usize = parts
        .next()
        .unwrap()
        .strip_prefix("height=")
        .unwrap()
        .parse()
        .unwrap();

    let body = lines.collect::<Vec<&str>>().join("\n");
    let grid = Grid::parse(&body);

    assert_eq!(grid.width, width, "the header width must match the rows");
    assert_eq!(grid.height, height, "the header height must match the rows");
    grid
}

/// The sum of every ASCII digit in the grid.
pub fn part1(grid: &Grid) -> u64 {
    grid.cells
        .iter()
        .filter(|byte| byte.is_ascii_digit())
        .map(|byte| (byte - b'0') as u64)
        .sum()
}

/// The number of `#` cells with exactly three `#` neighbours, diagonals included.
pub fn part2(grid: &Grid) -> u64 {
    let mut crowded: u64 = 0;

    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at(row, col) != b'#' {
                continue;
            }

            let mut neighbours = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    if grid.get(row as i32 + dr, col as i32 + dc) == Some(b'#') {
                        neighbours += 1;
                    }
                }
            }

            if neighbours == 3 {
                crowded += 1;
            }
        }
    }

    crowded
}

fn main() {
    let grid = parse(EXAMPLE);
    println!("part1 = {}", part1(&grid));
    println!("part2 = {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drill01_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 10);
    }

    #[test]
    fn drill01_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 5);
    }
}
