// Advent of Code Day 5 Solutions
// Source: https://adventofcode.com/2022/day/5

use std::fs::read_to_string;
use std::time::Instant;

pub fn part_1 () {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_05/input.txt").unwrap();

    let (crates_sec, instructions_sec) = input.split_once("\n\n").unwrap();

    let (first_line, _) = crates_sec.split_once("\n").unwrap();
    let create_indices = linspace(1, first_line.len(), 4);
    let num_indicies = create_indices.len();

    let max_num_crates_start = crates_sec.lines().count() - 1;

    // alloc memory for an n x m matrix where n = space for all crates in one column, m = number of create columns
    let mut crates = vec![vec![' '; max_num_crates_start * num_indicies]; num_indicies];

    // parse crates into starting locations
    for (i, line) in crates_sec.lines().rev().enumerate() {
        if i != 0 {
            let chars: Vec<char> = line.chars().collect::<Vec<char>>();
            let mut j: usize = 0;
            for create_idx in &create_indices {
                crates[j][i-1] = chars[*create_idx];
                j+=1;
            }
        }
    }

    // get number of crates in each column
    let mut num_crates = crates.clone()
        .into_iter()
        .map(|x| x.into_iter().filter(|y| *y != ' ').count())
        .collect::<Vec<usize>>();

    let moves: Vec<Vec<usize>> = parse_instructions(instructions_sec);

    for crate_move in &moves {
        let num_move = crate_move[0];
        let from = crate_move[1] - 1;
        let to = crate_move[2] - 1;

        let mut crates_moving = vec![' '; num_move];
        let mut i = crates_moving.len();
        for crate_char in &crates[from][num_crates[from]-num_move..num_crates[from]] {
            i -= 1;
            crates_moving[i] = *crate_char;
        }

        let mut j = num_crates[to];
        for crate_char in crates_moving {
            crates[to][j] = crate_char;
            j += 1;
        }

        for k in num_crates[from]-num_move..num_crates[from] {
            crates[from][k] = ' ';
        }

        num_crates[from] -= num_move;
        num_crates[to] += num_move;
    }

    let mut last_crates = vec![' '; num_indicies];
    for i in 0..last_crates.len() {
        last_crates[i] = crates[i][num_crates[i] - 1];
    }

    let last_crates_word = last_crates.into_iter().collect::<String>();

    println!("Day 5 Part 1: Last crates spell out: {:?}", last_crates_word);
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

fn linspace(x0: usize, x1: usize, dx: usize) -> Vec<usize> {
    let mut x: Vec<usize> = vec![x0; (x1 - x0)/dx + 1];
    for i in 1..x.len() {
        x[i] = x[i-1] + dx;
    }
    x
}

fn parse_instructions(inst: &str) -> Vec<Vec<usize>> {
    inst.lines()
        .map(
            |x| x.replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",")
            .split(',')
            .map(|y| y.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )
        .collect()
}