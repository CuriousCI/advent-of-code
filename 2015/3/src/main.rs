use std::collections::HashMap;

fn part_one() -> i32 {
    let directions = include_str!("input.txt").trim().chars();

    let mut houses_visits: HashMap<(i32, i32), i32> = HashMap::new();
    let mut houses_with_multiple_visits = 0;

    let (mut x, mut y) = (0, 0);

    for direction in directions {
        match direction {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => unreachable!(),
        }

        houses_visits.insert(
            (x, y),
            match houses_visits.get(&(x, y)) {
                Some(value) => value + 1,
                None => {
                    houses_with_multiple_visits += 1;

                    1
                }
            },
        );
    }

    houses_with_multiple_visits
}

fn part_two() -> i32 {
    let directions = include_str!("input.txt").trim().chars();

    let mut houses_visits: HashMap<(i32, i32), i32> = HashMap::new();
    let mut houses_with_multiple_visits = 0;

    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);

    for (index, direction) in directions.into_iter().enumerate() {
        if index & 1 == 0 {
            match direction {
                '<' => santa_x -= 1,
                '>' => santa_x += 1,
                '^' => santa_y -= 1,
                'v' => santa_y += 1,
                _ => unreachable!(),
            }
            houses_visits.insert(
                (santa_x, santa_y),
                match houses_visits.get(&(santa_x, santa_y)) {
                    Some(value) => value + 1,
                    None => {
                        houses_with_multiple_visits += 1;

                        1
                    }
                },
            );
        } else {
            match direction {
                '<' => robo_x -= 1,
                '>' => robo_x += 1,
                '^' => robo_y -= 1,
                'v' => robo_y += 1,
                _ => unreachable!(),
            }

            houses_visits.insert(
                (robo_x, robo_y),
                match houses_visits.get(&(robo_x, robo_y)) {
                    Some(value) => value + 1,
                    None => {
                        houses_with_multiple_visits += 1;

                        1
                    }
                },
            );
        }
    }

    houses_with_multiple_visits
}

fn main() {
    println!("{}", part_two());
}
