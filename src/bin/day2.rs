#![feature(test)]

#[macro_use]
extern crate scan_fmt;

use benchtest::benchtest;

const INPUT: &str = include_str!(concat!("data/day", 2, ".txt"));

fn parse(line: &str) -> (usize, usize, char, String) {
    scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String).unwrap()
}

fn puzzle_a(s: &str) -> u64 {
    let mut valid_count = 0;
    for line in s.trim().lines() {
        let (lo, hi, letter, password) = parse(line);
        let count = password.chars().filter(|&c| c == letter).count();
        if (lo..=hi).contains(&count) {
            valid_count += 1;
        }
    }
    valid_count
}

fn puzzle_b(s: &str) -> u64 {
    let mut valid_count = 0;
    for line in s.trim().lines() {
        let (lo, hi, letter, password) = parse(line);

        if (password.chars().nth(lo - 1).unwrap() == letter)
            != (password.chars().nth(hi - 1).unwrap() == letter)
        {
            valid_count += 1;
        }
    }
    valid_count
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 643,
    puzzle_b_test: puzzle_b(INPUT) => 388
}
