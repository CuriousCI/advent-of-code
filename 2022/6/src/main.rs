fn part_one() -> usize {
    let characters = include_str!("input").chars().collect::<Vec<char>>();
    for index in 0..characters.len() {
        let mut c: Vec<char> = vec![];
        for offset in 0..4 {
            c.push(characters[index + offset]);
        }

        c.sort();
        c.dedup();

        if c.len() == 4 {
            return index + 4;
        }
    }

    unreachable!()
}

fn part_two() -> usize {
    let characters = include_str!("input").chars().collect::<Vec<char>>();
    for index in 0..characters.len() {
        let mut c: Vec<char> = vec![];

        for offset in 0..14 {
            c.push(characters[index + offset]);
        }

        c.sort();
        c.dedup();

        if c.len() == 14 {
            return index + 14;
        }
    }

    unreachable!()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
