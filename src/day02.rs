use crate::prelude::*;

pub struct A;

impl A {
    fn is_safe(report: &[i32]) -> bool {
        let direction = report[0] > report[1];

        for i in 1..report.len() {
            let a = report[i - 1];
            let b = report[i];

            if (a > b) != direction {
                return false;
            }

            if a == b {
                return false;
            }

            if (a - b).abs() > 3 {
                return false;
            }
        }

        true
    }
}

impl Puzzle for A {
    type Input = Vec<Vec<i32>>;
    type Output = usize;

    fn example_input() -> Self::Input {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    fn example_output() -> Self::Output {
        2
    }

    fn input_file() -> &'static str {
        "inputs/day02/input"
    }

    fn parse_input<B>(&mut self, reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut reports = Vec::new();

        for line in reader.lines() {
            let mut levels = Vec::new();

            let line = line?;
            for level in line.split_whitespace() {
                levels.push(level.parse()?);
            }

            reports.push(levels)
        }

        Ok(reports)
    }

    fn solve(&mut self, reports: Self::Input) -> Result<Self::Output> {
        Ok(reports.iter().filter(|report| A::is_safe(report)).count())
    }
}

pub struct B;

impl B {
    fn is_safe(report: &[i32]) -> bool {
        if A::is_safe(report) {
            return true;
        }

        let mut v = Vec::with_capacity(report.len() - 1);

        for i in 0..report.len() {
            let first_half = &report[..i];
            let second_half = &report[i + 1..];

            v.extend_from_slice(first_half);
            v.extend_from_slice(second_half);

            if A::is_safe(&v) {
                return true;
            }

            v.clear();
        }

        false
    }
}

impl Puzzle for B {
    type Input = Vec<Vec<i32>>;
    type Output = usize;

    fn example_input() -> Self::Input {
        A::example_input()
    }

    fn example_output() -> Self::Output {
        4
    }

    fn input_file() -> &'static str {
        "inputs/day02/input"
    }

    fn parse_input<B>(&mut self, reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut a = A;
        a.parse_input(reader)
    }

    fn solve(&mut self, reports: Self::Input) -> Result<Self::Output> {
        Ok(reports.iter().filter(|report| B::is_safe(report)).count())
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
