use std::string::ToString;

use crate::days::day2::People::{Draw, Me, Oppo};
use crate::days::day2::people::People;
use crate::days::day2::round::Round;
use crate::days::day2::Shape::{A, B, C};
use crate::days::day2::shape::Shape;

mod people;
mod round;
mod shape;

pub(crate) fn run() {
    let input = include_str!("actual.txt");
    let rounds: Vec<Round> = input.split("\n").map(|line| {
        let mut split = line.split(" ");
        let oppo = Shape::try_from(split.next().unwrap().to_string()).unwrap();
        let winner = People::try_from(split.next().unwrap().to_string()).unwrap();
        let mut round = Round::new(
            oppo,
            None,
            Some(winner),
        );
        round.calc_winner();
        return round;
    }).collect();

    let sum = rounds.iter().map(|round| {
        round.points.unwrap_or(0)
    }).collect::<Vec<i32>>().into_iter().sum::<i32>();

    println!("{}", sum);
}