use std::cmp::Reverse;

fn part_one() -> u32 {
    include_str!("input")
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<u32>).sum())
        .max()
        .unwrap_or(0)
}

fn part_two() -> u32 {
    let mut elves_calories: Vec<u32> = include_str!("input")
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<u32>).sum())
        .collect();

    elves_calories.sort_by_key(|&v| Reverse(v));
    elves_calories.iter().take(3).sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
