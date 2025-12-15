const KEY: &str = "iwrupvqb";

pub fn part_1() -> u64 {
    (1..)
        .find_map(|i| {
            let input = format!("{}{}", KEY, i);
            let digest = md5::compute(input);
            let hex = format!("{:x}", digest);

            hex.starts_with("00000").then_some(i)
        })
        .unwrap()
}

pub fn part_2() -> u64 {
    (1..)
        .find_map(|i| {
            let input = format!("{}{}", KEY, i);
            let digest = md5::compute(input);
            let hex = format!("{:x}", digest);

            hex.starts_with("000000").then_some(i)
        })
        .unwrap()
}
