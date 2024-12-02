use aoc_runner_derive::{aoc, aoc_generator};
use owo_colors::OwoColorize;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|line| line.iter().map(|n| n.parse::<i32>().unwrap()).collect())
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
enum Op {
    Incr,
    Decr,
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let mut safe_count = 0;

    for report in input {
        if check_report(report) {
            /*             println!("{:?} is {}", report, "safe".green().bold());
             */
            safe_count += 1
        } else {
            /*             println!("{:?} is {}", report, "unsafe".red().bold());
             */
        }
    }

    safe_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    let mut safe_count = 0;

    for report in input {
        let mut safe = false;
        'report_iter: for n in 0..report.len() {
            let mut report = report.clone();
            report.remove(n);
            if check_report(&report) {
                safe = true;
                break 'report_iter;
            }
        }
        if safe {
            safe_count += 1
        }
    }

    safe_count
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut mode: Option<Op> = None;
    let mut safe = true;
    for window in report.windows(2).map(|window| (window[0], window[1])) {
        let n = window.0 - window.1;
        let mode = mode
            .get_or_insert_with(|| if n > 0 { Op::Decr } else { Op::Incr })
            .clone();
        if (mode == Op::Incr && n > 0) || (mode == Op::Decr && n < 0) || n > 3 || n < -3 || n == 0 {
            safe = false;
            break;
        }
    }

    safe
}
