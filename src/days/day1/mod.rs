pub(crate) fn run() {
    let input = include_str!("actual.txt");
    // split into line groups separated by empty lines
    let groups = input.split("\n\n").collect::<Vec<&str>>();

    let mut sums: Vec<i32> = groups.iter().map(|group| {
        let mut sum = 0;
        for line in group.split("\n") {
            sum += line.parse().unwrap_or(0);
        }
        return sum;
    }).collect();
    sums.sort();

    println!("Max Sum: {:?}", sums.iter().rev().take(3).sum::<i32>());

    println!("Day 1");
}