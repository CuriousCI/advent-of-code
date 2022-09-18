use std::collections::HashMap;

fn part_one() -> i32 {
    let (doubles, triples) = include_str!("input")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .fold(HashMap::new(), |mut counts, letter| {
                    *counts.entry(letter).or_insert(0) += 1;
                    counts
                })
                .iter()
                .fold((0, 0), |mut category, (_, count)| {
                    if *count == 2 {
                        category.0 = 1;
                    }
                    if *count == 3 {
                        category.1 = 1;
                    }

                    category
                })
        })
        .reduce(|acc, i| (acc.0 + i.0, acc.1 + i.1))
        .unwrap();

    doubles * triples
}

fn part_two() -> String {
    let mut lines: Vec<String> = include_str!("input")
        .trim()
        .lines()
        .map(String::from)
        .collect();

    lines.sort();

    let mut previous_line = lines.get(0).unwrap();

    for line in lines[1..].iter() {
        let mut equal_letters = String::new();

        for index in 0..line.len() {
            if line.chars().nth(index).unwrap() == previous_line.chars().nth(index).unwrap() {
                equal_letters.push(line.chars().nth(index).unwrap());
            }
        }

        if equal_letters.len() == line.len() - 1 {
            return equal_letters;
        }

        previous_line = line;
    }

    String::from("")
}

fn main() {
    println!("{}", part_two());
}
