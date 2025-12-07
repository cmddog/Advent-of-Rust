pub fn part_1() -> u64 {
    let input = include_str!("inputs/2.txt");
    parse_ranges(input)
        .filter(|&n| {
            let n = n.to_string();
            let len = n.len();
            if len % 2 != 0 {
                return false;
            }
            let (a,b) = n.split_at(len / 2);
            a == b
        })
        .sum()
}

pub fn part_2() -> u64 {
    let input = include_str!("inputs/2.txt");
    parse_ranges(input)
        .filter(|&n| {
            let n = n.to_string();
            let len = n.len();

            (1..=len/2)
                .filter(|&pattern_len| len % pattern_len == 0)
                .any(|pattern_len| {
                    let pattern = &n[..pattern_len];
                    n.as_bytes()
                        .chunks(pattern_len)
                        .all(|chunk| chunk == pattern.as_bytes())
                })
        })
        .sum()
}

fn parse_ranges(input: &str) -> impl Iterator<Item = u64> {
    input.split(",")
        .flat_map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();
            start..=end
        })
}
