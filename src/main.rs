mod day_01;

fn main() {

    let day: u8 = 1;

    match day {
        1 => {day_01::day_01()},
        _ => println!("Unknown day {day}.")
    }

}
