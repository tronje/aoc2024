use crate::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule {
    earlier: u32,
    later: u32,
}

impl Rule {
    fn new(earlier: u32, later: u32) -> Self {
        Self { earlier, later }
    }
}

impl FromStr for Rule {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split('|');

        let earlier = split.next().ok_or("not a rule")?.parse()?;
        let later = split.next().ok_or("not a rule")?.parse()?;

        Ok(Self { earlier, later })
    }
}

pub struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn follows(&self, rule: &Rule) -> bool {
        let earlier = self.pages.iter().position(|&item| item == rule.earlier);
        let later = self.pages.iter().position(|&item| item == rule.later);

        match (earlier, later) {
            (Some(lower), Some(higher)) => lower < higher,
            _ => true,
        }
    }

    fn follows_all<'a, R>(&self, rules: R) -> bool
    where
        R: Iterator<Item = &'a Rule>,
    {
        for rule in rules {
            if !self.follows(rule) {
                return false;
            }
        }

        true
    }

    fn fix<'a, R>(&mut self, rules: R)
    where
        R: Iterator<Item = &'a Rule>,
    {
        for rule in rules {
            if !self.follows(rule) {
                let earlier = self
                    .pages
                    .iter()
                    .position(|&item| item == rule.earlier)
                    .unwrap();

                let later = self
                    .pages
                    .iter()
                    .position(|&item| item == rule.later)
                    .unwrap();

                self.pages.swap(earlier, later);
            }
        }
    }

    fn middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }
}

impl FromStr for Update {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut pages = Vec::new();

        for n in s.split(',') {
            pages.push(n.parse()?);
        }

        Ok(Self { pages })
    }
}

pub struct Input {
    rules: HashSet<Rule>,
    updates: Vec<Update>,
}

pub struct A;

impl Puzzle for A {
    type Input = Input;
    type Output = u32;

    fn example_input() -> Self::Input {
        let rules = vec![
            Rule::new(47, 53),
            Rule::new(97, 13),
            Rule::new(97, 61),
            Rule::new(97, 47),
            Rule::new(75, 29),
            Rule::new(61, 13),
            Rule::new(75, 53),
            Rule::new(29, 13),
            Rule::new(97, 29),
            Rule::new(53, 29),
            Rule::new(61, 53),
            Rule::new(97, 53),
            Rule::new(61, 29),
            Rule::new(47, 13),
            Rule::new(75, 47),
            Rule::new(97, 75),
            Rule::new(47, 61),
            Rule::new(75, 61),
            Rule::new(47, 29),
            Rule::new(75, 13),
            Rule::new(53, 13),
        ];

        let rules = HashSet::from_iter(rules);

        let updates = vec![
            Update {
                pages: vec![75, 47, 61, 53, 29],
            },
            Update {
                pages: vec![97, 61, 53, 29, 13],
            },
            Update {
                pages: vec![75, 29, 13],
            },
            Update {
                pages: vec![75, 97, 47, 61, 53],
            },
            Update {
                pages: vec![61, 13, 29],
            },
            Update {
                pages: vec![97, 13, 75, 29, 47],
            },
        ];

        Input { rules, updates }
    }

    fn example_output() -> Self::Output {
        143
    }

    fn input_file() -> &'static str {
        "inputs/day05/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut rules = HashSet::new();
        let mut updates = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line.contains('|') {
                rules.insert(line.parse()?);
            } else if !line.is_empty() {
                updates.push(line.parse()?);
            }
        }

        Ok(Input { rules, updates })
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let out = input
            .updates
            .iter()
            .filter(|update| update.follows_all(input.rules.iter()))
            .map(Update::middle_page)
            .sum();

        Ok(out)
    }
}

pub struct B;

impl Puzzle for B {
    type Input = Input;
    type Output = u32;

    fn example_input() -> Self::Input {
        A::example_input()
    }

    fn example_output() -> Self::Output {
        123
    }

    fn input_file() -> &'static str {
        "inputs/day05/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        A::parse_input(reader)
    }

    fn solve(&mut self, mut input: Self::Input) -> Result<Self::Output> {
        let mut sum = 0;
        for update in input
            .updates
            .iter_mut()
            .filter(|update| !update.follows_all(input.rules.iter()))
        {
            // this feels like it shouldn't work, but it does...
            while !update.follows_all(input.rules.iter()) {
                update.fix(input.rules.iter());
            }

            sum += update.middle_page();
        }

        Ok(sum)
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
