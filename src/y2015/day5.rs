pub fn part_1() -> usize {
    include_str!("inputs/5.txt")
        .lines()
        .filter(|&line| {
            line.chars().filter(|c| "aeoiu".contains(*c)).count() >= 3
                && line.as_bytes().windows(2).any(|w| w[0] == w[1])
                && ["ab", "cd", "pq", "xy"].iter().all(|&s| !line.contains(s))
        })
        .count()
}

pub fn part_2() -> usize {
    include_str!("inputs/5.txt")
        .lines()
        .filter(|&line| {
            line.as_bytes().windows(2).enumerate().any(|(i1, w1)| {
                line.as_bytes()
                    .windows(2)
                    .enumerate()
                    .any(|(i2, w2)| w1 == w2 && i2 > i1 + 1)
            }) && line.as_bytes().windows(3).any(|w| w[0] == w[2])
        })
        .count()
}
