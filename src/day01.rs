use crate::{Puzzle, Result};
use std::{collections::HashMap, io::BufRead};

pub struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

pub struct A;

impl Puzzle for A {
    type Input = Input;
    type Output = i32;

    fn example_input() -> Self::Input {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        Input { left, right }
    }

    fn example_output() -> Self::Output {
        11
    }

    fn input_file() -> &'static str {
        "inputs/day01/input"
    }

    fn parse_input<B: BufRead>(&mut self, reader: B) -> Result<Self::Input> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut split = line.split_whitespace();

            let n = split.next().ok_or("invalid input")?.parse()?;
            left.push(n);

            let n = split.next().ok_or("invalid input")?.parse()?;
            right.push(n);
        }

        Ok(Input { left, right })
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let Input {
            mut left,
            mut right,
        } = input;

        left.sort();
        right.sort();

        Ok(left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum())
    }
}

pub struct B;

impl Puzzle for B {
    type Input = Input;
    type Output = i32;

    fn example_input() -> Self::Input {
        A::example_input()
    }

    fn example_output() -> Self::Output {
        31
    }

    fn input_file() -> &'static str {
        "inputs/day01/input"
    }

    fn parse_input<B: BufRead>(&mut self, reader: B) -> Result<Self::Input> {
        let mut a = A;
        a.parse_input(reader)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let Input { left, right } = input;

        let mut counts = HashMap::new();

        for x in left.iter() {
            let count = right.iter().filter(|&y| y == x).count() as i32;
            *counts.entry(x).or_insert(0) += x * count;
        }

        Ok(counts.values().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> Result<()> {
        let mut a = A;
        a.test_example()
    }

    #[test]
    fn b() -> Result<()> {
        let mut b = B;
        b.test_example()
    }
}
