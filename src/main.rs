use std::env;

mod days;

// read day from command line and launch function dayN where N is the integer
fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_ref() {
        "1" => days::day1::run(),
        "2" => days::day2::run(),
        "3" => days::day3::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}
