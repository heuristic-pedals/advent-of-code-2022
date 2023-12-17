mod aoc;

use aoc::utils;

fn main() {
    // parse day number from cli input
    let day: u8 = utils::parse_cli_day();

    // day selection
    match day {
        1 => aoc::day_01::part1_part2(),
        _ => println!("Unknown day number {day}."),
    }
}
