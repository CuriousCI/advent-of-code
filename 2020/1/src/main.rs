fn part_one() -> u32 {
    let numbers: Vec<u32> = include_str!("input")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for f1 in 0..numbers.len() {
        for f2 in f1 + 1..numbers.len() {
            if numbers[f1] + numbers[f2] == 2020 {
                return numbers[f1] * numbers[f2];
            }
        }
    }

    0
}

fn part_two() -> u32 {
    let numbers: Vec<u32> = include_str!("input")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for f1 in 0..numbers.len() {
        for f2 in f1 + 1..numbers.len() {
            for f3 in f2 + 1..numbers.len() {
                if numbers[f1] + numbers[f2] + numbers[f3] == 2020 {
                    return numbers[f1] * numbers[f2] * numbers[f3];
                }
            }
        }
    }

    0
}

fn main() {
    println!("{}", part_two());
}
