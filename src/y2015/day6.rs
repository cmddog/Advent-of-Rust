use regex::Regex;

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
    let mut lamps = vec![[false; 1000]; 1000];
    let instructions = parse_input();
    for (operation, (x1, y1), (x2, y2)) in instructions {
        for x in x1..=x2 {
            for y in y1..=y2 {
                lamps[x as usize][y as usize] = match operation {
                    Operation::On => true,
                    Operation::Off => false,
                    Operation::Toggle => !lamps[x as usize][y as usize],
                }
            }
        }
    }
    lamps.iter().flatten().filter(|&&b| b).count()
}

pub fn part_2() -> usize {
    let mut lamps: Vec<[usize; 1000]> = vec![[0; 1000]; 1000];
    let instructions = parse_input();
    for (operation, (x1, y1), (x2, y2)) in instructions {
        for x in x1..=x2 {
            for y in y1..=y2 {
                lamps[x as usize][y as usize] = match operation {
                    Operation::On => lamps[x as usize][y as usize] + 1,
                    Operation::Off => lamps[x as usize][y as usize].saturating_sub(1),
                    Operation::Toggle => lamps[x as usize][y as usize] + 2,
                }
            }
        }
    }
    lamps.iter().flatten().sum()
}
