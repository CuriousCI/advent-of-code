fn part_one(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

            (start..=end)
                .filter(|id: &usize| {
                    let string_id = id.to_string();
                    string_id.len().is_multiple_of(2)
                        && string_id[..string_id.len() / 2] == string_id[string_id.len() / 2..]
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

            (start..=end)
                .filter(|id: &usize| {
                    let string_id = id.to_string();

                    (1..=string_id.len() / 2).any(|substr_len| {
                        string_id.len().is_multiple_of(substr_len)
                            && (0..string_id.len())
                                .step_by(substr_len)
                                .all(|i| string_id[i..i + substr_len] == string_id[..substr_len])
                    })
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
