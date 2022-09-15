use std::{str::FromStr, string::ParseError};

enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {}

enum Direction {
    L(i32),
    R(i32),
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);

        Ok(match direction {
            "L" => Direction::L(distance.parse().unwrap()),
            "R" => Direction::R(distance.parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

fn part_one() -> i32 {
    let directions: Vec<Direction> = include_str!("input")
        .trim()
        .split(", ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut orientation = Orientation::North;
    let (mut x, mut y) = (0, 0);

    for direction in directions {
        match direction {
            Direction::L(distance) => match orientation {
                Orientation::North => {
                    orientation = Orientation::West;
                    x -= distance;
                }
                Orientation::East => {
                    orientation = Orientation::North;
                    y -= distance;
                }
                Orientation::South => {
                    orientation = Orientation::East;
                    x += distance;
                }
                Orientation::West => {
                    orientation = Orientation::South;
                    y += distance;
                }
            },
            Direction::R(distance) => match orientation {
                Orientation::North => {
                    orientation = Orientation::East;
                    x += distance;
                }
                Orientation::East => {
                    orientation = Orientation::South;
                    y += distance;
                }
                Orientation::South => {
                    orientation = Orientation::West;
                    x -= distance;
                }
                Orientation::West => {
                    orientation = Orientation::North;
                    y -= distance;
                }
            },
        }
    }

    x.abs() + y.abs()
}

fn part_two() -> i32 {
    let directions: Vec<Direction> = include_str!("input")
        .trim()
        .split(", ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut orientation = Orientation::North;
    let (mut x, mut y) = (0, 0);

    for direction in directions {
        match direction {
            Direction::L(distance) => match orientation {
                Orientation::North => {
                    orientation = Orientation::West;
                    x -= distance;
                }
                Orientation::East => {
                    orientation = Orientation::North;
                    y -= distance;
                }
                Orientation::South => {
                    orientation = Orientation::East;
                    x += distance;
                }
                Orientation::West => {
                    orientation = Orientation::South;
                    y += distance;
                }
            },
            Direction::R(distance) => match orientation {
                Orientation::North => {
                    orientation = Orientation::East;
                    x += distance;
                }
                Orientation::East => {
                    orientation = Orientation::South;
                    y += distance;
                }
                Orientation::South => {
                    orientation = Orientation::West;
                    x -= distance;
                }
                Orientation::West => {
                    orientation = Orientation::North;
                    y -= distance;
                }
            },
        }
    }

    x.abs() + y.abs()
}

fn main() {
    println!("{}", part_two());
}
