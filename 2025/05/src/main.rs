use std::collections::HashSet;

fn part_one(input: &str) -> usize {
    let (fresh_ingredients_ranges, ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ingredients_ranges: Vec<(u64, u64)> = fresh_ingredients_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    ingredients
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .filter(|ingredient| {
            fresh_ingredients_ranges
                .iter()
                .any(|(start, end)| (start..=end).contains(&ingredient))
        })
        .count()
}

fn part_two(input: &str) -> u64 {
    let (fresh_ingredients_ranges, _) = input.split_once("\n\n").unwrap();

    let mut fresh_ingredients_ranges: Vec<(u64, u64)> = fresh_ingredients_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    fresh_ingredients_ranges.sort_unstable();

    let mut merged_fresh_ingredients_ranges: HashSet<(u64, u64)> = HashSet::new();

    for (mut start, mut end) in fresh_ingredients_ranges {
        while let Some(&(old_start, old_end)) =
            merged_fresh_ingredients_ranges
                .iter()
                .find(|(old_start, old_end)| {
                    (old_start..=old_end).contains(&&start) || (old_start..=old_end).contains(&&end)
                })
        {
            merged_fresh_ingredients_ranges.remove(&(old_start, old_end));
            start = old_start.min(start);
            end = old_end.max(end);
        }

        merged_fresh_ingredients_ranges.insert((start, end));
    }

    merged_fresh_ingredients_ranges
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
