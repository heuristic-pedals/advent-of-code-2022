// Advent of Code 2022 Day 7 Solutions
// Source: https://adventofcode.com/2022/day/7

use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use std::usize;

pub fn part1() {
    let timer = Instant::now();

    let input =  read_to_string("data/day_07/test_input.txt")
                            .unwrap()
                            .replace("$ ls\n", "")
                            .replace("$ cd ..\n", "");

    let parsed_input = input.split("$ cd ")
        .filter(|x| *x != "")
        .map(|y| y.split_once("\n").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut dir_map: HashMap<String, (Vec<String>, usize)> = HashMap::new();

    for dir_contents in parsed_input {
        let (k, v1, v2) = get_dirs_and_files(dir_contents);
        dir_map.insert(k, (v1, v2));
    }

    dbg!(dir_size(String::from("/"), &dir_map));

    println!("Elapsed time: {:.2?}", timer.elapsed());
}

fn get_dirs_and_files(dir_content: (&str, &str)) -> (String, Vec<String>, usize) {

    let k = dir_content.0.to_string();
    
    let mut num_contents: usize = 0;
    let mut num_dirs: usize = 0;
    for line in dir_content.1.lines() {
        num_contents += 1;
        if line.starts_with("dir") { num_dirs += 1};
    }
    let num_files: usize = num_contents - num_dirs;

    let mut dirs: Vec<String> = vec![String::new(); num_dirs];
    let mut files: Vec<usize> = vec![0; num_files];
    let mut i: usize = 0;
    let mut j: usize = 0;

    for line in dir_content.1.lines(){
        if line.starts_with("dir"){
            dirs[i] = line.split_once(" ").unwrap().1.to_string();
            i += 1;
        } else {
            files[j] = line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
            j += 1;
        }
    }

    (k, dirs, files.iter().sum::<usize>())
}

fn dir_size(dir_name: String, dir_map: &HashMap<String, (Vec<String>, usize)>) -> usize {

    let dir_contents = dir_map.get(&dir_name).unwrap();

    if dir_contents.0.len() != 0 {
        let mut sub_dir_total: usize = 0;
        for dir in &dir_contents.0 {
            sub_dir_total += dir_size(dir.clone(), dir_map);
        }
        return dir_contents.1 + sub_dir_total;

    } else {
        return dir_contents.1
    }
}