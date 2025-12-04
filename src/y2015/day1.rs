pub fn part_1() -> i32 {
    let input = include_str!("day1.txt");
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn part_2() -> usize {
    let input = include_str!("day1.txt");
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .position(|x| x == -1)
        .expect("Input not found") + 1
}

// readFile "i" >>= (print . sum . map f)
// f str = case str of
//  '(' -> 1
//  ')' -> -1