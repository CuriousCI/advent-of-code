fn part_one() -> u32 {
    include_str!("input")
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum()
        })
        .max()
        .unwrap()
}

fn part_two() -> u32 {
    let mut elves_calories: Vec<u32> = include_str!("input")
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum()
        })
        .collect();

    elves_calories.sort();
    elves_calories.reverse();
    elves_calories.iter().take(3).sum()
}

fn main() {
    println!("{}", part_two())
}
