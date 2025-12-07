pub fn part_1() -> u32 {
    let input = include_str!("inputs/3.txt");
    input.lines()
        .map(|line| {
            let digits = line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let max = digits[..digits.len() - 1].iter().max().unwrap();
            let max_index = digits[..digits.len() - 1].iter().position(|&x| x == *max).unwrap();
            let next_max = digits[max_index + 1..].iter().max().unwrap();
            max * 10 + next_max
        })
        .sum::<u32>()
}

pub fn part_2() -> u64 {
    let input = include_str!("inputs/3.txt");
    input.lines()
        .map(|line| {
            let mut digits = line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let mut results = Vec::new();

            for remaining in (0..12).rev() {
                let end = digits.len() - remaining;
                let max = digits[..end].iter().max().unwrap();
                let max_index = digits[..end].iter().position(|&x| x == *max).unwrap();

                results.push(*max);
                digits.drain(..=max_index);
            }

            results.iter().fold(0u64, |acc, &digit| acc * 10 + digit as u64)
        })
        .sum::<u64>()
}