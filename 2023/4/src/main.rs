use std::collections::HashSet;

fn part_one() -> u64 {
    include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.split_once('|'))
        })
        .map(|(card, winning_numbers)| {
            let winners: HashSet<&str> = winning_numbers.split_whitespace().collect();
            let winnings_count = card
                .split_whitespace()
                .filter(|n| winners.contains(n))
                .count();

            if winnings_count > 0 {
                (2 as u64).pow(winnings_count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_two() -> u128 {
    let winnings: Vec<usize> = include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.split_once('|'))
        })
        .map(|(card, winning_numbers)| {
            let winning_numbers: HashSet<&str> = winning_numbers.split_whitespace().collect();

            card.split_whitespace()
                .filter(|n| winning_numbers.contains(n))
                .count()
        })
        .collect();

    let mut copies: Vec<u128> = vec![1; winnings.len()];

    for (index, &winning) in winnings.iter().enumerate() {
        if winning > 0 {
            for card in index + 1..index + winning + 1 {
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

// if winning > 0 && index + winning as usize + 1 <= copies.len() {
// }
