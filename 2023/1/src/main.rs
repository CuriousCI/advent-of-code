fn part_one() -> u32 {
    include_str!("input")
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

            let mut number = String::with_capacity(2);
            number.push(*digits.first().unwrap());
            number.push(*digits.last().unwrap());

            number.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_two() -> u32 {
    let conversions = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let conversions: Vec<(Vec<char>, &char)> = conversions
        .iter()
        .map(|(symbol, digit)| (symbol.chars().collect(), digit))
        .collect();

    include_str!("input")
        .lines()
        .map(|line| {
            let mut digits = vec![];

            let mut counters: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

            for symbol in line.chars() {
                if symbol.is_digit(10) {
                    digits.push(symbol);
                    counters = [0, 0, 0, 0, 0, 0, 0, 0, 0];
                } else {
                    for (index, (from, to)) in conversions.iter().enumerate() {
                        if from[counters[index]] == symbol {
                            counters[index] += 1;
                        } else {
                            counters[index] = if symbol == from[0] { 1 } else { 0 };
                        }

                        if counters[index] == from.len() {
                            digits.push(**to);
                            counters[index] = 0;
                        }
                    }
                }
            }

            let mut number = String::with_capacity(2);
            number.push(*digits.first().unwrap());
            number.push(*digits.last().unwrap());

            // println!("{} -> {:?} -> {}", line, digits, number);

            number.parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
