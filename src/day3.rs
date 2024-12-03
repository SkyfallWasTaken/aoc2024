use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;

#[derive(Debug, Clone)]
pub enum Op {
    Do,
    DoNot,
    Mul(i64, i64),
}

fn map_op(opcode: &str, n1: Option<i64>, n2: Option<i64>) -> Op {
    match opcode {
        "do" => Op::Do,
        "don't" => Op::DoNot,
        "mul" => Op::Mul(n1.unwrap(), n2.unwrap()),
        _ => {
            unreachable!()
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Op> {
    let re = Regex::new(r"(do|don't|mul)?\((?:(\w+),(\w+)?)?\)").unwrap();
    let mut results: Vec<Op> = Vec::new();
    let matches = re.captures_iter(input);
    for r#match in matches.map(|m| m.unwrap()) {
        let opcode = r#match.get(1).map(|m| m.as_str());
        if let Some(opcode) = opcode {
            results.push(map_op(
                opcode,
                r#match.get(2).map(|m| m.as_str().parse().unwrap()),
                r#match.get(3).map(|m| m.as_str().parse().unwrap()),
            ))
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
