pub fn part_1() -> u32 {
    let input = include_str!("inputs/4s.txt");
    let input = format_input(input);
    
    0
}

pub fn part_2() -> String {
    String::from("Not yet implemented")
}

fn format_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars()
            .map(|c| {
                match c {
                    '@' => 1,
                    '.' => 0,
                    _ => unreachable!()
                }
            })
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}