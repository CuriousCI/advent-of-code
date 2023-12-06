fn part_one() -> u64 {
    let (time, distance) = include_str!("input").split_once('\n').unwrap();

    let time: Vec<f64> = time
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();

    let distance: Vec<f64> = distance
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();

    let mut result = 1;

    for (&time, distance) in time.iter().zip(distance) {
        let delta = time.powi(2) - 4.0 * distance;
        let upper = (-1.0 * delta.sqrt() - time) / -2.0;
        let lower = (delta.sqrt() - time) / -2.0;

        result *= (upper.ceil() - lower.floor()) as u64 - 1
    }

    result as u64
}

fn part_two() -> u64 {
    let (time, distance) = include_str!("input").split_once('\n').unwrap();

    let time: f64 = time
        .chars()
        .filter(|ch| ch.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: f64 = distance
        .chars()
        .filter(|ch| ch.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let delta = time.powi(2) - 4.0 * distance;
    let upper = (-1.0 * delta.sqrt() - time) / -2.0;
    let lower = (delta.sqrt() - time) / -2.0;

    (upper.ceil() - lower.floor()) as u64 - 1
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
