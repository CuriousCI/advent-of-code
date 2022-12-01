fn part_one() -> u32 {
    include_str!("input")
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", part_one())
}
