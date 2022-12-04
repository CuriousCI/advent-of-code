use std::cmp::Reverse;

fn part_one() -> u32 {
    include_str!("input")
        .split("\n\n")
        .map(|elf| elf.lines().map(str::parse::<u32>).map(Result::unwrap).sum())
        .max()
        .unwrap()
}

fn part_two() -> u32 {
    let mut elves_calories: Vec<u32> = include_str!("input")
        .split("\n\n")
        .map(|elf| elf.lines().map(str::parse::<u32>).map(Result::unwrap).sum())
        .collect();

    elves_calories.sort_by_key(|&v| Reverse(v));
    elves_calories.iter().take(3).sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
