// Advent of Code 2022 Day 4 Solutions
// Source: https://adventofcode.com/2022/day/4

use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

pub fn part1_part2() {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_04/input.txt").unwrap();

    let parsed_lines = input.lines().map(|x| parse_ints_in_line(&x));

    let mut total_complete_overlaps: usize = 0;
    let mut total_partial_overlaps: usize = 0;
    for parsed_line in parsed_lines {
        total_complete_overlaps += completely_overlapping_bounds(&parsed_line);
        total_partial_overlaps += partially_overlapping_bounds(&parsed_line);
    }

    println!(
        "Day 4 Part 1: Number of complete overlaps {:?}",
        total_complete_overlaps
    );
    println!(
        "Day 4 Part 2: Number of partial overlaps {:?}",
        total_partial_overlaps
    );
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

/// Check for completely overlapping bounds
///
/// # Arguments
///
/// * `limits` - A vector of limits, paired like [min1, max1, min2, max2]
fn completely_overlapping_bounds(limits: &Vec<usize>) -> usize {
    let diff1: usize = limits[1] - limits[0];
    let diff2: usize = limits[3] - limits[2];
    match diff1 >= diff2 {
        true => {
            match (limits[2] >= limits[0]) && (limits[3] <= limits[1]) {
                true => 1,
                _ => 0,
            }
        }
        _ => {
            match (limits[0] >= limits[2]) && (limits[1] <= limits[3]) {
                true => 1,
                _ => 0,
            }
        }
    }
}

/// Check for partially overlapping bounds
///
/// # Arguments
///
/// * `limits` - A vector of limits, paired like [min1, max1, min2, max2]
fn partially_overlapping_bounds(limits: &Vec<usize>) -> usize {
    if (limits[1] >= limits[2]) && (limits[3] >= limits[0]) {
        return 1
    }
    0
}
