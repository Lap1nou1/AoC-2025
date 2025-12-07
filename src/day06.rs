use std::iter::Peekable;
use std::{io::*, iter};

enum Operations {
    Add,
    Mul,
}

impl Operations {
    fn from_char(s: char) -> Option<Operations> {
        if s == '*' {
            Some(Operations::Mul)
        } else if s == '+' {
            Some(Operations::Add)
        } else {
            None
        }
    }

    fn apply_operation(self: &Self, x: &u64, y: &u64) -> u64 {
        match self {
            Operations::Add => x + y,
            Operations::Mul => x * y,
        }
    }

    fn get_neutral(self: &Self) -> u64 {
        match self {
            Operations::Add => 0,
            Operations::Mul => 1,
        }
    }
}

fn resolve_part_one(input: &str) {
    let mut lines = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let nb_problems = lines[0].clone().count();
    let ops = lines
        .pop()
        .expect("Empty input")
        .map(|op| Operations::from_char(op.chars().next().unwrap()).expect("Invalid input"))
        .collect::<Vec<_>>();

    let values = lines
        .iter_mut()
        .map(|line| line.map(|val| (&val).parse::<u64>().expect("Not a number")));

    let mut solutions = Vec::with_capacity(nb_problems);

    for i in 0..nb_problems {
        solutions.push(ops[i].get_neutral())
    }

    for line in values {
        for (i, val) in line.enumerate() {
            solutions[i] = ops[i].apply_operation(&solutions[i], &val);
        }
    }

    let ans1 = solutions.iter().sum::<u64>();

    println!("First part answer {ans1}");
}

fn resolve_part_two(input: &str) -> () {
    // No trim because I use the remaining spaces of each line
    let mut lines = input.lines().map(|s| String::from(s)).collect::<Vec<_>>();
    let nb_lines = lines.len();

    let mut ans2 = 0;

    let mut buf = Vec::new();
    while !lines[0].is_empty() {
        let mut number = String::with_capacity(lines.len() - 1);

        for line in lines[..nb_lines - 1].iter_mut() {
            number.push(line.pop().expect("Empty line"))
        }

        buf.push(number.trim().parse::<u64>().expect("???"));

        if let Some(op) =
            Operations::from_char(lines.last_mut().expect("Empty file").pop().unwrap())
        {
            ans2 += buf
                .iter()
                .fold(op.get_neutral(), |x, y| op.apply_operation(&x, y));

            buf.clear();

            // Spill unecessary space
            lines.iter_mut().for_each(|line| {
                line.pop();
            });
        }
    }

    println!("Second part answer {ans2}");
}

pub(crate) const DAY: usize = 6;
pub fn run(input: &str) -> Result<()> {
    resolve_part_one(input);
    resolve_part_two(input);

    Ok(())
}
