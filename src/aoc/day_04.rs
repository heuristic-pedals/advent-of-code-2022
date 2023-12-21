// Advent of Code 2022 Day 4 Solutions
// Source: https://adventofcode.com/2022/day/4

use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

pub fn part_1() {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_04/input.txt").unwrap();

    let num_overlaps: usize = input
        .lines()
        .map(|x| parse_ints_in_line(&x))
        .map(|y| overlapping_bounds(y))
        .sum();

    println!("Day 4 Part 1: Number of overlaps {:?}", num_overlaps);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// Parse input line into vector of integers using regex
///
/// # Arguments
///
/// * `line` - A string deonoting the line to parse
fn parse_ints_in_line(line: &str) -> Vec<usize> {
    let pattern = Regex::new(r"[0-9]+").unwrap();
    pattern
        .find_iter(line)
        .map(|digits| digits.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

/// Check for overlapping bounds
///
/// # Arguments
///
/// * `limits` - A vector of limits, paired like [min1, max1, min2, max2]
fn overlapping_bounds(limits: Vec<usize>) -> usize {
    let diff1: usize = limits[1] - limits[0];
    let diff2: usize = limits[3] - limits[2];

    if diff1 >= diff2 {
        if (limits[2] >= limits[0]) & (limits[3] <= limits[1]) {
            return 1;
        } else {
            return 0;
        }
    } else {
        if (limits[0] >= limits[2]) & (limits[1] <= limits[3]) {
            return 1;
        } else {
            return 0;
        }
    }
}
