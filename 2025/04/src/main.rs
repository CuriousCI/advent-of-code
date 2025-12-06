const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbouring_rolls(map: &[Vec<char>], row: usize, col: usize) -> usize {
    OFFSETS
        .into_iter()
        .filter(|&(row_offset, col_offset)| {
            row.checked_add_signed(row_offset).is_some_and(|new_row| {
                col.checked_add_signed(col_offset).is_some_and(|new_col| {
                    map.get(new_row)
                        .and_then(|row| row.get(new_col))
                        .is_some_and(|cell| cell == &'@')
                })
            })
        })
        .count()
}

fn part_one(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut free_rolls = 0;

    for (row, map_row) in map.iter().enumerate() {
        for (col, map_cell) in map_row.iter().enumerate() {
            if map_cell != &'@' {
                continue;
            }

            if count_neighbouring_rolls(&map, row, col) < 4 {
                free_rolls += 1;
            }
        }
    }

    free_rolls
}

fn part_two(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut free_rolls = 0;
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] != '@' {
                    continue;
                }

                if count_neighbouring_rolls(&map, row, col) < 4 {
                    free_rolls += 1;
                    has_changed = true;
                    map[row][col] = 'x';
                }
            }
        }
    }

    free_rolls
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
