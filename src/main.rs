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
        "5" => days::day5::run(),
        "6" => days::day6::run(),
        "7" => days::day7::run(),
        "8" => days::day8::run(),
        "9" => days::day9::run(),
        _ => println!("Day {} not implemented yet", day),
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

// #[test]
    // fn test_day5() {
    //     let result1 = days::day5::part1();
    //     assert_eq!(result1, String::from("CMZ"));
    //     let result1 = days::day5::part2();
    //     assert_eq!(result1, String::from("MCD"));
    // }

    // #[test]
    // fn test_day6() {
    //     let res1 = days::day6::part1();
    //     assert_eq!(res1, 1876);
    //     let res1 = days::day6::part2();
    //     assert_eq!(res1, 2202);
    // }

    // #[test]
    // fn test_day7() {
    //     let res1 = days::day7::part1();
    //     assert_eq!(res1, 95437);
    // }

    #[test]
    fn test_day9() {
        let res1 = days::day9::part1();
        assert_eq!(res1, 6057);
        let res1 = days::day9::part2();
        assert_eq!(res1, 2514);
    }

    // #[bench]
    // fn bench_day3(b: &mut Bencher) {
    //     b.iter(|| days::day3::run());
    // }
    //
    // #[bench]
    // fn bench_day4(b: &mut Bencher) {
    //     b.iter(|| days::day4::run());
    // }
    //
    // #[bench]
    // fn bench_day5_p1(b: &mut Bencher) {
    //     b.iter(|| days::day5::part1());
    // }
    //
    // #[bench]
    // fn bench_day5_p2(b: &mut Bencher) {
    //     b.iter(|| days::day5::part2());
    // }
    //
    // #[bench]
    // fn bench_day6_p1(b: &mut Bencher) {
    //     b.iter(|| days::day6::part1());
    // }
    //
    // #[bench]
    // fn bench_day6_p2(b: &mut Bencher) {
    //     b.iter(|| days::day6::part2());
    // }

    #[bench]
    fn bench_day9_p1(b: &mut Bencher) {
        b.iter(|| days::day9::part1());
    }

    #[bench]
    fn bench_day9_p2(b: &mut Bencher) {
        b.iter(|| days::day9::part2());
    }
}
