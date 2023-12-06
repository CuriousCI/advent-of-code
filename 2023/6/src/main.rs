fn part_one() -> u64 {
    let (time, distance) = include_str!("input").split_once('\n').unwrap();

    let time: Vec<u64> = time
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();

    let distance: Vec<u64> = distance
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();

    let mut result = 1;
    for (time, distance) in time.iter().zip(distance) {
        let delta = time.pow(2) - 4 * distance;
        let upper = -time + sqrt(delta);
        //
    }

    result
}

fn main() {
    println!("{}", part_one());
}
