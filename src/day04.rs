use crate::prelude::*;

pub struct Input {
    lines: Vec<Vec<char>>,
}

impl Input {
    fn find_up(&self, x: usize, y: usize) -> bool {
        if y < 3 {
            return false;
        }

        self.lines[y - 1][x] == 'M' && self.lines[y - 2][x] == 'A' && self.lines[y - 3][x] == 'S'
    }

    fn find_down(&self, x: usize, y: usize) -> bool {
        if y + 4 > self.lines.len() {
            return false;
        }

        self.lines[y + 1][x] == 'M' && self.lines[y + 2][x] == 'A' && self.lines[y + 3][x] == 'S'
    }

    fn find_up_left(&self, x: usize, y: usize) -> bool {
        if x < 3 || y < 3 {
            return false;
        }

        self.lines[y - 1][x - 1] == 'M'
            && self.lines[y - 2][x - 2] == 'A'
            && self.lines[y - 3][x - 3] == 'S'
    }

    fn find_up_right(&self, x: usize, y: usize) -> bool {
        if x + 4 > self.lines[0].len() || y < 3 {
            return false;
        }

        self.lines[y - 1][x + 1] == 'M'
            && self.lines[y - 2][x + 2] == 'A'
            && self.lines[y - 3][x + 3] == 'S'
    }

    fn find_down_left(&self, x: usize, y: usize) -> bool {
        if x < 3 || y + 4 > self.lines.len() {
            return false;
        }

        self.lines[y + 1][x - 1] == 'M'
            && self.lines[y + 2][x - 2] == 'A'
            && self.lines[y + 3][x - 3] == 'S'
    }

    fn find_down_right(&self, x: usize, y: usize) -> bool {
        if x + 4 > self.lines[0].len() || y + 4 > self.lines.len() {
            return false;
        }

        self.lines[y + 1][x + 1] == 'M'
            && self.lines[y + 2][x + 2] == 'A'
            && self.lines[y + 3][x + 3] == 'S'
    }

    fn count_xmas(&self) -> u32 {
        let mut count = 0;

        for (y, line) in self.lines.iter().enumerate() {
            for (x, &c) in line.iter().enumerate() {
                if c == 'X' {
                    if x >= 3 && line[x - 3..x] == ['S', 'A', 'M'] {
                        count += 1;
                    }

                    if x < line.len() - 3 && line[x + 1..x + 4] == ['M', 'A', 'S'] {
                        count += 1;
                    }

                    if self.find_up(x, y) {
                        count += 1;
                    }

                    if self.find_down(x, y) {
                        count += 1;
                    }

                    if self.find_up_left(x, y) {
                        count += 1;
                    }

                    if self.find_up_right(x, y) {
                        count += 1;
                    }

                    if self.find_down_left(x, y) {
                        count += 1;
                    }

                    if self.find_down_right(x, y) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn count_x_mas(&self) -> u32 {
        let mut count = 0;

        for (y, line) in self.lines.iter().enumerate() {
            for (x, &top_left) in line.iter().enumerate() {
                let top_right = match line.get(x + 2) {
                    Some(&top_right) => top_right,
                    None => continue,
                };

                let bottom_left = match self.lines.get(y + 2).and_then(|line| line.get(x)) {
                    Some(&bottom_left) => bottom_left,
                    None => continue,
                };

                let bottom_right = match self.lines.get(y + 2).and_then(|line| line.get(x + 2)) {
                    Some(&bottom_right) => bottom_right,
                    None => continue,
                };

                match self.lines.get(y + 1).and_then(|line| line.get(x + 1)) {
                    Some(&center) => {
                        if center != 'A' {
                            continue;
                        }
                    }
                    None => continue,
                }

                match (top_left, bottom_right) {
                    ('M', 'S') | ('S', 'M') => {}
                    _ => continue,
                }

                match (top_right, bottom_left) {
                    ('M', 'S') | ('S', 'M') => {}
                    _ => continue,
                }

                count += 1;
            }
        }

        count
    }
}

pub struct A;

impl Puzzle for A {
    type Input = Input;
    type Output = u32;

    fn example_input() -> Self::Input {
        let lines = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];

        Input { lines }
    }

    fn example_output() -> Self::Output {
        18
    }

    fn input_file() -> &'static str {
        "inputs/day04/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?.chars().collect());
        }

        Ok(Input { lines })
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        Ok(input.count_xmas())
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
        9
    }

    fn input_file() -> &'static str {
        "inputs/day04/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        A::parse_input(reader)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        Ok(input.count_x_mas())
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
