use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::io::Lines;
use std::str::FromStr;

struct Instruction {
    amount: i32,
    from: i32,
    to: i32,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("amount", &self.amount)
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn build_stacks(stacks_str: &str) -> Stacks {
    let mut stack_lines: Vec<&str> = stacks_str.lines().collect();
    let last_stack_line = stack_lines.pop().unwrap();
    let stack_count = last_stack_line.split_ascii_whitespace().count();
    let max_stack_height = stack_lines.len();
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(max_stack_height); stack_count];
    for stack_line in stack_lines.iter().rev() {
        let stack_line_bytes = stack_line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            let byte_index = 1 + 4 * i;
            if byte_index >= stack_line_bytes.len() {
                break;
            }
            let crate_byte = stack_line_bytes[byte_index];
            if crate_byte != b' ' {
                stack.push(crate_byte as char);
            }
        }
    }
    stacks
}

fn process_instruction(instruction: &Instruction, stacks: &Stacks) -> Stacks {
    let mut new = stacks.clone();
    for i in 0..instruction.amount {
        let crate_to_move = new[instruction.from as usize].pop().unwrap();
        new[instruction.to as usize].push(crate_to_move);
    }
    return new;
}

fn process_instruction_part2(instruction: &Instruction, stacks: &Stacks) -> Stacks {
    let mut new = stacks.clone();
    let len = new[instruction.from as usize].len();

    //take last "instruction.amount" values from new[instruction.from as usize]
    let mut to_move = new[instruction.from as usize].iter().rev().take(instruction.amount as usize).cloned().collect::<Vec<char>>();
    new[instruction.from as usize].truncate(len as usize - to_move.len() as usize);

    new[instruction.to as usize].append(&mut to_move.iter().rev().cloned().collect::<Vec<char>>());
    return new;
}

pub(crate) fn run() {
    part1();
    part2();
}

pub(crate) fn part2() -> String {
    let input = include_str!("actual.txt");

    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();
    let mut instructions: Vec<Instruction> = vec![];

    for line in instructions_str.lines() {
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let amount = words[1].parse::<i32>().unwrap();
        let from = words[3].parse::<i32>().unwrap();
        let to = words[5].parse::<i32>().unwrap();

        let instruction = Instruction { amount, from: from - 1, to: to - 1 };

        instructions.push(instruction);
    }
    let mut stacks = build_stacks(stacks_str);

    for instruction in instructions {
        stacks = process_instruction_part2(&instruction, &stacks);
    }


    let last_values = stacks.iter().map(|stack| stack.last().unwrap()).collect::<Vec<&char>>();

    let solution = last_values.iter().map(|c| c.to_string()).collect::<String>();
    println!("{:?}", solution);

    solution
}

pub(crate) fn part1() -> String {
    let input = include_str!("actual.txt");

    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();
    let mut instructions: Vec<Instruction> = vec![];

    for line in instructions_str.lines() {
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let amount = words[1].parse::<i32>().unwrap();
        let from = words[3].parse::<i32>().unwrap();
        let to = words[5].parse::<i32>().unwrap();

        let instruction = Instruction { amount, from: from - 1, to: to - 1 };

        instructions.push(instruction);
    }
    let mut stacks = build_stacks(stacks_str);

    for instruction in instructions {
        stacks = process_instruction(&instruction, &stacks);
    }


    let last_values = stacks.iter().map(|stack| stack.last().unwrap()).collect::<Vec<&char>>();

    let solution = last_values.iter().map(|c| c.to_string()).collect::<String>();
    println!("{:?}", solution);

    solution
}