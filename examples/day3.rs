use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashMap;
use std::str::FromStr;
use std::ops::Add;

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let input_file = File::open("data/day3")?;
    let input_reader = BufReader::new(input_file);

    let mut grid = Grid::default();
    let mut min_intersection = 0;
    for (wire_id, wire) in input_reader.lines().enumerate() {
        let mut pos = central_port();
        for instr in wire?.split(',') {
            let dir: Direction = instr[0..1].parse()?;
            let steps: usize = instr[1..].parse()?;
            for _ in 0..steps {
                pos = pos + dir;
                if let Some(_) = grid.place(pos, wire_id) {
                    min_intersection = min_intersection.min(pos.x.abs() + pos.y.abs());
                }
            }
        }
    }

    println!("{}", grid);

    println!("Closest intersection distance: {}", min_intersection);

    Ok(())
}

fn central_port() -> GridPos {
    GridPos {x: 0, y: 0}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidDirection;

impl Error for InvalidDirection {}

impl fmt::Display for InvalidDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid direction")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = InvalidDirection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s {
            "U" => Up,
            "D" => Down,
            "R" => Right,
            "L" => Left,
            _ => return Err(InvalidDirection),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i64,
    pub y: i64,
}

impl Add<Direction> for GridPos {
    type Output = GridPos;

    fn add(self, other: Direction) -> Self {
        let GridPos {x, y} = self;

        use Direction::*;
        match other {
            Up => GridPos {x, y: y + 1},
            Down => GridPos {x, y: y - 1},
            Right => GridPos {x: x + 1, y},
            Left => GridPos {x: x - 1, y},
        }
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    /// The values in the grid with a piece of wire with the given wire ID
    tiles: HashMap<GridPos, usize>,
}

impl Grid {
    /// Places a wire with the given ID
    ///
    /// Returns Some(other wire ID) if a wire with a different ID exists at this location already
    pub fn place(&mut self, pos: GridPos, wire_id: usize) -> Option<usize> {
        match self.tiles.get(&pos) {
            Some(&wire2_id) if wire_id == wire2_id => None,
            Some(&wire2_id) => Some(wire2_id),
            None => {
                self.tiles.insert(pos, wire_id);
                None
            },
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let init_range = 0..0;
        let (x_range, y_range) = self.tiles.keys().fold((init_range.clone(), init_range), |(mut x_range, mut y_range), pos| {
            x_range.start = x_range.start.min(pos.x);
            x_range.end = x_range.end.max(pos.x+1);
            y_range.start = y_range.start.min(pos.x);
            y_range.end = y_range.end.max(pos.x+1);
            (x_range, y_range)
        });

        for y in y_range.rev() {
            for x in x_range.clone() {
                let pos = GridPos {x, y};
                match self.tiles.get(&pos) {
                    Some(wire_id) => write!(f, "{}", wire_id)?,
                    None => write!(f, ".")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
