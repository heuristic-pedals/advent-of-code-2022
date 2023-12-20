// Advent of Code 2022 Day 3 Solutions
// source: https://adventofcode.com/2022/day/3

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub fn part1() {
    let timer: Instant = Instant::now();

    let file: File = File::open("data/day_03/input.txt").unwrap();

    let total_items: usize = io::BufReader::new(file)
        .lines()
        .map(|x: Result<String, io::Error>| x.unwrap())
        .map(|y| get_dup_item(&y))
        .sum();

    println!("Day 3 Part 1: Sum of duplicate items is {:?}", total_items);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// Detect duplicate item between the two compartments
///
/// # Arguments
///
/// * `items` - A string deonoting the items in both compartments
fn get_dup_item(items: &String) -> usize {

    let (comp_1, comp_2) = items.split_at(items.len()/2);

    let unique_bytes: &u8 = comp_1
        .as_bytes()
        .iter()
        .filter(|x| comp_2.as_bytes().contains(x))
        .collect::<Vec<&u8>>()[0];

    let item: u8 = match unique_bytes > &96 {
        true => unique_bytes - 96,
        _ => unique_bytes - 38,
    };

    usize::from(item)
}