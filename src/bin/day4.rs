#![feature(test)]

use benchtest::benchtest;
use itertools::Itertools;
use std::collections::HashMap;
use std::mem;

const INPUT: &str = include_str!(concat!("data/day", 4, ".txt"));
const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[derive(Debug)]
struct Passport {
    info: HashMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            info: HashMap::new(),
        }
    }

    fn add_from_line(&mut self, line: &str) {
        for pair in line.split_ascii_whitespace() {
            let (k, v) = pair.split(":").next_tuple().unwrap();
            self.info.insert(k.to_owned(), v.to_owned());
        }
    }

    fn has_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|field| self.info.contains_key(field.to_owned()))
    }

    fn is_valid(&self) -> bool {
        for field in REQUIRED_FIELDS {
            match self.info.get_key_value(field.to_owned()) {
                Some((k, v)) => {
                    if !Self::validate_field(k, v) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        return true;
    }

    fn validate_field(key: &str, value: &str) -> bool {
        match key {
            "byr" => match value.parse::<u64>() {
                Ok(n) => n >= 1920 && n <= 2002,
                _ => false,
            },
            "iyr" => match value.parse::<u64>() {
                Ok(n) => n >= 2010 && n <= 2020,
                _ => false,
            },
            "eyr" => match value.parse::<u64>() {
                Ok(n) => n >= 2020 && n <= 2030,
                _ => false,
            },
            "hgt" => {
                let chars = value.chars().collect::<Vec<_>>();

                match chars.as_slice() {
                    [n @ .., 'c', 'm'] => match n.into_iter().collect::<String>().parse::<u64>() {
                        Ok(n) => n >= 150 && n <= 193,
                        _ => false,
                    },
                    [n @ .., 'i', 'n'] => match n.into_iter().collect::<String>().parse::<u64>() {
                        Ok(n) => n >= 59 && n <= 76,
                        _ => false,
                    },
                    _ => false,
                }
            }
            "hcl" => {
                let chars = value.chars().collect::<Vec<_>>();

                match chars.as_slice() {
                    ['#', rest @ ..] => rest.iter().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
                    _ => false,
                }
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            "pid" => value.len() == 9 && value.parse::<u64>().is_ok(),
            _ => unreachable!(),
        }
    }

    fn is_empty(&self) -> bool {
        self.info.is_empty()
    }
}

// Try in a more functional way
fn puzzle_a(s: &str) -> usize {
    let mut passports = vec![];
    let mut passport = Passport::new();

    for line in s.trim().lines() {
        if line.is_empty() {
            // TODO: mem replace alternative?
            passports.push(mem::replace(&mut passport, Passport::new()));
        }
        passport.add_from_line(line);
    }

    if !passport.is_empty() {
        passports.push(passport);
    }

    passports
        .iter()
        .filter(|passport| passport.has_required_fields())
        .count()
}

fn puzzle_b(s: &str) -> usize {
    let mut passports = vec![];
    let mut passport = Passport::new();

    for line in s.trim().lines() {
        if line.is_empty() {
            passports.push(mem::replace(&mut passport, Passport::new()));
        }
        passport.add_from_line(line);
    }

    if !passport.is_empty() {
        passports.push(passport);
    }

    passports
        .iter()
        .filter(|passport| passport.is_valid())
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
