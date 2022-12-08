fn part_one() -> u32 {
    let grid: Vec<Vec<u32>> = include_str!("input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut visible: Vec<Vec<bool>> = vec![];
    for line in grid.iter() {
        let mut row: Vec<bool> = vec![];
        for _ in line {
            row.push(false);
        }
        visible.push(row);
    }

    for x in 0..grid[0].len() {
        visible[0][x] = true;
        visible[grid.len() - 1][x] = true;
    }

    for y in 0..grid.len() {
        visible[y][0] = true;
        visible[y][grid[0].len() - 1] = true;
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut biggest = true;
            for left in 0..x {
                if grid[y][left] >= grid[y][x] {
                    biggest = false;
                    break;
                }
            }
            if biggest {
                visible[y][x] = visible[y][x] || true;
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut biggest = true;
            for right in (x + 1)..grid[0].len() {
                if grid[y][right] >= grid[y][x] {
                    biggest = false;
                    break;
                }
            }
            if biggest {
                visible[y][x] = visible[y][x] || true;
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut biggest = true;
            for top in 0..y {
                if grid[top][x] >= grid[y][x] {
                    biggest = false;
                    break;
                }
            }
            if biggest {
                visible[y][x] = visible[y][x] || true;
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut biggest = true;
            for bottom in (y + 1)..grid.len() {
                if grid[bottom][x] >= grid[y][x] {
                    biggest = false;
                    break;
                }
            }

            if biggest {
                visible[y][x] = visible[y][x] || true;
            }
        }
    }

    let mut visible_count = 0;
    for row in visible.iter() {
        for tree in row.iter() {
            if *tree {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn part_two() -> u32 {
    let grid: Vec<Vec<u32>> = include_str!("input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut scenic_scores: Vec<Vec<u32>> = vec![];
    for line in grid.iter() {
        scenic_scores.push(vec![1; line.len()]);
    }

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let mut left_score = 0;

            for left in (0..x).rev() {
                left_score += 1;

                if grid[y][left] >= grid[y][x] {
                    break;
                }
            }

            scenic_scores[y][x] *= left_score;
        }
    }

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let mut right_score = 0;
            for right in x + 1..grid[0].len() {
                right_score += 1;

                if grid[y][right] >= grid[y][x] {
                    break;
                }
            }

            scenic_scores[y][x] *= right_score;
        }
    }

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let mut top_score = 0;
            for top in (0..y).rev() {
                top_score += 1;
                if grid[top][x] >= grid[y][x] {
                    break;
                }
            }

            scenic_scores[y][x] *= top_score;
        }
    }

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let mut bottom_score = 0;
            for bottom in y + 1..grid.len() {
                bottom_score += 1;

                if grid[bottom][x] >= grid[y][x] {
                    break;
                }
            }

            scenic_scores[y][x] *= bottom_score;
        }
    }

    *scenic_scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
