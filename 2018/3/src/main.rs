use std::cmp::{max, min};
use std::vec;
use std::{str::FromStr, string::ParseError};

struct Claim {
    _id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rectangle) = s.split_once(" @ ").unwrap();
        let (coordinates, size) = rectangle.split_once(": ").unwrap();
        let (x, y) = coordinates.split_once(",").unwrap();
        let (width, height) = size.split_once("x").unwrap();

        Ok(Self {
            _id: id[1..].parse().unwrap(),
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
        })
    }
}

fn part_one() -> i32 {
    let claims: Vec<Claim> = include_str!("example")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let (start, end) = claims.iter().fold((i32::MAX, 0), |(start, end), claim| {
        (min(start, claim.x), max(end, claim.x + claim.width))
    });

    let area = 0;

    for x in start..end {
        let mut intervals: Vec<(i32, i32)> = claims
            .iter()
            .filter(|claim| x >= claim.x && x < claim.x + claim.width)
            .map(|claim| (claim.y, claim.y + claim.height - 1))
            .collect();

        if intervals.len() > 1 {
            intervals.sort();

            let _intersections: Vec<(i32, i32)>;
        }
    }

    area
}

fn main() {
    println!("{}", part_one());
}
