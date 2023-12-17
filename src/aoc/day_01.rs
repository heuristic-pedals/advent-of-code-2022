// super is equivalent to crate::aoc::utils;
use super::utils;

pub fn part1() {

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
