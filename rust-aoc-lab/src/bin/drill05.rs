//! Drill 5: follow a small simulation, then jump over its cycle.

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("../../data/examples/05.txt");

#[derive(Debug, Clone)]
pub struct Input {
    pub track: i64,
    pub steps: u64,
    pub target: u64,
    pub particles: Vec<(i64, i64)>,
}

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header = lines.next().expect("a header line");

    let mut track = 0;
    let mut steps = 0;
    let mut target = 0;

    for word in header.split_whitespace() {
        if let Some(value) = word.strip_prefix("track=") {
            track = value.parse().unwrap();
        } else if let Some(value) = word.strip_prefix("steps=") {
            steps = value.parse().unwrap();
        } else if let Some(value) = word.strip_prefix("target=") {
            target = value.parse().unwrap();
        }
    }

    let particles = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let position: i64 = parts.next().unwrap().parse().unwrap();
            let velocity: i64 = parts.next().unwrap().parse().unwrap();
            (position, velocity)
        })
        .collect();

    Input {
        track,
        steps,
        target,
        particles,
    }
}

fn step(positions: &[(i64, i64)], track: i64) -> Vec<(i64, i64)> {
    positions
        .iter()
        .map(|&(position, velocity)| ((position + velocity).rem_euclid(track), velocity))
        .collect()
}

fn checksum(positions: &[(i64, i64)]) -> u64 {
    positions.iter().map(|&(position, _)| position as u64).sum()
}

/// The checksum after the small number of steps in the header.
pub fn part1(input: &Input) -> u64 {
    let mut positions = input.particles.clone();
    for _ in 0..input.steps {
        positions = step(&positions, input.track);
    }
    checksum(&positions)
}

/// The checksum at the huge target time, found by skipping a repeated state.
pub fn part2(input: &Input) -> u64 {
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

    checksum(&states[input.target as usize])
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
    fn drill05_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 16);
    }

    #[test]
    fn drill05_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 13);
    }
}
