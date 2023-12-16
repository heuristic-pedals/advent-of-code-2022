mod utils;
mod days;

fn main() {
    // parse day number from cli input
    let day: u8 = utils::parse_cli_day();

    // day selection
    match day {
        1 => days::day_01::part1(),
        _ => println!("Unknown day number {day}."),
    }
}
