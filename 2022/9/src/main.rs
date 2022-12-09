use std::collections::HashSet;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn part_one() -> usize {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    let directions: Vec<(&str, i32)> = include_str!("input")
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_once(" ").unwrap();
            (direction, amount.parse().unwrap())
        })
        .collect();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    visited_positions.insert((tail.x, tail.y));

    for (direction, amount) in directions {
        for _ in 0..amount {
            match direction {
                "D" => head.y += 1,
                "U" => head.y -= 1,
                "R" => head.x += 1,
                "L" => head.x -= 1,
                _ => unreachable!(),
            }

            let horizontal = head.x - tail.x;
            let vertical = head.y - tail.y;

            if horizontal.abs() > 1 || vertical.abs() > 1 {
                if vertical == 0 {
                    tail.x += if horizontal > 0 { 1 } else { -1 };
                } else if horizontal == 0 {
                    tail.y += if vertical > 0 { 1 } else { -1 };
                } else {
                    tail.x += if horizontal > 0 { 1 } else { -1 };
                    tail.y += if vertical > 0 { 1 } else { -1 };
                }
            }

            visited_positions.insert((tail.x, tail.y));
        }
    }

    visited_positions.len()
}

fn part_two() -> usize {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    let directions: Vec<(&str, i32)> = include_str!("input")
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_once(" ").unwrap();
            (direction, amount.parse().unwrap())
        })
        .collect();

    let mut rope: Vec<Point> = vec![Point { x: 0, y: 0 }; 10];
    visited_positions.insert((0, 0));

    for (direction, amount) in directions {
        for _ in 0..amount {
            match direction {
                "D" => rope[0].y += 1,
                "U" => rope[0].y -= 1,
                "R" => rope[0].x += 1,
                "L" => rope[0].x -= 1,
                _ => unreachable!(),
            }

            for node in 0..rope.len() - 1 {
                let horizontal = rope[node].x - rope[node + 1].x;
                let vertical = rope[node].y - rope[node + 1].y;

                if horizontal.abs() > 1 || vertical.abs() > 1 {
                    if vertical == 0 {
                        rope[node + 1].x += if horizontal > 0 { 1 } else { -1 };
                    } else if horizontal == 0 {
                        rope[node + 1].y += if vertical > 0 { 1 } else { -1 };
                    } else {
                        rope[node + 1].x += if horizontal > 0 { 1 } else { -1 };
                        rope[node + 1].y += if vertical > 0 { 1 } else { -1 };
                    }
                }
            }

            visited_positions.insert((rope[rope.len() - 1].x, rope[rope.len() - 1].y));
        }
    }

    visited_positions.len()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
