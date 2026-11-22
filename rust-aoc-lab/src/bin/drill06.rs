//! Drill 6: merge inclusive intervals, then align independent cycles.

use rust_aoc_lab::intervals;
use rust_aoc_lab::num;

const EXAMPLE: &str = include_str!("../../data/examples/06.txt");

#[derive(Debug, Default)]
pub struct Input {
    pub intervals: Vec<intervals::Interval>,
    pub cycles: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    let mut result = Input::default();
    let mut section = "";

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.ends_with(':') {
            section = line;
            continue;
        }

        match section {
            "intervals:" => {
                let (start, end) = line.split_once('-').unwrap();
                result
                    .intervals
                    .push((start.parse().unwrap(), end.parse().unwrap()));
            }
            "cycles:" => result.cycles.push(line.parse().unwrap()),
            _ => {}
        }
    }

    result
}

/// The number of integers covered by all intervals together.
pub fn part1(input: &Input) -> u64 {
    let merged = intervals::merge(input.intervals.clone());
    intervals::total_length(&merged) as u64
}

/// The first positive time at which every cycle is back at its start.
pub fn part2(input: &Input) -> u64 {
    num::lcm_many(&input.cycles)
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
    fn drill06_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 28);
    }

    #[test]
    fn drill06_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 60);
    }
}
