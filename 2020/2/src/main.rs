use std::{str::FromStr, string::ParseError};

struct Password {
    lower_bound: u32,
    upper_bound: u32,
    letter: String,
    password: String,
}

impl Password {
    fn is_valid_one(&self) -> bool {
        let count: u32 = self
            .password
            .chars()
            .filter(|letter| letter.to_string() == self.letter)
            .count() as u32;

        count >= self.lower_bound && count <= self.upper_bound
    }

    fn is_valid(&self) -> bool {
        let letters: Vec<String> = self
            .password
            .chars()
            .map(|letter| letter.to_string())
            .collect();

        let (left, right) = (
            (*letters.get(self.lower_bound as usize - 1).unwrap()).clone(),
            (*letters.get(self.upper_bound as usize - 1).unwrap()).clone(),
        );

        !(left == right || (left != self.letter && right != self.letter))
    }
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule, password) = s.split_once(": ").unwrap();
        let (bounds, letter) = rule.split_once(" ").unwrap();
        let (lower, upper) = bounds.split_once('-').unwrap();

        Ok(Self {
            lower_bound: lower.parse().unwrap(),
            upper_bound: upper.parse().unwrap(),
            password: String::from(password),
            letter: String::from(letter),
        })
    }
}

fn part_one() -> u32 {
    include_str!("input")
        .trim()
        .lines()
        .map(str::parse::<Password>)
        .map(Result::unwrap)
        .filter(Password::is_valid_one)
        .count() as u32
}

fn part_two() -> u32 {
    include_str!("input")
        .trim()
        .lines()
        .map(str::parse::<Password>)
        .map(Result::unwrap)
        .filter(Password::is_valid)
        .count() as u32
}

fn main() {
    println!("{}", part_two());
}
