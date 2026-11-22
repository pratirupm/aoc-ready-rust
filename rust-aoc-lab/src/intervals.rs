//! Inclusive intervals and the operations Advent of Code keeps asking for.

/// An inclusive interval, written `(start, end)`.
pub type Interval = (i64, i64);

/// Sort intervals and join the ones that touch or overlap.
pub fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return intervals;
    }

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
}

/// The number of integers covered by a list of already merged intervals.
pub fn total_length(intervals: &[Interval]) -> i64 {
    intervals.iter().map(|&(start, end)| end - start + 1).sum()
}

/// The gaps between merged intervals inside `[low, high]`.
pub fn gaps(intervals: &[Interval], low: i64, high: i64) -> Vec<Interval> {
    let mut holes = Vec::new();
    let mut cursor = low;

    for &(start, end) in intervals {
        if start > cursor {
            holes.push((cursor, start - 1));
        }
        cursor = cursor.max(end + 1);
        if cursor > high {
            break;
        }
    }

    if cursor <= high {
        holes.push((cursor, high));
    }

    holes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_intervals_merge() {
        assert_eq!(merge(vec![(1, 3), (4, 6)]), vec![(1, 6)]);
    }

    #[test]
    fn overlapping_intervals_merge() {
        assert_eq!(merge(vec![(1, 5), (3, 8)]), vec![(1, 8)]);
    }

    #[test]
    fn separate_intervals_stay_separate() {
        assert_eq!(merge(vec![(1, 2), (5, 6)]), vec![(1, 2), (5, 6)]);
    }

    #[test]
    fn total_length_counts_each_integer_once() {
        let merged = merge(vec![(10, 20), (15, 25), (30, 40)]);
        assert_eq!(total_length(&merged), 27);
    }

    #[test]
    fn gaps_are_the_holes_between() {
        let merged = merge(vec![(2, 3), (7, 8)]);
        assert_eq!(gaps(&merged, 0, 10), vec![(0, 1), (4, 6), (9, 10)]);
    }
}
