use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::{Captures, Regex};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"mul\(-?(\w+),-?(\w+)\)").unwrap();
    let mut results: Vec<(i64, i64)> = Vec::new();
    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        let tup = (n1.parse().unwrap(), n2.parse().unwrap());
        results.push(tup);
    }
    results
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[(i64, i64)]) -> i64 {
    let mut n: i64 = 0;
    for (n1, n2) in input {
        dbg!((n1, n2));
        n += n1 * n2
    }
    n
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[(i64, i64)]) -> i64 {
    todo!()
}
