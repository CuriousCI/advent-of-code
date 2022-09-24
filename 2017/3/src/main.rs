fn part_one() -> u32 {
    let access_port = include_str!("input").trim().parse().unwrap();

    access_port
}

fn main() {
    println!("{}", part_one());
}
