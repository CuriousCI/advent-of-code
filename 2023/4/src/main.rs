use std::{collections::HashSet, u64};

fn part_one() -> u64 {
    include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.trim().split_once('|'))
        })
        .map(|(card, winning_numbers)| {
            let card = card
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap());

            let winning_numbers: HashSet<u64> = winning_numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mut winning_count = 0;

            for number in card {
                if winning_numbers.contains(&number) {
                    winning_count += 1;
                }
            }

            if winning_count > 0 {
                return (2 as u64).pow(winning_count - 1);
            }

            0
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
}
