fn total_overlap(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    if x1 <= y1 && x2 >= y2 {
        return true;
    }
    if y1 <= x1 && y2 >= x2 {
        return true;
    }
    return false;
}

fn partial_overlap(ranges: Vec<(i32, i32)>) -> i32 {
    let first = ranges[0];
    let second = ranges[1];
    if second.0 > first.1 || second.1 < first.0 {
        return 0;
    }

    return 1;
}

pub(crate) fn run() {
    let pairs = include_str!("actual.txt");

    let result: i32 = pairs.split('\n').map(|pair| {
        let ranges: Vec<(i32, i32)> = pair.split(',')
            .map(|range| {
                let mut split = range.split('-');
                let start = split.next().unwrap().parse::<i32>().unwrap();
                let end = split.next().unwrap().parse::<i32>().unwrap();
                return (start, end);
            }).collect();
        partial_overlap(ranges)
    }).sum();

    println!("{}", result);
}