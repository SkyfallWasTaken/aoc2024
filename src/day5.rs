use aoc_runner_derive::{aoc, aoc_generator};

pub type Rule = (i32, i32);

#[derive(Clone)]
pub struct Build {
    pub rules: Vec<Rule>,
    pub updates: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Build {
    let mut split = input.split("\n\n");
    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split('|');
            let n1 = split.next().unwrap().parse().unwrap();
            let n2 = split.next().unwrap().parse().unwrap();
            (n1, n2)
        })
        .collect();

    let updates = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    Build { rules, updates }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Build) -> i32 {
    let mut middle_num = 0;
    'update: for update in &input.updates {
        for (i, n) in update.iter().enumerate() {
            for (n1, n2) in &input.rules {
                if n1 == n {
                    // After
                    if let Some(other_pos) = update.iter().position(|x| x == n2) {
                        if other_pos < i {
                            continue 'update;
                        }
                    }
                } else if n2 == n {
                    if let Some(other_pos) = update.iter().position(|x| x == n2) {
                        if other_pos > i {
                            continue 'update;
                        }
                    }
                }
            }
        }
        // Valid
        let midpoint = update.len().div_ceil(2);
        middle_num += update[midpoint - 1];
    }
    middle_num
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Build) -> usize {
    todo!()
}
