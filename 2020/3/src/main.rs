fn part_one() -> usize {
    let map: Vec<Vec<char>> = include_str!("input")
        .trim()
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();

    let (mut x, mut y, mut trees) = (0, 0, 0);
    while y + 1 < map.len() {
        x += 3;
        y += 1;

        if map[y][x % map.first().unwrap().len()] == '#' {
            trees += 1;
        }
    }

    trees
}

fn part_two() -> usize {
    let map: Vec<Vec<char>> = include_str!("input")
        .trim()
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;
    for (x_off, y_off) in slopes {
        let (mut x, mut y, mut trees) = (0, 0, 0);
        while y + y_off < map.len() {
            x += x_off;
            y += y_off;

            if map[y][x % map.first().unwrap().len()] == '#' {
                trees += 1;
            }
        }

        product *= trees;
    }

    product
}

fn main() {
    println!("{}", part_two());
}
