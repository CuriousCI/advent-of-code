use std::{str::FromStr, string::ParseError, vec};

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
        // #123 @ 45,56: 213x18

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

    let (fabric_x, fabric_width) = claims.iter().fold((std::i32::MAX, 0), |mut fabric, claim| {
        if claim.x < fabric.0 {
            fabric.0 = claim.x;
        }

        if claim.x + claim.width > fabric.1 {
            fabric.1 = claim.x + claim.width;
        }

        fabric
    });

    let mut area = 0;
    for x in fabric_x..=fabric_width {
        let mut intervals: Vec<(i32, i32)> = claims
            .iter()
            .map(|claim| {
                if x > claim.x && x < claim.x + claim.width {
                    return (claim.y, claim.y + claim.height);
                }

                (0, 0)
            })
            .filter(|interval| {
                return interval.0 != 0 && interval.1 != 0;
            })
            .collect();

        if intervals.len() > 0 {
            intervals.sort();

            let first_interval = 0;

            for index in 1..intervals.len() {}
        }
    }

    area
}

fn main() {
    println!("{}", part_one());
}
