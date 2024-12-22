use crate::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
            Self::Concat => {
                let exp = right.ilog10() + 1;
                let factor = 10i32.pow(exp) as u64;

                left * factor + right
            }
        }
    }

    fn next(self) -> (Self, bool) {
        match self {
            Self::Add => (Self::Multiply, false),
            Self::Multiply => (Self::Add, true),
            _ => unreachable!(),
        }
    }

    fn next_with_concat(self) -> (Self, bool) {
        match self {
            Self::Add => (Self::Multiply, false),
            Self::Multiply => (Self::Concat, false),
            Self::Concat => (Self::Add, true),
        }
    }
}

pub struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self, operators: &[Operator]) -> bool {
        let mut out_value = self.operands[0];

        for i in 1..self.operands.len() {
            out_value = operators[i - 1].apply(out_value, self.operands[i]);
        }

        out_value == self.test_value
    }

    fn solvable(&self, with_concat: bool) -> bool {
        let mut operators = Vec::with_capacity(self.operands.len() - 1);
        for _ in 0..operators.capacity() {
            operators.push(Operator::Add);
        }

        let highest_operator = if with_concat {
            Operator::Concat
        } else {
            Operator::Multiply
        };

        loop {
            if self.evaluate(&operators) {
                return true;
            }

            if operators.iter().all(|&op| op == highest_operator) {
                break;
            }

            for op in &mut operators {
                let (next, overflow) = if with_concat {
                    op.next_with_concat()
                } else {
                    op.next()
                };

                *op = next;

                if !overflow {
                    break;
                }
            }
        }

        false
    }
}

pub struct A;

impl Puzzle for A {
    type Input = Vec<Equation>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let input_str = "190: 10 19\n\
                         3267: 81 40 27\n\
                         83: 17 5\n\
                         156: 15 6\n\
                         7290: 6 8 6 15\n\
                         161011: 16 10 13\n\
                         192: 17 8 14\n\
                         21037: 9 7 18 13\n\
                         292: 11 6 16 20"
            .to_owned();

        let reader = BufReader::new(input_str.as_bytes());
        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        3749
    }

    fn input_file() -> &'static str {
        "inputs/day07/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut equations = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut components = line.split(": ");

            let test_value = components.next().ok_or("invalid input")?.parse()?;

            let operands = components
                .next()
                .ok_or("invalid input")?
                .split(' ')
                .map(|s| s.parse())
                .collect::<std::result::Result<Vec<_>, _>>()?;

            equations.push(Equation {
                test_value,
                operands,
            });
        }

        Ok(equations)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|eq| eq.solvable(false))
            .map(|eq| eq.test_value)
            .sum())
    }
}

pub struct B;

impl Puzzle for B {
    type Input = Vec<Equation>;
    type Output = u64;

    fn example_input() -> Self::Input {
        A::example_input()
    }

    fn example_output() -> Self::Output {
        11387
    }

    fn input_file() -> &'static str {
        "inputs/day07/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        A::parse_input(reader)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .filter(|eq| eq.solvable(true))
            .map(|eq| eq.test_value)
            .sum())
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
    fn concatenation() {
        assert_eq!(Operator::Concat.apply(12, 345), 12345);
        assert_eq!(Operator::Concat.apply(4, 2), 42);
        assert_eq!(Operator::Concat.apply(1337, 420), 1337420);
    }

    #[test]
    fn b() -> Result<()> {
        let mut b = B;
        b.test_example()
    }
}
