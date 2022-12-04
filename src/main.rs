#![feature(test)]

extern crate test;

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
        "4" => days::day4::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_day3(b: &mut Bencher) {
        b.iter(|| days::day3::run());
    }

    #[bench]
    fn bench_day4(b: &mut Bencher) {
        b.iter(|| days::day4::run());
    }
}
