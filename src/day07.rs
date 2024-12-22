use crate::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
        }
    }
}

struct Operators(u32);

impl Operators {
    fn get(&self, idx: usize) -> Operator {
        let shift = idx as u32;
        if (self.0 >> shift) & 1 == 1 {
            Operator::Multiply
        } else {
            Operator::Add
        }
    }
}

pub struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self, operators: Operators) -> bool {
        let mut out_value = self.operands[0];

        for i in 1..self.operands.len() {
            out_value = operators.get(i - 1).apply(out_value, self.operands[i]);
        }

        out_value == self.test_value
    }

    fn solvable(&self) -> bool {
        let operator_combinations = 2u32.pow(self.operands.len() as u32 - 1);

        for op_code in 0..operator_combinations {
            let operators = Operators(op_code);
            if self.evaluate(operators) {
                return true;
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
            .filter(|eq| eq.solvable())
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
}
