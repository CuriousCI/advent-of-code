use std::str::FromStr;

struct Present {
    l: i32,
    w: i32,
    h: i32,
}

impl Present {
    fn wrapping_paper(self) -> i32 {
        let (l, w, h) = (self.l, self.w, self.h);

        2 * w * h + 2 * w * l + 2 * l * h + vec![l * w, l * h, w * h].iter().min().unwrap()
    }

    fn ribbon(self) -> i32 {
        let (l, w, h) = (self.l, self.w, self.h);

        let mut sides = vec![l, w, h];
        sides.sort();
        sides[0] * 2 + sides[1] * 2 + l * w * h
    }
}

impl FromStr for Present {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dimensions: Vec<i32> = s.split('x').map(str::parse).map(Result::unwrap).collect();

        Ok(Self {
            l: dimensions[0],
            w: dimensions[1],
            h: dimensions[2],
        })
    }
}

fn part_one() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|present: Present| present.wrapping_paper())
        .sum()
}

fn part_two() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|present: Present| present.ribbon())
        .sum()
}

fn main() {
    println!("{}", part_two());
}
