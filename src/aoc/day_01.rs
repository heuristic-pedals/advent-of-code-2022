// super is equivalent to crate::aoc::utils;
use super::utils;

pub fn part1() {
    println!("day 01");
    let input = utils::read_text_input("data/day_01/test_input.txt");

    for line in input{
        let line = line.unwrap();
        println!("{line}")
    }

}
