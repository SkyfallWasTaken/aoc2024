use aho_corasick::AhoCorasick;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .into_iter()
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.map(|s| s.to_string()).collect())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> usize {
    /* let mut count = 0;
    let matcher = AhoCorasick::new(&["XMAS", "SAMX"]).unwrap();
    for chunk in input {
        for line in chunk {
            count += matcher.find_overlapping_iter(line).count()
        }
    }

    if input.len() == 4 {
        let first = &input[0];
        for (i, c) in first.iter().enumerate() {
            if *c != "X" {
                continue;
            }
            if i >= first.len() - 3 {
                // We can't get a diagonal at all, just skip it
                continue;
            }

            let first = c;
            let second = &input[1][i + 1];
            let third = &input[2][i + 2];
            let fourth = &input[3][i + 3];

            if (*first == "X" && *second == "M" && *third == "A" && *fourth == "S")
                || (*first == "S" && *second == "A" && *third == "M" && *fourth == "S")
            {
                count += 1
            }

            let first = c;
            let second = &input[1][i];
            let third = &input[2][i];
            let fourth = &input[3][i];

            if (*first == "X" && *second == "M" && *third == "A" && *fourth == "S")
                || (*first == "S" && *second == "A" && *third == "M" && *fourth == "X")
            {
                count += 1
            }
        }
    }
    count */
}
/*
#[aoc(day4, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> usize {
    todo!()
}
 */
