// Advent of Code Day 6 Solutions
// Source: https://adventofcode.com/2022/day/6

use std::fs::read_to_string;
use itertools::Itertools;
use std::time::Instant;

pub fn part1_part2() {

    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_06/input.txt").unwrap();
    let packet_window_size: usize = 4;
    let message_window_size: usize = 14;

    let packet_start: usize = find_marker(packet_window_size, &input);
    let message_start: usize = find_marker(message_window_size, &input);

    println!("Day 6 Part 1: Packet start {}", packet_start);
    println!("Day 6 Part 2: Message start {}", message_start);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

pub fn find_marker(window_size: usize, input: &str) -> usize {
    let (idx, _state) = input.chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
            .map(|(i, x)| (i, x.into_iter().unique().count() == window_size))
            .find(|(_i, x)| *x == true)
            .unwrap();

    idx + window_size
}