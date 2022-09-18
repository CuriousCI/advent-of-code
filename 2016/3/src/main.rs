use std::{str::FromStr, string::ParseError};

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    fn is_valid(self) -> bool {
        let (a, b, c) = (self.a, self.b, self.c);

        a + b > c && a + c > b && b + c > a
    }
}

impl FromStr for Triangle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides: Vec<i32> = s
            .split(' ')
            .filter(|item| *item != "")
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Ok(Self {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        })
    }
}

fn part_one() -> usize {
    include_str!("input")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(Triangle::is_valid)
        .filter(|is_valid| *is_valid)
        .count()
}

fn main() {
    println!("{}", part_one());
}
