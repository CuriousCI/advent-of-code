use std::collections::HashSet;

fn part_one() -> u64 {
    include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.split_once('|'))
        })
        .map(|(card, winners)| {
            let winners: HashSet<&str> = winners.split_whitespace().collect();

            let wins = card
                .split_whitespace()
                .filter(|n| winners.contains(n))
                .count();

            if wins > 0 {
                (2 as u64).pow(wins as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_two() -> u64 {
    let wins: Vec<usize> = include_str!("input")
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, line)| line.split_once('|'))
        })
        .map(|(card, winners)| {
            let winners: HashSet<&str> = winners.split_whitespace().collect();

            card.split_whitespace()
                .filter(|n| winners.contains(n))
                .count()
        })
        .collect();

    let mut copies: Vec<_> = vec![1; wins.len()];

    for (id, &wins) in wins.iter().enumerate() {
        if wins > 0 {
            for next in id + 1..id + wins + 1 {
                copies[next] += copies[id];
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
