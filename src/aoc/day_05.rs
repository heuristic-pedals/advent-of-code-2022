// Advent of Code Day 5 Solutions
// Source: https://adventofcode.com/2022/day/5

use std::fs::read_to_string;
use std::time::Instant;

pub fn part_1 () {
    let timer: Instant = Instant::now();

    let input: String = read_to_string("data/day_05/test_input.txt").unwrap();

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

    dbg!(&crates);

    let num_crates = crates
        .into_iter()
        .map(|x| x.into_iter().filter(|y| *y != ' ').count())
        .collect::<Vec<usize>>();

    dbg!(&num_crates);

    let _moves: Vec<Vec<u8>> = parse_instructions(instructions_sec);

    println!("Day 5 Part 1: Last crates spell out: {:?}", "hello");
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

fn linspace(x0: usize, x1: usize, dx: usize) -> Vec<usize> {
    let mut x: Vec<usize> = vec![x0; (x1 - x0)/dx + 1];
    for i in 1..x.len() {
        x[i] = x[i-1] + dx;
    }
    x
}

fn parse_instructions(inst: &str) -> Vec<Vec<u8>> {
    inst.lines()
        .map(
            |x| x.replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",")
            .split(',')
            .map(|y| y.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
        )
        .collect()
}