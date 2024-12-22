use crate::prelude::*;
use std::str::Chars;

#[derive(Clone, Copy)]
struct Mul {
    left: u32,
    right: u32,
}

impl Mul {
    fn compute(self) -> u32 {
        self.left * self.right
    }
}

pub struct A;

impl Puzzle for A {
    type Input = String;
    type Output = u32;

    fn example_input() -> Self::Input {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned()
    }

    fn example_output() -> Self::Output {
        161
    }

    fn input_file() -> &'static str {
        "inputs/day03/input"
    }

    fn parse_input<B>(&mut self, mut reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut out = String::new();
        reader.read_to_string(&mut out)?;
        Ok(out)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let parser = Parser::new(input.chars());
        let out = parser.map(|instruction| instruction.compute()).sum();

        Ok(out)
    }
}

pub struct B;

impl Puzzle for B {
    type Input = String;
    type Output = u32;

    fn example_input() -> Self::Input {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".into()
    }

    fn example_output() -> Self::Output {
        48
    }

    fn input_file() -> &'static str {
        "inputs/day03/input"
    }

    fn parse_input<B>(&mut self, reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut a = A;
        a.parse_input(reader)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut out = 0;
        let mut split = input.split("don't()");

        // gather all until first "don't()" instruction
        if let Some(substring) = split.next() {
            let parser = Parser::new(substring.chars());

            for mul in parser {
                out += mul.compute();
            }
        }

        for substring in split {
            for subsubstring in substring.split("do()").skip(1) {
                let parser = Parser::new(subsubstring.chars());

                for mul in parser {
                    out += mul.compute();
                }
            }
        }

        Ok(out)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    M,
    U,
    L,
    OpenParen,
    ClosingParen,
    Comma,
    Digit(u32),
}

impl Token {
    fn follows(self, other: Self) -> bool {
        use Token::*;

        matches!(
            (other, self),
            (M, U)
                | (U, L)
                | (L, OpenParen)
                | (OpenParen, Digit(_))
                | (Digit(_), Digit(_))
                | (Digit(_), Comma)
                | (Comma, Digit(_))
                | (Digit(_), ClosingParen)
        )
    }
}

impl TryFrom<char> for Token {
    type Error = &'static str;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'm' => Ok(Self::M),
            'u' => Ok(Self::U),
            'l' => Ok(Self::L),
            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::ClosingParen),
            ',' => Ok(Self::Comma),
            other => match other.to_digit(10) {
                Some(digit) => Ok(Self::Digit(digit)),
                None => Err("unrecognized token"),
            },
        }
    }
}

struct Parser<'a> {
    input: Chars<'a>,
    left: u32,
    right: u32,
    comma_seen: bool,
    last_token: Token,
}

impl<'a> Parser<'a> {
    fn new(input: Chars<'a>) -> Self {
        Self {
            input,
            left: 0,
            right: 0,
            comma_seen: false,
            last_token: Token::ClosingParen,
        }
    }

    fn reset(&mut self) {
        self.left = 0;
        self.right = 0;
        self.comma_seen = false;
        self.last_token = Token::ClosingParen;
    }
}

impl Iterator for Parser<'_> {
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.input.next()?;

            let token = match Token::try_from(c) {
                Ok(token) => token,
                Err(_) => {
                    self.reset();
                    continue;
                }
            };

            if token == Token::M {
                self.reset();
                self.last_token = token;
                continue;
            }

            if !token.follows(self.last_token) {
                self.reset();
                continue;
            }

            match token {
                Token::ClosingParen => {
                    let mul = Mul {
                        left: self.left,
                        right: self.right,
                    };

                    self.reset();
                    return Some(mul);
                }

                Token::Comma => self.comma_seen = true,

                Token::Digit(digit) => {
                    if self.comma_seen {
                        self.right *= 10;
                        self.right += digit;
                    } else {
                        self.left *= 10;
                        self.left += digit;
                    }
                }

                _ => {}
            }

            self.last_token = token;
        }
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
