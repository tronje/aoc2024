use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    y: i32, // must be first to derive a working PartialOrd
    x: i32,
}

impl Position {
    fn distance(self, other: Self) -> (i32, i32) {
        let hi = self.max(other);
        let lo = self.min(other);

        (hi.x - lo.x, hi.y - lo.y)
    }
}

pub struct Map {
    antennas: HashMap<Position, char>,
    width: i32,
    height: i32,
}

impl Map {
    fn contains(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width) && (y >= 0 && y < self.height)
    }

    fn antinode_positions(&self, a: Position, b: Position) -> [Option<Position>; 2] {
        let (x_dist, y_dist) = a.distance(b);
        let mut out = [None, None];

        let hi = a.max(b);
        let lo = a.min(b);

        let lo_x = lo.x - x_dist;
        let lo_y = lo.y - y_dist;

        if self.contains(lo_x, lo_y) {
            out[0] = Some(Position { x: lo_x, y: lo_y });
        }

        let hi_x = hi.x + x_dist;
        let hi_y = hi.y + y_dist;

        if self.contains(hi_x, hi_y) {
            out[1] = Some(Position { x: hi_x, y: hi_y });
        }

        out
    }
}

pub struct A;

impl Puzzle for A {
    type Input = Map;
    type Output = usize;

    fn example_input() -> Self::Input {
        let input_str = "............\n\
                         ........0...\n\
                         .....0......\n\
                         .......0....\n\
                         ....0.......\n\
                         ......A.....\n\
                         ............\n\
                         ............\n\
                         ........A...\n\
                         .........A..\n\
                         ............\n\
                         ............"
            .to_owned();

        let reader = BufReader::new(input_str.as_bytes());
        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        14
    }

    fn input_file() -> &'static str {
        "inputs/day08/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut antennas = HashMap::new();
        let mut width = 0;

        let mut x = 0;
        let mut y = 0;

        for line in reader.lines() {
            let row = line?;

            for c in row.chars() {
                if c != '.' {
                    antennas.insert(Position { x, y }, c);
                }

                x += 1;

                if x > width {
                    width = x;
                }
            }

            x = 0;
            y += 1;
        }

        Ok(Map {
            antennas,
            width,
            height: y,
        })
    }

    fn solve(&mut self, map: Self::Input) -> Result<Self::Output> {
        let mut antinodes = HashSet::new();
        let mut antenna_pairs = HashSet::new();

        for (pos_a, ty_a) in &map.antennas {
            for (pos_b, ty_b) in &map.antennas {
                if ty_a != ty_b || pos_a == pos_b {
                    continue;
                }

                if !antenna_pairs.contains(&(*pos_a, *pos_b))
                    && !antenna_pairs.contains(&(*pos_b, *pos_a))
                {
                    antenna_pairs.insert((*pos_a, *pos_b));
                }
            }
        }

        for (a, b) in antenna_pairs {
            for antinode in map.antinode_positions(a, b).into_iter().flatten() {
                antinodes.insert(antinode);
            }
        }

        Ok(antinodes.len())
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
