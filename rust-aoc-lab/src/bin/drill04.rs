//! Drill 4: two small dynamic programs that count orders.

const EXAMPLE: &str = include_str!("../../data/examples/04.txt");

/// One target length per line.
pub fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

/// The number of sequences of 1, 2, and 3 that add up to `target`.
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

/// The number of sequences that add up to `target` with no two 3s in a row.
fn ways_no_double_three(target: u64) -> u64 {
    let target = target as usize;
    // dp[total][last_was_three]
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

    dp[target][0] + dp[target][1]
}

/// The sum of `ways_any` for every target.
pub fn part1(targets: &[u64]) -> u64 {
    targets.iter().map(|&target| ways_any(target)).sum()
}

/// The sum of `ways_no_double_three` for every target.
pub fn part2(targets: &[u64]) -> u64 {
    targets
        .iter()
        .map(|&target| ways_no_double_three(target))
        .sum()
}

fn main() {
    let targets = parse(EXAMPLE);
    println!("part1 = {}", part1(&targets));
    println!("part2 = {}", part2(&targets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drill04_part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 23_891_448_151);
    }

    #[test]
    fn drill04_part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 14_016_474_181);
    }
}
