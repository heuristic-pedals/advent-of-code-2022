// Advent of Code Day 5 Solutions
// Source: https://adventofcode.com/2022/day/5

use std::fs::read_to_string;
use std::time::Instant;

pub fn part1_part2() {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_05/input.txt").unwrap();
    let (crates_sec, instructions_sec) = input.split_once("\n\n").unwrap();

    // stack initial crates, get number of crates in each column, get instrucitons
    let (mut crates, num_crate_cols) = stack_initial_crates(crates_sec);
    let mut num_crates: Vec<usize> = crates_per_column(&crates);
    let moves: Vec<Vec<usize>> = parse_instructions(instructions_sec);

    // log identical starting configuration for part 2
    let mut crates_p2: Vec<Vec<char>> = crates.clone();
    let mut num_crates_p2: Vec<usize> = num_crates.clone();

    // move crates using cratemover 9000 and 9001 features
    let part_1_last_crates: String =
        move_crates(&mut crates, &moves, &mut num_crates, num_crate_cols, true);
    let part_2_last_crates: String = move_crates(
        &mut crates_p2,
        &moves,
        &mut num_crates_p2,
        num_crate_cols,
        false,
    );

    println!(
        "Day 5 Part 1: Last crates spell out: {:?}",
        part_1_last_crates
    );
    println!(
        "Day 5 Part 2: Last crates spell out: {:?}",
        part_2_last_crates
    );
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// create a linspace between x0 and x1 with spacing dx
fn linspace(x0: usize, x1: usize, dx: usize) -> Vec<usize> {
    let mut x: Vec<usize> = vec![x0; (x1 - x0) / dx + 1];
    for i in 1..x.len() {
        x[i] = x[i - 1] + dx;
    }
    x
}

/// knife and fork instructions out of line
fn parse_instructions(inst: &str) -> Vec<Vec<usize>> {
    inst.lines()
        .map(|x| {
            x.replace("move ", "")
                .replace(" from ", ",")
                .replace(" to ", ",")
                .split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn stack_initial_crates(crates_sec: &str) -> (Vec<Vec<char>>, usize) {
    // alloc memory for an n x m matrix where n = space for all crates in one column,
    // m = number of create columns
    let (first_line, _) = crates_sec.split_once("\n").unwrap();
    let create_indices: Vec<usize> = linspace(1, first_line.len(), 4);
    let num_crate_cols: usize = create_indices.len();
    let max_num_crates_start: usize = crates_sec.lines().count() - 1;
    let mut crates: Vec<Vec<char>> =
        vec![vec![' '; max_num_crates_start * num_crate_cols]; num_crate_cols];

    // parse crates into starting locations
    for (i, line) in crates_sec.lines().rev().enumerate() {
        if i != 0 {
            let chars: Vec<char> = line.chars().collect::<Vec<char>>();
            let mut j: usize = 0;
            for create_idx in &create_indices {
                crates[j][i - 1] = chars[*create_idx];
                j += 1;
            }
        }
    }
    (crates, num_crate_cols)
}

fn crates_per_column(crates: &Vec<Vec<char>>) -> Vec<usize> {
    crates
        .into_iter()
        .map(|x| x.into_iter().filter(|y| **y != ' ').count())
        .collect::<Vec<usize>>()
}

fn move_crates(
    crates: &mut Vec<Vec<char>>,
    moves: &Vec<Vec<usize>>,
    num_crates: &mut Vec<usize>,
    num_crate_cols: usize,
    cratemover_9000: bool,
) -> String {
    for crate_move in moves {
        // knife and fork instructions out and re-index col nums to zero
        let num_move: usize = crate_move[0];
        let from: usize = crate_move[1] - 1;
        let to: usize = crate_move[2] - 1;

        // move and update crate numbers
        let mut j: usize = num_crates[to];
        match cratemover_9000 {
            true => {
                for i in (num_crates[from] - num_move..num_crates[from]).rev() {
                    crates[to][j] = crates[from][i];
                    crates[from][i] = ' ';
                    j += 1;
                }
            }
            _ => {
                for i in num_crates[from] - num_move..num_crates[from] {
                    crates[to][j] = crates[from][i];
                    crates[from][i] = ' ';
                    j += 1;
                }
            }
        };
        num_crates[from] -= num_move;
        num_crates[to] += num_move;
    }

    // get last crate chars and parse into a string
    let mut last_crates = vec![' '; num_crate_cols];
    for i in 0..last_crates.len() {
        last_crates[i] = crates[i][num_crates[i] - 1];
    }
    last_crates.into_iter().collect::<String>()
}
