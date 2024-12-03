use aoc_runner_derive::{aoc, aoc_generator};
use regex_automata::meta::Regex;

#[derive(Debug, Clone)]
pub enum Op {
    Do,
    DoNot,
    Mul(i64, i64),
}

fn map_op(opcode: &str, n1: Option<i64>, n2: Option<i64>) -> Op {
    match opcode.len() {
        2 => Op::Do,
        5 => Op::DoNot,
        3 => Op::Mul(n1.unwrap(), n2.unwrap()),
        _ => unreachable!(),
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Op> {
    let re = Regex::new(r"(do|don't|mul)?\((?:(\w+),(\w+)?)?\)").unwrap();
    let mut results: Vec<Op> = Vec::new();
    let caps = re.captures_iter(input);
    for cap in caps {
        let opcode = cap.get_group(1).map(|span| &input[span.start..span.end]);
        if let Some(opcode) = opcode {
            let n1 = cap.get_group(2).map(|span| {
                let s = &input[span.start..span.end];
                s.parse().unwrap()
            });
            let n2 = cap.get_group(3).map(|span| {
                let s = &input[span.start..span.end];
                s.parse().unwrap()
            });
            results.push(map_op(opcode, n1, n2))
        }
    }
    results
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Op]) -> i64 {
    let mut n: i64 = 0;
    for op in input {
        if let Op::Mul(n1, n2) = op {
            n += n1 * n2
        }
    }
    n
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Op]) -> i64 {
    let mut n: i64 = 0;
    let mut r#do = true;
    for op in input {
        match op {
            Op::Do => r#do = true,
            Op::DoNot => r#do = false,
            Op::Mul(n1, n2) => {
                if r#do {
                    n += n1 * n2
                }
            }
        }
    }
    n
}
