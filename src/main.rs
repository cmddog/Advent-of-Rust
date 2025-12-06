use inquire::Select;

mod runner;
mod y2015;
mod y2025;

fn main() {
    let year = Select::new("Select year:", vec![2015, 2025])
        .prompt()
        .unwrap();

    let days = match year {
        2015 => y2015::available_days(),
        2025 => y2025::available_days(),
        _ => vec![],
    };

    let day = Select::new("Select day:", days)
        .prompt()
        .unwrap();

    println!("\n========== {} Day {} ==========", year, day);
    match year {
        2015 => y2015::run(day),
        2025 => y2025::run(day),
        _ => println!("Year not implemented"),
    }
}
