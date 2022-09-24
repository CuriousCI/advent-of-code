fn part_one() -> u32 {
    let numbers: Vec<u32> = include_str!("input")
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .map(Option::unwrap)
        .collect();

    numbers
        .iter()
        .fold((numbers.last().unwrap(), 0), |(prev, sum), number| {
            (number, sum + if number == prev { *prev } else { 0 })
        })
        .1
}

fn part_two() -> u32 {
    let numbers: Vec<u32> = include_str!("input")
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .map(Option::unwrap)
        .collect();

    let mut total = 0;
    let offset = numbers.len() / 2;
    for index in 0..numbers.len() {
        if numbers[index] == numbers[(index + offset) % numbers.len()] {
            total += numbers[index];
        }
    }

    total
}

fn main() {
    println!("{}", part_two());
}
