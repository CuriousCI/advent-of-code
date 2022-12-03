fn part_one() -> u32 {
    include_str!("input")
        .lines()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            for item in left.chars() {
                if right.contains(item) {
                    return item;
                }
            }

            unreachable!()
        })
        .map(|ch| {
            ch as u32 + 1
                - if ch.is_ascii_lowercase() {
                    'a' as u32
                } else {
                    'A' as u32 - 26
                }
        })
        .sum()
}

fn part_two() -> u32 {
    include_str!("input")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            for item in chunk[0].chars() {
                if chunk[1].contains(item) && chunk[2].contains(item) {
                    return item;
                }
            }

            unreachable!();
        })
        .map(|ch| {
            ch as u32 + 1
                - if ch.is_ascii_lowercase() {
                    'a' as u32
                } else {
                    'A' as u32 - 26
                }
        })
        .sum()
}

fn main() {
    println!("{}", part_two());
}
