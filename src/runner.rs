use std::time::Instant;
use std::fmt::Display;

pub fn run_day<F1, F2, T1, T2>(part_1: F1, part_2: F2)
where
    F1: Fn() -> T1,
    F2: Fn() -> T2,
    T1: Display,
    T2: Display,
{
    let time = Instant::now();
    println!("Part 1: {}", part_1());
    println!("\x1b[90mExecution time: {:?}\x1b[0m", time.elapsed());

    let time = Instant::now();
    println!("Part 2: {}", part_2());
    println!("\x1b[90mExecution time: {:?}\x1b[0m", time.elapsed());
}
