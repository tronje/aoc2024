use crate::prelude::*;
use std::collections::HashSet;
use std::io::BufReader;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Up => Self {
                x: self.x,
                y: self.y.overflowing_sub(1).0,
            },

            Orientation::Right => Self {
                x: self.x + 1,
                y: self.y,
            },

            Orientation::Down => Self {
                x: self.x,
                y: self.y + 1,
            },

            Orientation::Left => Self {
                x: self.x.overflowing_sub(1).0,
                y: self.y,
            },
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    orientation: Orientation,
}

impl Guard {
    fn step(&self) -> Position {
        self.position.step(self.orientation)
    }

    fn turn(&mut self) {
        self.orientation = self.orientation.turn();
    }
}

#[derive(Clone)]
pub struct Map {
    obstacles: HashSet<Position>,
    guard: Guard,
    visited_positions: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Map {
    fn guard_update(&mut self) {
        while self.obstacles.contains(&self.guard.step()) {
            self.guard.turn();
        }

        self.guard.position = self.guard.step();

        if self.finished() {
            return;
        }

        self.visited_positions.insert(self.guard.position);
    }

    fn finished(&self) -> bool {
        self.guard.position.x >= self.width || self.guard.position.y >= self.height
    }
}

pub struct A;

impl Puzzle for A {
    type Input = Map;
    type Output = u32;

    fn example_input() -> Self::Input {
        let str_input = "....#.....\n\
                         .........#\n\
                         ..........\n\
                         ..#.......\n\
                         .......#..\n\
                         ..........\n\
                         .#..^.....\n\
                         ........#.\n\
                         #.........\n\
                         ......#..."
            .to_owned();

        let reader = BufReader::new(str_input.as_bytes());
        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        41
    }

    fn input_file() -> &'static str {
        "inputs/day06/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut x = 0;
        let mut width = 0;
        let mut y = 0;

        let mut obstacles = HashSet::new();
        let mut guard = Guard::default();

        for line in reader.lines() {
            let row = line?;

            for c in row.chars() {
                match c {
                    '#' => {
                        obstacles.insert(Position { x, y });
                    }

                    '^' => guard.position = Position { x, y },

                    _ => {}
                }

                x += 1;

                if x > width {
                    width = x;
                }
            }

            x = 0;
            y += 1;
        }

        let mut visited_positions = HashSet::new();
        visited_positions.insert(guard.position);

        Ok(Map {
            obstacles,
            guard,
            visited_positions,
            width,
            height: y,
        })
    }

    fn solve(&mut self, mut map: Self::Input) -> Result<Self::Output> {
        while !map.finished() {
            map.guard_update();
        }

        Ok(map.visited_positions.len() as u32)
    }
}

pub struct B;

impl B {
    fn test_obstacle(&self, map: &Map, obstacle: Position) -> bool {
        let mut test_map = map.clone();
        test_map.obstacles.insert(obstacle);

        let mut guard_positions = HashSet::new();
        guard_positions.insert(test_map.guard);

        while !test_map.finished() {
            test_map.guard_update();

            if !guard_positions.insert(test_map.guard) {
                // if insert returns false, the value was already contained; this means the
                // guard visited a (position, orientation) combination a second time, so she's
                // in a loop
                return true;
            }
        }

        false
    }
}

impl Puzzle for B {
    type Input = Map;
    type Output = u32;

    fn example_input() -> Self::Input {
        A::example_input()
    }

    fn example_output() -> Self::Output {
        6
    }

    fn input_file() -> &'static str {
        "inputs/day06/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        A::parse_input(reader)
    }

    fn solve(&mut self, map: Self::Input) -> Result<Self::Output> {
        let mut count = 0;
        let mut possible_obstacles = HashSet::new();

        let mut test_map = map.clone();

        loop {
            test_map.guard_update();

            if test_map.finished() {
                break;
            }

            // can't place an obstacle in guard's starting position
            if test_map.guard.position != map.guard.position {
                possible_obstacles.insert(test_map.guard.position);
            }
        }

        for obs in possible_obstacles {
            if self.test_obstacle(&map, obs) {
                count += 1;
            }
        }

        Ok(count)
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
