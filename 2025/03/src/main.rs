use std::cmp::Reverse;

fn find_bank_max_joltage(bank: &[u64], batteries_to_turn_on: usize) -> u64 {
    let mut subarray_start = 0;

    (0..batteries_to_turn_on)
        .rev()
        .map(|remaining_batteries| {
            let (battery_subarray_index, joltage) = bank
                [subarray_start..bank.len() - remaining_batteries]
                .iter()
                .enumerate()
                .min_by_key(|&(battery_index, joltage)| (Reverse(joltage), battery_index))
                .unwrap();

            subarray_start += battery_subarray_index + 1;
            joltage * 10u64.pow(remaining_batteries as u32)
        })
        .sum()
}

fn max_joltage_sum(input: &str, batteries_to_turn_on: usize) -> u64 {
    input
        .lines()
        .map(|bank| {
            find_bank_max_joltage(
                &bank
                    .chars()
                    .map(|char| char.to_digit(10).unwrap().into())
                    .collect::<Vec<_>>(),
                batteries_to_turn_on,
            )
        })
        .sum()
}

fn part_one(input: &str) -> u64 {
    max_joltage_sum(input, 2)
}

fn part_two(input: &str) -> u64 {
    max_joltage_sum(input, 12)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
