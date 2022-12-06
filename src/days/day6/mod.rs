fn all_different(chars: &[char], len: usize) -> bool {
    let mut chars = chars.to_vec();
    chars.sort();
    chars.dedup();
    return chars.len() == len;
}

pub(crate) fn run() {}


pub(crate) fn part1() -> i32 {
    let string = include_str!("actual.txt");
    let mut counter = 4;
    for chars in string.chars().collect::<Vec<char>>().windows(4) {
        if all_different(chars, 4 as usize) {
            println!("{} is the answer", counter);
            break;
        }
        counter += 1;
    }

    return counter as i32;
}

pub(crate) fn part2() -> i32 {
    let string = include_str!("actual.txt");
    let mut counter = 14;
    for chars in string.chars().collect::<Vec<char>>().windows(14) {
        if all_different(chars, 14 as usize) {
            println!("{} is the answer", counter);
            break;
        }
        counter += 1;
    }

    return counter as i32;
}