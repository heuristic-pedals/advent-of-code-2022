// Advent of Code Day 2 Solutions
// Source: https://adventofcode.com/2022/day/2

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub fn part1_part2() {
    let timer: Instant = Instant::now();

    let file: File = File::open("data/day_02/input.txt").unwrap();

    let points: Vec<(usize, usize)> = io::BufReader::new(file)
        .lines()
        .map(|x: Result<String, io::Error>| x.unwrap())
        .map(|y: String| {
            (
                score_round_using_prescribed_move(&y),
                score_round_using_prescribed_result(&y),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let mut move_total: usize = 0;
    let mut result_total: usize = 0;
    for (x, y) in points.iter() {
        move_total += x;
        result_total += y;
    }

    println!("Day 2 Part 1: Total number of points is {:?}", move_total);
    println!("Day 2 Part 2: Total number of points is {:?}", result_total);
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

/// Returns a score based on playing in accordance with prescibed result.
///
/// # Arguments
///
/// * `round` - A string deonoting the competitors move and the precribed
///             result separated by a space.
fn score_round_using_prescribed_result(round: &String) -> usize {
    let score_bytes: &[u8] = round.as_bytes();
    match score_bytes[2] {
        b'X' => {
            // need to lose
            match score_bytes[0] {
                b'A' => 3, // play scissors (3) and lose (0)
                b'B' => 1, // play rock (1) and lose (0)
                _ => 2,    // plsy paper (2) and lose (0)
            }
        }
        b'Y' => {
            // need to draw
            match score_bytes[0] {
                b'A' => 4, // play rock (1) and draw (3)
                b'B' => 5, // play paper (2) and draw (3)
                _ => 6,    // play scissors (3) and draw (3)
            }
        }
        _ => {
            // need to win
            match score_bytes[0] {
                b'A' => 8, // play paper (2) and win (6)
                b'B' => 9, // play scissors (3) and win (6)
                _ => 7,    // play rock (1) and win (6)
            }
        }
    }
}
