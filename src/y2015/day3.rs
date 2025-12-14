use std::collections::HashSet;

pub fn part_1() -> usize {
    let instructions = parse_instructions();

    track_positions(instructions.iter())
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2() -> usize {
    let instructions = parse_instructions();

    track_positions(instructions.iter().step_by(2))
        .chain(track_positions(instructions.iter().skip(1).step_by(2)))
        .collect::<HashSet<_>>()
        .len()
}

fn track_positions<'a>(
    moves: impl Iterator<Item = &'a  (i32, i32)>
) -> impl Iterator<Item = (i32, i32)> {
    moves.scan((0, 0), |(x, y), (dx, dy)| {
        *x += dx;
        *y += dy;
        Some((*x, *y))
    })
}

fn parse_instructions() -> Vec<(i32,i32)> {
    include_str!("inputs/3.txt")
        .chars()
        .map(|c| match c {
            '^' => (0,1),
            '>' => (1,0),
            'v' => (0, -1),
            '<' => (-1, 0),
            _ => unreachable!("Invalid input")
        })
        .collect::<Vec<_>>()
}
