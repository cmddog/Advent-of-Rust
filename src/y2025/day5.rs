use std::ops::RangeInclusive;

pub fn part_1() -> usize {
    let (valid_ranges, available) = include_str!("inputs/5.txt").split_once("\n\n").unwrap();
    let valid_ranges = parse_ranges(valid_ranges).collect::<Vec<_>>();
    available.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|n| valid_ranges.iter().any(|range| range.contains(n)))
        .count()
}

pub fn part_2() -> u64 {
    let (valid_ranges, _) = include_str!("inputs/5.txt").split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(valid_ranges)
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r| *r.start());
    ranges.iter()
        .fold(Vec::new(), |mut acc: Vec<RangeInclusive<u64>>, range| {
            if acc.is_empty() {
                return vec![range.clone()];
            }
            let prev_end = acc.last().unwrap().end();
            let next_start = range.start();
            let next_end = range.end();

            if prev_end >= next_start {
                let last = acc.last_mut().unwrap();
                *last = *last.start()..=(*last.end()).max(*next_end);
            } else {
                acc.push(range.clone());
            }
            acc
        })
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>()
}

fn parse_ranges(ranges: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    ranges.lines()
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
            a..=b
        })
}
