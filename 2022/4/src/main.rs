fn part_one() -> usize {
    include_str!("input")
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (a, b) = left.split_once('-').unwrap();
            let (c, d) = right.split_once('-').unwrap();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            let c: u32 = c.parse().unwrap();
            let d: u32 = d.parse().unwrap();

            (a >= c && b <= d) || (c >= a && d <= b)
        })
        .count()
}

fn part_two() -> usize {
    include_str!("input")
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (a, b) = left.split_once('-').unwrap();
            let (c, d) = right.split_once('-').unwrap();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            let c: u32 = c.parse().unwrap();
            let d: u32 = d.parse().unwrap();

            (c <= b && d >= a) || (a <= d && b >= c)
        })
        .count()
}

fn main() {
    println!("{}", part_two());
}
