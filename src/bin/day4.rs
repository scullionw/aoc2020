#![feature(test)]

use benchtest::benchtest;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!("data/day", 4, ".txt"));
const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[derive(Debug)]
struct Passport {
    info: HashMap<String, String>,
}

impl Passport {
    fn new(lines: &[&str]) -> Self {
        let mut info = HashMap::new();

        for line in lines {
            for pair in line.split_ascii_whitespace() {
                let (k, v) = pair.split(":").next_tuple().unwrap();
                info.insert(k.to_owned(), v.to_owned());
            }
        }

        Self { info }
    }

    fn fields_present(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|field| self.info.contains_key(field.to_owned()))
    }

    fn fields_valid(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|field| {
            self.info
                .get_key_value(field.to_owned())
                .map_or(false, |(k, v)| Self::validate_field(k, v))
        })
    }

    fn validate_field(key: &str, value: &str) -> bool {
        match key {
            "byr" => matches!(value.parse(), Ok(1920..=2002)),
            "iyr" => matches!(value.parse(), Ok(2010..=2020)),
            "eyr" => matches!(value.parse(), Ok(2020..=2030)),
            "hgt" => match value.chars().collect_vec().as_slice() {
                [n @ .., 'c', 'm'] => {
                    matches!(n.iter().collect::<String>().parse(), Ok(150..=193))
                }
                [n @ .., 'i', 'n'] => {
                    matches!(n.iter().collect::<String>().parse(), Ok(59..=76))
                }
                _ => false,
            },
            "hcl" => match value.chars().collect_vec().as_slice() {
                ['#', rest @ ..] => rest.iter().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
                _ => false,
            },
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            "pid" => value.len() == 9 && value.parse::<u64>().is_ok(),
            _ => unreachable!(),
        }
    }
}

fn puzzle_a(s: &str) -> usize {
    let lines = s.trim().lines().collect_vec();
    let groups = lines.as_slice().split(|l| l.is_empty());
    groups
        .map(Passport::new)
        .filter(Passport::fields_present)
        .count()
}

fn puzzle_b(s: &str) -> usize {
    let lines = s.trim().lines().collect_vec();
    let groups = lines.as_slice().split(|l| l.is_empty());
    groups
        .map(Passport::new)
        .filter(Passport::fields_valid)
        .count()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 196,
    puzzle_b_test: puzzle_b(INPUT) => 114
}
