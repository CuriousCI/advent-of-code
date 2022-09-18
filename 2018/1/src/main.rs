use std::collections::HashSet;

fn part_one() -> i32 {
    include_str!("input")
        .trim()
        .split('\n')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .sum()
}

fn part_two() -> i32 {
    let changes: Vec<i32> = include_str!("input")
        .trim()
        .split('\n')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut frequency = 0;
    let mut frequencies: HashSet<i32> = HashSet::from([frequency]);

    loop {
        for change in &changes {
            frequency += change;

            if frequencies.contains(&frequency) {
                return frequency;
            }

            frequencies.insert(frequency);
        }
    }
}

fn main() {
    println!("{}", part_two());
}
