fn part_one(input: &str) -> i32 {
    let sequence: Vec<_> = input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut dial = 50;
    let mut zero_count = 0;

    for (direction, offset) in sequence {
        dial = (dial + offset * if direction == 'L' { -1 } else { 1 }) % 100;
        if dial == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part_two(input: &str) -> i32 {
    let sequence: Vec<_> = input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut dial = 50;
    let mut zero_count = 0;

    for (direction, offset) in sequence {
        for _ in 0..offset {
            dial = (dial + if direction == 'L' { -1 } else { 1 }) % 100;
            if dial == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
