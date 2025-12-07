pub fn part_1() -> i32 {
    let input = include_str!("inputs/1.txt");
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum::<i32>()
}

pub fn part_2() -> usize {
    let input = include_str!("inputs/1.txt");
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .position(|x| x == -1)
        .expect("Input not found") + 1
}

// readFile "i" >>= (print . sum . map f)
// f = \case
//  '(' -> 1
//  ')' -> -1