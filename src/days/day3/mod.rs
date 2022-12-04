fn find_common(input: (&str, &str)) -> Option<char> {
    for (_i, c) in input.0.chars().enumerate() {
        for c2 in input.1.chars() {
            if c == c2 {
                return Some(c);
            }
        }
    }
    return None;
}

fn find_common_triple(input: (&str, &str, &str)) -> Option<char> {
    for (_i, c) in input.0.chars().enumerate() {
        for c2 in input.1.chars() {
            if c == c2 {
                for c3 in input.2.chars() {
                    if c == c3 {
                        return Some(c);
                    }
                }
            }
        }
    }
    return None;
}

fn priority(c: Option<char>) -> i32 {
    if c.is_none() {
        return 0;
    }
    let intval = c.unwrap() as i32;
    return if intval <= 90 {
        intval - 38
    } else {
        intval - 96
    };
}


pub(crate) fn run() {
    let input = include_str!("actual.txt");
    let lines = input.lines();
    let mut priorities: Vec<i32> = vec![];

    for line in lines {
        priorities.push(
            priority(
                find_common(line.split_at(line.len() / 2))
            )
        );
    }

    println!("Part 1 solution: {}", priorities.iter().sum::<i32>());

    let input2 = include_str!("actual.txt");
    let badge_groups = input2.lines();
    let vector = badge_groups.collect::<Vec<&str>>();
    let mut part2_priority = 0;
    for chunk in vector.chunks(3) {
        part2_priority += priority(
            find_common_triple(
                (chunk[0], chunk[1], chunk[2])
            )
        );
    }

    println!("Part 2 solution: {}", part2_priority);
}