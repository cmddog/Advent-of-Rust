use crate::runner::run_day;

mod day1;
mod day2;
mod day5;

pub fn run(day: u8) {
    match day {
        1 => run_day(day1::part_1, day1::part_2),
        2 => run_day(day2::part_1, day2::part_1),
        5 => run_day(day5::part_1, day5::part_2),
        _ => println!("Day {} not implemented", day),
    }
}

pub fn available_days() -> Vec<u8> {
    vec![1, 2, 5]
}