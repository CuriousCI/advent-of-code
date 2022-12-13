#[derive(Debug)]
struct Spot {
    x: usize,
    y: usize,
}

fn dijkstra() {}

fn part_one() -> u32 {
    let map: Vec<Vec<u32>> = include_str!("input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|square| match square {
                    'S' => 69,
                    'E' => 420,
                    elevation => elevation as u32 - 97,
                })
                .collect()
        })
        .collect();

    let mut spot: Spot = Spot { x: 0, y: 0 };
    let mut destination: Spot = Spot { x: 0, y: 0 };

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 69 {
                spot = Spot { x, y }
            }

            if map[y][x] == 420 {
                destination = Spot { x, y }
            }
        }
    }

    println!("{:?}", spot);
    println!("{:?}", destination);

    // Dijkstra forever

    for y in 0..map.len() {
        for x in 0..map[y].len() {}
    }

    // println!("{:?}", paths);

    10
}

fn main() {
    println!("{}", part_one());
}
