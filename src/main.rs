mod aoc;

use aoc::utils;

fn main() {
    // parse day number from cli input
    let day: u8 = utils::parse_cli_day();

    // day selection
    match day {
        1 => aoc::day_01::part1_part2_v2(),
        2 => aoc::day_02::part1_part2(),
        3 => aoc::day_03::part1_part2(),
        4 => aoc::day_04::part1_part2(),
        5 => aoc::day_05::part1_part2(),
        6 => aoc::day_06::part_1(),
        _ => println!("Unknown day number {day}."),
    }
}
