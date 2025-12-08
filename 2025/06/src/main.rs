fn part_one(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();

    let numbers: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(index, operator)| match operator {
            "+" => numbers.iter().map(|line| line[index]).sum(),
            "*" => numbers.iter().map(|line| line[index]).product(),
            _ => 0,
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    let mut current_numbers = vec![];
    for col in (0..lines[0].len()).rev() {
        let mut number: u64 = 0;
        for line in &lines[..lines.len() - 1] {
            if let Some(digit) = line[col].to_digit(10) {
                number *= 10;
                number += digit as u64;
            }
        }

        if number != 0 {
            current_numbers.push(number);
        }

        match lines[lines.len() - 1][col] {
            '+' => {
                total += current_numbers.iter().sum::<u64>();
                current_numbers.clear();
            }
            '*' => {
                total += current_numbers.iter().product::<u64>();
                current_numbers.clear();
            }
            _ => (),
        }
    }

    total
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
