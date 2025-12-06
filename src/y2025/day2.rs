pub fn part_1() -> u64 {
    let input = include_str!("day2.txt");
    input
        .split(",")
        .flat_map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();
            start..=end
        })
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
