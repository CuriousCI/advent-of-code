fn part_one() -> String {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let (mut x, mut y): (i32, i32) = (1, 1);

    include_str!("input")
        .trim()
        .lines()
        .map(|instructions| {
            for instruction in instructions.chars() {
                match instruction {
                    'U' => y -= 1,
                    'D' => y += 1,
                    'L' => x -= 1,
                    'R' => x += 1,
                    _ => unreachable!(),
                }

                if y < 0 {
                    y += 1;
                }

                if y > 2 {
                    y -= 1;
                }

                if x < 0 {
                    x += 1;
                }

                if x > 2 {
                    x -= 1;
                }
            }

            keypad[y as usize][x as usize]
        })
        .collect()
}

fn part_two() -> String {
    let keypad = vec![
        vec!['1'],
        vec!['2', '3', '4'],
        vec!['5', '6', '7', '8', '9'],
        vec!['A', 'B', 'C'],
        vec!['D'],
    ];

    let (mut x, mut y): (i32, i32) = (1, 1);

    include_str!("example")
        .trim()
        .lines()
        .map(|instructions| {
            for instruction in instructions.chars() {
                match instruction {
                    'U' => y -= 1,
                    'D' => y += 1,
                    'L' => x -= 1,
                    'R' => x += 1,
                    _ => unreachable!(),
                }

                if y < 0 {
                    y += 1;
                }

                if y > 4 {
                    y -= 1;
                }

                if x < 0 {
                    x += 1;
                }

                match y {
                    0 | 4 => {
                        if x > 0 {
                            x = 0;
                        }
                    }
                    1 | 3 => {
                        if x > 2 {
                            x = 2
                        }
                    }
                    2 => {
                        if x > 4 {
                            x = 4
                        }
                    }
                    _ => unreachable!(),
                }
            }

            keypad[y as usize][x as usize]
        })
        .collect()
}

fn main() {
    println!("{}", part_two());
}
