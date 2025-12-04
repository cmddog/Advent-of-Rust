use std::time::Instant;

mod day1;
mod day2;

pub fn run() {
    println!("----- Day 1 -----");
    let time = Instant::now();
    println!("Part 1: {}", day1::part_1());
    println!("\x1b[90mElapsed time: {:?}\x1b[0m", time.elapsed());
    let time = Instant::now();
    println!("Part 2: {}", day1::part_2());
    println!("\x1b[90mElapsed time: {:?}\x1b[0m", time.elapsed());
}