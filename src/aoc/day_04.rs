// Advent of Code 2022 Day 4 Solutions
// Source: https://adventofcode.com/2022/day/4

use std::{fs::read_to_string, usize};
use regex::Regex;

pub fn part_1 () {
    let input: String = read_to_string("data/day_04/test_input.txt").unwrap();

    let vec_input: Vec<Vec<usize>> = input
        .lines()
        .map(|x| parse_line(x.to_string()))
        .collect();

    dbg!(&vec_input);

}

/// parse input line into a vector of integers using regex
fn parse_line(line: String) -> Vec<usize> {

    let pattern = Regex::new(r"[0-9]+").unwrap();

    pattern
        .find_iter(line.as_str())
        .map(|digits| digits.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}