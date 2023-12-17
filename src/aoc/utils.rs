use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_cli_day() -> u8 {
    // collect day cli arg - program name takes 0th position so use 1st handle
    // case when unexpected number of arguments are given, then parse
    let args: Vec<String> = env::args().collect();
    let num_args: usize = args.len() - 1;
    if num_args != 1 {
        panic!("Unexpected number of argement. Expected 1, got {num_args}.");
    }
    args[1].trim().parse().expect("Unable to parse input.")
}

pub fn read_input_buffer(file_path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // read in a text file. modified from this source:
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    println!("Reading text input: {file_path}");
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
