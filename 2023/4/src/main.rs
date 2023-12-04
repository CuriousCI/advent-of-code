use std::{collections::HashSet, u64};

fn part_one() -> u64 {
    include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.trim().split_once('|'))
        })
        .map(|(card, winning_numbers)| {
            let winning_numbers: HashSet<u64> = winning_numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let winnings_count = card
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .count() as u32;

            if winnings_count > 0 {
                return (2 as u64).pow(winnings_count - 1);
            }

            0
        })
        .sum()
}

fn part_two() -> u128 {
    let winnings: Vec<u128> = include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.trim().split_once('|'))
        })
        .map(|(card, winning_numbers)| {
            let card: Vec<u128> = card
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u128>().unwrap())
                .collect();

            let winning_numbers: HashSet<u128> = winning_numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            card.iter().filter(|n| winning_numbers.contains(n)).count() as u128
            // (card, winning_numbers)
        })
        .collect();

    let mut copies: Vec<u128> = vec![1; winnings.len()];

    for (index, winning) in winnings.iter().enumerate() {
        if winning > &0 && index + (*winning as usize) + 1 <= copies.len() {
            for card in index + 1..index + *winning as usize + 1 {
                copies[card] += copies[index];
            }
        }
    }

    copies.iter().sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
