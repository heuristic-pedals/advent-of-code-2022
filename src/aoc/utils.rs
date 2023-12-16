use std::env;

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
