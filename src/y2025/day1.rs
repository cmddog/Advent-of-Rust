use std::iter;

pub fn part_1() -> usize {
    let input = include_str!("inputs/1.txt");
    input
        .lines()
        .map(|s| s.replace("L", "-").replace("R", "").parse::<i32>().unwrap())
        .scan(50, |acc, x| {
            *acc = (*acc + x) % 100;
            Some(*acc)
        })
        .filter(|&n| n == 0)
        .count()
}

pub fn part_2() -> usize {
    let input = include_str!("inputs/1.txt");
    input
        .lines()
        .map(|s| s.replace("L", "-").replace("R", "").parse::<i32>().unwrap())
        .flat_map(|x| iter::repeat_n(x.signum(), x.abs() as usize))
        .scan(50, |acc, x| {
            *acc = (*acc + x) % 100;
            Some(*acc)
        })
        .filter(|&n| n == 0)
        .count()
}
