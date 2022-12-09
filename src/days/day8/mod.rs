use std::fmt::{Debug, Formatter};


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub(crate) fn run() {
    part1();
}

pub(crate) fn part1() -> usize {
    let input = include_str!("test.txt");
    let lines = input.lines().enumerate();

    let lines: Vec<Vec<char>> = lines.map(|(_idx, line)| {
        line.chars().collect::<Vec<char>>()
    }).collect();

    let columns = transpose(lines.clone());

    println!("{:?}", lines);
    println!("{:?}", columns);

    let mut result = (lines.len()* 2) + (columns.len() -2 ) * 2;

    for line in lines {
        let mut max = 0;
        let mut max_idx = 0;
        for (idx, c) in line.iter().enumerate() {
            let curr = c.to_digit(10).unwrap();
            if curr > max {
                max = curr;
                max_idx = idx;
            }
        }
    }

    return result;
}