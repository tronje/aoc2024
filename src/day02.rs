use crate::prelude::*;

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

pub struct A;

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
        Ok(reports.iter().filter(|report| is_safe(report)).count())
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
}
