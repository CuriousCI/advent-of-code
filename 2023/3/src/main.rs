use std::collections::HashSet;

fn part_one() -> u64 {
    let matrix: Vec<Vec<char>> = include_str!("input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut is_dirty: Vec<Vec<bool>> = vec![];
    for line in matrix.iter() {
        let mut l = vec![];
        for _ in line.iter() {
            l.push(false);
        }
        is_dirty.push(l);
    }

    let adj: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (y, line) in matrix.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if !symbol.is_digit(10) && symbol != &'.' {
                for (adj_x, adj_y) in adj {
                    let n_x = x as i64 + adj_x;
                    let n_y = y as i64 + adj_y;

                    if n_x >= 0
                        && n_x < is_dirty[0].len() as i64
                        && n_y >= 0
                        && n_y < is_dirty.len() as i64
                    {
                        is_dirty[n_y as usize][n_x as usize] = true;
                    }
                }
            }
        }
    }

    let mut result = 0;

    for (y, line) in matrix.iter().enumerate() {
        let mut number = String::new();
        let mut is_number_dirty = false;
        let mut numbers = vec![];
        for (x, ch) in line.iter().enumerate() {
            if ch.is_digit(10) {
                number.push(*ch);
                if is_dirty[y][x] {
                    is_number_dirty = true;
                }
            } else if !number.is_empty() {
                numbers.push((number, is_number_dirty));
                is_number_dirty = false;
                number = String::new();
            }
        }

        for (number, is_number_dirty) in numbers {
            if is_number_dirty {
                result += number.parse::<u64>().unwrap();
            }
        }
        if is_number_dirty && !number.is_empty() {
            result += number.parse::<u64>().unwrap();
        }
    }

    result
}

fn part_two() -> u64 {
    let matrix: Vec<Vec<char>> = include_str!("input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let adj: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut gear_ids: Vec<Vec<u64>> = vec![];
    for line in matrix.iter() {
        let mut l = vec![];
        for _ in line.iter() {
            l.push(0);
        }
        gear_ids.push(l);
    }

    let mut gears: Vec<Vec<u64>> = vec![];
    let mut gear_id = 1;

    for (y, line) in matrix.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if symbol == &'*' {
                for (adj_x, adj_y) in adj {
                    let n_x = x as i64 + adj_x;
                    let n_y = y as i64 + adj_y;

                    if n_x >= 0
                        && n_x < matrix[0].len() as i64
                        && n_y >= 0
                        && n_y < matrix.len() as i64
                    {
                        gear_ids[n_y as usize][n_x as usize] = gear_id;
                    }
                }
                gears.push(vec![]);
                gear_id += 1;
            }
        }
    }

    gears.push(vec![]);

    for (y, line) in matrix.iter().enumerate() {
        let mut number = String::new();
        let mut touched_gears = HashSet::new();
        for (x, ch) in line.iter().enumerate() {
            if ch.is_digit(10) {
                number.push(*ch);
                if gear_ids[y][x] != 0 {
                    touched_gears.insert(gear_ids[y][x]);
                }
            } else if !number.is_empty() {
                let n: u64 = number.parse().unwrap();
                for &gear in touched_gears.iter() {
                    gears[gear as usize].push(n);
                }
                number = String::new();
                touched_gears = HashSet::new();
            } else {
                touched_gears = HashSet::new();
            }
        }
        if !number.is_empty() {
            let n: u64 = number.parse().unwrap();
            for &gear in touched_gears.iter() {
                gears[gear as usize].push(n);
            }
        }
    }

    let mut result = 0;

    for g in gears.iter() {
        if g.len() == 2 {
            result += g[0] * g[1];
        }
    }

    result
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
