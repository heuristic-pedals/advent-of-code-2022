// super is equivalent to crate::aoc::utils;
use super::utils;
use std::time::Instant;

pub fn _part1() {

    let lines: Vec<String> = utils::read_input_whole("data/day_01/input.txt");

    let mut max_elf: (usize, usize) = (0, 0);
    let mut elf_cal_total: usize = 0;
    let mut elf_id: usize = 1;

    for line in lines {
        let calories: usize = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if elf_cal_total > max_elf.1 {
                    max_elf = (elf_id, elf_cal_total);
                }
                elf_id += 1;
                elf_cal_total = 0;
                continue;
            },
        };
        elf_cal_total += calories
    }
    println!("Part 1: Elf number {} has the most calories at {}", max_elf.0, max_elf.1);

    assert_eq!(max_elf.1, 69501);

}

pub fn part1_part2() {

    let timer: Instant = Instant::now();

    let lines: Vec<String> = utils::read_input_whole("data/day_01/input.txt");

    let top_x: usize = 3;
    let mut max_cals: Vec<usize> = vec![0; top_x];
    let mut max_ids: Vec<usize> = vec![0; top_x];
    let mut elf_cal_total: usize = 0;
    let mut elf_id: usize = 1;

    for line in lines{
        if line != "" {
            let calories: usize = line.trim().parse().unwrap();
            elf_cal_total += calories;
        } else {
            let min_top_cals: &usize = max_cals.iter().min().unwrap();
            let min_top_cals_idx: usize = max_cals.iter().position(|x: &usize| x == min_top_cals).unwrap();
            if elf_cal_total > *min_top_cals {
                max_cals[min_top_cals_idx] = elf_cal_total;
                max_ids[min_top_cals_idx] = elf_id;
            }
            elf_id += 1;
            elf_cal_total = 0;
        }

    }  

    let top_x_total: usize = max_cals.iter().sum();

    println!("Part 1: Elf number {} has the most calories at {}", max_ids[0], max_cals[0]);
    println!("Part 2: Sum of top {} elves' calories is {}", top_x, top_x_total);
    println!("Elapsed time: {:.2?}", timer.elapsed());


}