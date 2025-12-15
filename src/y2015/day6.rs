use regex::{Captures, Regex};

enum Operation {
    On,
    Off,
    Toggle,
}

fn parse_input() -> Vec<(Operation, (u32, u32), (u32, u32))> {
    let re =
        Regex::new(r"(?<operation>turn (?:on|off)|toggle) (?<c1>\d+,\d+) through (?<c2>\d+,\d+)")
            .unwrap();
    include_str!("inputs/6.txt")
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let (x1, y1) = caps["c1"].split_once(",").unwrap();
            let (x2, y2) = caps["c2"].split_once(",").unwrap();
            (
                match &caps["operation"] {
                    "turn on" => Operation::On,
                    "turn off" => Operation::Off,
                    "toggle" => Operation::Toggle,
                    _ => unreachable!("Invalid input"),
                },
                (x1.parse().unwrap(), y1.parse().unwrap()),
                (x2.parse().unwrap(), y2.parse().unwrap()),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part_1() -> usize {
    0
}

pub fn part_2() -> usize {
    0
}
