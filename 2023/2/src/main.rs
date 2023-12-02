fn part_one() -> u64 {
    let mut result = 0;

    for line in include_str!("input").lines() {
        let (id, extractions) = line
            .strip_prefix("Game ")
            .and_then(|line| line.split_once(": "))
            .unwrap();

        let mut all_2 = true;
        for extraction in extractions.split(';') {
            let mut all = true;

            for extracted in extraction.split(',') {
                let (count, color) = extracted.trim().split_once(' ').unwrap();

                let count = count.parse::<u64>().unwrap();
                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!(),
                };

                if count > max {
                    all = false;
                    break;
                }
            }

            if !all {
                all_2 = false;
                break;
            }
        }

        if all_2 {
            result += id.parse::<u64>().unwrap();
        }
    }

    result
}

fn part_two() -> u64 {
    let mut result = 0;

    for line in include_str!("input").lines() {
        let (_, extractions) = line
            .strip_prefix("Game ")
            .and_then(|line| line.split_once(": "))
            .unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for extraction in extractions.split(';') {
            for extracted in extraction.split(',') {
                let (count, color) = extracted.trim().split_once(' ').unwrap();

                let count = count.parse::<u64>().unwrap();
                match color {
                    "red" => red = red.max(count),
                    "green" => green = green.max(count),
                    "blue" => blue = blue.max(count),
                    _ => unreachable!(),
                };
            }
        }

        result += red * green * blue;
    }

    result
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
