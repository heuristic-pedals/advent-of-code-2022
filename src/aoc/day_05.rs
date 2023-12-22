// Advent of Code Day 5 Solutions
// Source: https://adventofcode.com/2022/day/5

use std::fs::read_to_string;
use std::time::Instant;

pub fn part_1 () {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_05/test_input.txt").unwrap();

    let (_crates, instructions) = input.split_once("\n\n").unwrap();

    let _moves: Vec<Vec<u8>> = parse_instructions(instructions);

    println!("Day 5 Part 1: Last crates spell out: {:?}", "hello");
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

fn parse_instructions(inst: &str) -> Vec<Vec<u8>> {
    inst.lines()
        .map(
            |x| x.replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",")
            .split(',')
            .map(|y| y.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
        )
        .collect()
}