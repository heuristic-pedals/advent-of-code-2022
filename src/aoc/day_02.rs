// Advent of Code Day 2 Solutions
// Source: https://adventofcode.com/2022/day/2

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub fn part1() {
    let timer: Instant = Instant::now();

    let file: File = File::open("data/day_02/input.txt").unwrap();

    let points: usize = io::BufReader::new(file)
        .lines()
        .map(|x: Result<String, io::Error>| x.unwrap())
        .map(|y: String| score_round_using_prescribed_move(&y))
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("Day 2 Part 1: Total number of points is {:?}", points);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// Returns a score based on playing in accordance with prescibed move.
///
/// # Arguments
///
/// * `round` - A string deonoting the competitors move and the precribed move
///             separated by a space.
fn score_round_using_prescribed_move(round: &String) -> usize {
    let score_bytes: &[u8] = round.as_bytes();
    match score_bytes[0] {
        b'A' => {
            // competitor goes rock
            match score_bytes[2] {
                b'X' => 4, // draw (3) an played rock (1)
                b'Y' => 8, // win (6) and played paper (2)
                _ => 3,    // lose (0) and played scissors (3)
            }
        }
        b'B' => {
            // competitor goes paper
            match score_bytes[2] {
                b'X' => 1, // lose (0) and played rock (1)
                b'Y' => 5, // draw (3) and played paper (2)
                _ => 9,    // win (6) and played scissors (3)
            }
        }
        _ => {
            // competitor goes scissors
            match score_bytes[2] {
                b'X' => 7, // win (6) and played rock (1)
                b'Y' => 2, // lose (0) and played paper (2)
                _ => 6,    // draw (3) and played scissors (3)
            }
        }
    }
}
