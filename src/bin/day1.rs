#![feature(test)]

use benchtest::benchtest;

const INPUT: &str = include_str!("data/day1.txt");

fn process(s: &str) -> Vec<u64> {
    s.trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn puzzle_a(s: &str) -> u64 {
    let nums = process(s);

    let mut solution = None;

    for a in &nums {
        for b in &nums {
            if a + b == 2020 {
                solution = Some(a * b);
            }
        }
    }

    solution.unwrap()
}

fn puzzle_b(s: &str) -> u64 {
    let nums = process(s);

    let mut solution = None;

    for a in &nums {
        for b in &nums {
            for c in &nums {
                if a + b + c == 2020 {
                    solution = Some(a * b * c);
                }
            }
        }
    }

    solution.unwrap()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 485739,
    puzzle_b_test: puzzle_b(INPUT) => 161109702
}
