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

    fn count(&self) -> u32 {
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

    fn parse_input<B>(&mut self, reader: B) -> Result<Self::Input>
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
        Ok(input.count())
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
