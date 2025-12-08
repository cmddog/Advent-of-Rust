pub fn part_1() -> u32 {
    let input = include_str!("inputs/2.txt");
    input.lines()
        .map(|line| {
            let dimensions = line.split("x")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let areas = Vec::from([
                2 * dimensions[0] * dimensions[1],
                2 * dimensions[1] * dimensions[2],
                2 * dimensions[2] * dimensions[0]
            ]);
            areas.iter().sum::<u32>() + areas.iter().min().unwrap() / 2
        })
        .sum::<u32>()
}

pub fn part_2() -> u32 {
    let input = include_str!("inputs/2.txt");
    input.lines()
        .map(|line| {
            let mut dimensions = line.split("x")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            dimensions.sort();
            let bow = dimensions[0] * dimensions[1] * dimensions[2];
            let ribbon = 2 * dimensions[0] + 2 * dimensions[1];
            bow + ribbon
        })
        .sum::<u32>()
}
