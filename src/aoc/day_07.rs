// Advent of Code 2022 Day 7 Solutions
// Source: https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
use std::usize;

pub fn part1() {
    let timer = Instant::now();

    // dedup nested folder names and remove unnessary lines
    let dedup_input = dedup_folder_names("data/day_07/input.txt");
    let input = dedup_input.replace("$ ls\n", "").replace("$ cd ..\n", "");

    // split lines into dir names and contents, parse dir contents and create hashmap
    let parsed_input = input
        .split("$ cd ")
        .filter(|x| *x != "")
        .map(|y| y.split_once("\n").unwrap())
        .collect::<Vec<(&str, &str)>>();
    let mut dir_map: HashMap<String, (Vec<String>, usize)> = HashMap::new();
    for dir_contents in parsed_input {
        let (k, v1, v2) = get_dirs_and_files(dir_contents);
        dir_map.insert(k, (v1, v2));
    }

    // collate results for each dir into hashmap
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    dir_size(String::from("/"), &dir_map, &mut dir_sizes);

    // get sum of dir sizes < 100000
    let dir_sizes = dir_sizes
        .values()
        .into_iter()
        .filter(|x| **x <= 100000)
        .sum::<usize>();

    assert_eq!(dir_sizes, 1490523);
    println!(
        "Day 7 Part 1: Combined size of dirs <= 100000: {:?}",
        dir_sizes
    );
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

/// Parse a dir's content into name, vec of sub-dirs and total size of files
/// 
/// # Arguments
///
/// * `dir_content` - All rows associated with the directory content from input
fn get_dirs_and_files(dir_content: (&str, &str)) -> (String, Vec<String>, usize) {
    // parse input to get num sub-dirs and files in dir to alloc vec memory
    let mut num_contents: usize = 0;
    let mut num_dirs: usize = 0;
    for line in dir_content.1.lines() {
        num_contents += 1;
        if line.starts_with("dir") {
            num_dirs += 1
        };
    }
    let num_files: usize = num_contents - num_dirs;
    let mut dirs: Vec<String> = vec![String::new(); num_dirs];
    let mut files: Vec<usize> = vec![0; num_files];
    
    // parse contents to strip out sub-dir names and file sizes
    let mut i: usize = 0;
    let mut j: usize = 0;
    for line in dir_content.1.lines() {
        if line.starts_with("dir") {
            dirs[i] = line.split_once(" ").unwrap().1.to_string();
            i += 1;
        } else {
            files[j] = line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
            j += 1;
        }
    }
    (dir_content.0.to_string(), dirs, files.iter().sum::<usize>())
}

/// Recursively calculate the size of a directory, including all sub-directorys
/// and files
/// 
/// # Arguments
///
/// * `dir_name` - Name of the directory
/// * `dir_map` - A map between dirs and their subdirs, and total file contents
/// * `dir_sizes` - A map between dirs and their total sizes. Mutable to allow
///                 updates during recursion
fn dir_size(
    dir_name: String,
    dir_map: &HashMap<String, (Vec<String>, usize)>,
    dir_sizes: &mut HashMap<String, usize>,
) -> usize {
    let dir_contents = dir_map.get(&dir_name).unwrap();
    // base case: no sub-dirs, just return total of files. Otherwise, recurisvely
    // get the size of the sub-dirs too.
    if dir_contents.0.len() != 0 {
        let mut sub_dir_total: usize = 0;
        for dir in &dir_contents.0 {
            sub_dir_total += dir_size(dir.clone(), dir_map, dir_sizes);
        }
        dir_sizes.insert(dir_name, dir_contents.1 + sub_dir_total);
        return dir_contents.1 + sub_dir_total;
    } else {
        dir_sizes.insert(dir_name, dir_contents.1);
        return dir_contents.1;
    }
}

/// De-dupllicate sub-dir folder names. Must be unique to be able to identify a
/// sub-dir's content uniquely. This is achieved by 'walking' through the commands
/// and changing dir names to be absolute relative to the root. Dashes are used to
/// segregate sub-dir names so that when moving up (using cd ..) the last sub-dir
/// can be easily removed (no need to store a history in memory).
/// 
/// # Arguments
///
/// * `path` - Path to input
fn dedup_folder_names(path: &str) -> String {
    // read input and prep a mutable copy to edit
    let orig_input = read_to_string(path).unwrap();
    let orig_input = orig_input.lines().collect::<Vec<&str>>();
    let mut dedup_input = orig_input
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let num_lines = orig_input.iter().count();
    let mut prefix = String::new();

    for i in 0..num_lines {
        // case moving into a directory: align prefix with new dir and change name in dedup_input
        if orig_input[i].starts_with("$ cd ") && !orig_input[i].contains("..") {
            prefix = prefix + &orig_input[i].replace("$ cd ", "").to_string() + "-";
            dedup_input[i] = "$ cd ".to_string() + &prefix[..prefix.len() - 1]
        // case moving 'up': remove previous sub-dir from prefix
        } else if orig_input[i] == ("$ cd ..") {
            let mut refeshed_prefix = String::from("");
            let dirs = prefix
                .split("-")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if dirs.len() == 1 {
                refeshed_prefix = "/-".to_string();  // need to handle back to root case
            } else {
                for j in 0..dirs.len() - 2 {
                    refeshed_prefix = refeshed_prefix + &dirs[j] + "-";
                }
            }
            prefix = refeshed_prefix;
        // case showing dir contents: update dir name, pre-pending prefix
        } else if orig_input[i].starts_with("dir ") {
            dedup_input[i] = "dir ".to_string() + &prefix + &orig_input[i].replace("dir ", "");
        }
    }

    dedup_input.join("\n")  // join dedup results back to a single string
}
