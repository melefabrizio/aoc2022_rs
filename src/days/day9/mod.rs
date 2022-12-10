use hashbrown::HashSet;
use std::fmt::Formatter;

#[derive(Eq, Hash, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

struct BoardP1 {
    head: Point,
    // tail is array of points
    tail: [Point; 1],
    tail_visited: HashSet<Point>,
}

impl BoardP1 {
    fn execute(&mut self, instruction: &str) {
        let iterator = instruction
            .split_ascii_whitespace().collect::<Vec<&str>>();
        let dir = iterator.first().unwrap();
        let steps = iterator.get(1).unwrap().parse::<i32>().unwrap();
        for _ in 0..steps {
            self.move_head(dir);
        }
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "U" => self.head.y += 1,
            "D" => self.head.y -= 1,
            "L" => self.head.x -= 1,
            "R" => self.head.x += 1,
            _ => panic!("Invalid direction"),
        }
        self.follow_tail(
            self.head,
            &mut self.tail.get(0).unwrap().clone(),
            0,
        );
        for i in 0..self.tail.len() - 1 {
            self.follow_tail(
                *self.tail.get(i).unwrap(),
                &mut self.tail.get(i + 1).unwrap().clone(),
                i + 1,
            );
        }
    }

    fn follow_tail(&mut self, head: Point, tail: &mut Point, tail_index: usize) {
        if (i32::abs(tail.x - head.x) <= 1) && (i32::abs(tail.y - head.y) <= 1) {
            return;
        }
        if (tail.x == head.x) || (tail.y == head.y) {
            if tail.x == head.x {
                tail.y += if tail.y < head.y { 1 } else { -1 };
            } else {
                tail.x += if tail.x < head.x { 1 } else { -1 };
            }
        } else {
            tail.x += if tail.x < head.x { 1 } else { -1 };
            tail.y += if tail.y < head.y { 1 } else { -1 };
        }
        self.tail[tail_index] = *tail;
        if tail_index == self.tail.len() - 1 {
            self.tail_visited.insert(*tail);
        }
    }
}

struct Board {
    head: Point,
    tail: [Point; 9],
    tail_visited: HashSet<Point>,
}

impl Board {
    fn execute(&mut self, instruction: &str) {
        let iterator = instruction
            .split_ascii_whitespace().collect::<Vec<&str>>();
        let dir = iterator.first().unwrap();
        let steps = iterator.get(1).unwrap().parse::<i32>().unwrap();
        for _ in 0..steps {
            self.move_head(dir);
        }
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "U" => self.head.y += 1,
            "D" => self.head.y -= 1,
            "L" => self.head.x -= 1,
            "R" => self.head.x += 1,
            _ => panic!("Invalid direction"),
        }
        self.follow_tail(
            self.head,
            &mut self.tail.get(0).unwrap().clone(),
            0,
        );
        for i in 0..self.tail.len() - 1 {
            self.follow_tail(
                *self.tail.get(i).unwrap(),
                &mut self.tail.get(i + 1).unwrap().clone(),
                i + 1,
            );
        }
    }

    fn follow_tail(&mut self, head: Point, tail: &mut Point, tail_index: usize) {
        if (i32::abs(tail.x - head.x) <= 1) && (i32::abs(tail.y - head.y) <= 1) {
            return;
        }
        if (tail.x == head.x) || (tail.y == head.y) {
            if tail.x == head.x {
                tail.y += if tail.y < head.y { 1 } else { -1 };
            } else {
                tail.x += if tail.x < head.x { 1 } else { -1 };
            }
        } else {
            tail.x += if tail.x < head.x { 1 } else { -1 };
            tail.y += if tail.y < head.y { 1 } else { -1 };
        }
        self.tail[tail_index] = *tail;
        if tail_index == self.tail.len() - 1 {
            self.tail_visited.insert(*tail);
        }
    }
}


pub(crate) fn run() {
    part1();
    part2();
}

pub(crate) fn part1() -> usize {
    let input = include_str!("actual.txt");
    let lines = input.lines().enumerate();

    let mut board = BoardP1 {
        head: Point { x: 0, y: 0 },
        tail: [Point { x: 0, y: 0 }],
        tail_visited: HashSet::new(),
    };
    board.tail_visited.insert(Point { x: 0, y: 0 });
    for (_idx, line) in lines {
        board.execute(line);
    }

    println!("{:?}", board.tail_visited.len());

    board.tail_visited.len()
}

pub(crate) fn part2() -> usize {
    let input = include_str!("actual.txt");
    let lines = input.lines().enumerate();

    let mut board = Board {
        head: Point { x: 0, y: 0 },
        tail: [Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }],
        tail_visited: HashSet::new(),
    };
    board.tail_visited.insert(Point { x: 0, y: 0 });
    for (_idx, line) in lines {
        board.execute(line);
    }

    println!("{:?}", board.tail_visited.len());

    board.tail_visited.len()
}
