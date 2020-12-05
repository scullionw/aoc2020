#![allow(unused_variables)]
#![feature(test)]

use benchtest::benchtest;

const INPUT: &str = include_str!(concat!("data/day", 3, ".txt"));

struct Grid {
    data: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let data = s
            .trim()
            .lines()
            .map(|line| Grid::process_line(line))
            .collect::<Vec<_>>();

        let width = data[0].as_slice().len();
        let height = data.as_slice().len();

        Self {
            data,
            height,
            width,
        }
    }

    fn process_line(line: &str) -> Vec<bool> {
        line.chars().map(|c| c == '#').collect()
    }

    fn slide(&self, right: usize, down: usize) -> u64 {
        let (mut i, mut j) = (0, 0);
        let mut tree_count = 0;
        loop {
            i += down;
            j += right;

            if i >= self.height {
                break tree_count;
            }

            if self.data[i][j % self.width] {
                tree_count += 1
            }
        }
    }
}

fn puzzle_a(s: &str) -> u64 {
    let grid = Grid::new(s);
    grid.slide(3, 1)
}

fn puzzle_b(s: &str) -> u64 {
    let grid = Grid::new(s);

    let trips = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    trips
        .iter()
        .map(|&(right, down)| grid.slide(right, down))
        .product()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 189,
    puzzle_b_test: puzzle_b(INPUT) => 1718180100
}
