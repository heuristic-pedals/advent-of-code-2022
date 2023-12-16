mod days;

fn main() {

    let day: u8 = 1;

    match day {
        1 => {days::day_01::part1()},
        _ => println!("Unknown day {day}.")
    }

}
