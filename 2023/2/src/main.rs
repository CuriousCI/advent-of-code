fn part_one_second() -> u64 {
    include_str!("input")
        .lines()
        .filter_map(|line| {
            line.strip_prefix("Game ")
                .and_then(|line| line.split_once(": "))
                .and_then(|(id, extractions)| {
                    extractions
                        .split(';')
                        .all(|extraction| {
                            extraction.split(',').all(|extracted| {
                                extracted
                                    .trim()
                                    .split_once(' ')
                                    .map(|(count, color)| {
                                        (
                                            count.parse::<u64>().unwrap(),
                                            match color {
                                                "red" => 12,
                                                "green" => 13,
                                                "blue" => 14,
                                                _ => unreachable!(),
                                            },
                                        )
                                    })
                                    .is_some_and(|(count, max)| count <= max)
                            })
                        })
                        .then_some(id.parse::<u64>().unwrap())
                })
        })
        .sum()
}

fn part_one() -> u64 {
    let mut result = 0;

    for line in include_str!("input").lines() {
        let (id, extractions) = line
            .strip_prefix("Game ")
            .and_then(|line| line.split_once(": "))
            .unwrap();

        for extraction in extractions.split(',') {
            let (count, color) = extraction.trim().split_once(' ').unwrap();

            let count = count.parse::<u64>().unwrap();
            let max = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => unreachable!(),
            };

            if count <= max {
                result += id.parse::<u64>().unwrap();
            }
        }
    }

    result
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_one_second());
}
