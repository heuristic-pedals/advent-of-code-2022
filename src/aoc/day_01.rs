// super is equivalent to crate::aoc::utils;
use super::utils;

pub fn part1() {
    println!("day 01");
    if let Ok(lines) = utils::read_input_buffer("data/day_01/test_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                println!("{line}")
            }
        }
    }
}
