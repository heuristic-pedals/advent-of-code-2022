// Advent of Code 2022 Day 3 Solutions
// source: https://adventofcode.com/2022/day/3

use std::fs::read_to_string;
use std::time::Instant;

pub fn part1_part2() {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_03/input.txt").unwrap();

    let total_items: usize = input
        .lines()
        .map(|y: &str| get_dup_item(y.to_string()))
        .sum();

    let vec_input = Vec::from_iter(input.lines());
    let chunked_input = vec_input.chunks(3);
    let total_badges: usize = chunked_input
        .map(|x: &[&str]| get_badge(x))
        .sum();

    println!("Day 3 Part 1: Sum of duplicate items is {:?}", total_items);
    println!("Day 3 Part 2: Sum of badges is {:?}", total_badges);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// Detect duplicate item between the two compartments
///
/// # Arguments
///
/// * `items` - A string deonoting the items in both compartments
fn get_dup_item(items: String) -> usize {

    let (comp_1, comp_2) = items.split_at(items.len()/2);

    let unique_bytes: &u8 = comp_1
        .as_bytes()
        .iter()
        .filter(|x| comp_2.as_bytes().contains(x))
        .collect::<Vec<&u8>>()[0];

    bytes_to_score(*unique_bytes)
}

/// Get the common badge between 3 consecutive bages
///
/// # Arguments
///
/// * `chunk` - A chunk representing 3 consecuitive strings deonoting the items
///             in both compartments of the bags
fn get_badge(chunk: &[&str]) -> usize {

    let bag_1 = chunk[0].to_string().into_bytes();

    let badge_bytes: &u8 = bag_1
        .iter()
        .filter(|x| chunk[1].to_string().into_bytes().contains(*x))
        .filter(|y| chunk[2].to_string().into_bytes().contains(*y))
        .collect::<Vec<&u8>>()[0];

    bytes_to_score(*badge_bytes)
}

/// Convert bytes representation of a char into the score
///
/// # Arguments
///
/// * `bytes_input` - Bytes representation of the char to convert.
fn bytes_to_score(bytes_input: u8) -> usize {
    let score: u8 = match bytes_input > 96 {
        true => bytes_input - 96,
        _ => bytes_input - 38,
    };
    usize::from(score)
}
