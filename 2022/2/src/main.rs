fn part_one() -> u32 {
    include_str!("input")
        .lines()
        .map(|line| line.split_once(' '))
        .map(Option::unwrap)
        .map(|(opponent, me)| {
            let choice = match me {
                "X" => 1,
                "Y" => 2,
                _ => 3,
            };

            choice
                + match opponent {
                    "A" => match me {
                        "X" => 3,
                        "Y" => 6,
                        _ => 0,
                    },
                    "B" => match me {
                        "Y" => 3,
                        "Z" => 6,
                        _ => 0,
                    },
                    _ => match me {
                        "Z" => 3,
                        "X" => 6,
                        _ => 0,
                    },
                }
        })
        .sum()
}

fn part_two() -> u32 {
    include_str!("input")
        .lines()
        .map(|line| line.split_once(' '))
        .map(Option::unwrap)
        .map(|(opponent, outcome)| match opponent {
            "A" => match outcome {
                "X" => 3,
                "Y" => 4,
                _ => 8,
            },
            "B" => match outcome {
                "X" => 1,
                "Y" => 5,
                _ => 9,
            },
            _ => match outcome {
                "X" => 2,
                "Y" => 6,
                _ => 7,
            },
        })
        .sum()
}

fn main() {
    println!("{}", part_two());
}
