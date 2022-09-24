fn part_one() -> u32 {
    include_str!("input")
        .trim()
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.map(str::parse).map(Result::unwrap).collect())
        .map(|numbers: Vec<u32>| numbers.iter().max().unwrap() - numbers.iter().min().unwrap())
        .sum()
}

fn part_two() -> u32 {
    include_str!("input")
        .trim()
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.map(str::parse).map(Result::unwrap).collect())
        .map(|mut numbers: Vec<u32>| {
            numbers.sort_by_key(|n| std::cmp::Reverse(*n));

            for dividend in 0..numbers.len() {
                for divisor in dividend + 1..numbers.len() {
                    if numbers[dividend] % numbers[divisor] == 0 {
                        return (numbers[dividend] / numbers[divisor]) as u32;
                    }
                }
            }

            0
        })
        .sum()
}

fn main() {
    println!("{}", part_two());
}
