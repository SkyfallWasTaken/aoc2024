use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("   ").map(|x| x.parse::<i32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();
    left.sort();
    right.sort();
    let input = left.iter().zip(right.iter()).collect::<Vec<_>>();
    dbg!(&input[0..5]);

    let mut n = 0;
    for pair in input {
        n += (pair.0 - pair.1).abs()
    }
    n
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();
    let mut score: usize = 0;
    for id in left {
        score += id as usize * right.iter().filter(|n| n == &&id).count()
    }
    score
}
