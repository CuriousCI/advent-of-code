fn part_one() -> i32 {
    include_str!("input.txt")
        .trim()
        .chars()
        .map(|instruction| match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn part_two() -> usize {
    let instructions = include_str!("input.txt").trim().chars();

    let mut floor = 1;
    for (index, instruction) in instructions.into_iter().enumerate() {
        floor += match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == 0 {
            return index + 1;
        }
    }

    0
}

fn main() {
    println!("{}", part_two());
}
