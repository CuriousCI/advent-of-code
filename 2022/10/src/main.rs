fn part_one() -> i32 {
    let mut signal_strengths: Vec<i32> = vec![];
    let mut x = 1;

    for instruction in include_str!("input").lines() {
        match instruction {
            "noop" => signal_strengths.push(x),
            _ => {
                let (_, v) = instruction.split_once(" ").unwrap();
                let v: i32 = v.parse().unwrap();
                signal_strengths.push(x);
                x += v;
                signal_strengths.push(x);
            }
        }
    }

    let mut result = 0;
    for signal in (20..=220).step_by(40) {
        result += signal as i32 * signal_strengths[signal - 1];
    }

    result
}

fn part_two() -> String {
    let mut signals: Vec<i32> = vec![];
    let mut x = 1;
    let mut crt: Vec<Vec<&str>> = vec![vec!["."; 40]; 6];

    for instruction in include_str!("input").lines() {
        match instruction {
            "noop" => signals.push(x),
            _ => {
                let (_, v) = instruction.split_once(" ").unwrap();
                let v: i32 = v.parse().unwrap();
                signals.push(x);
                x += v;
                signals.push(x);
            }
        }
    }

    for (cycle, x) in signals.iter().enumerate() {
        let crt_x: i32 = cycle as i32 % 40;
        let crt_y: i32 = (cycle as i32 - crt_x) / 40;
        let x = *x;

        if crt_x >= x - 2 && crt_x <= x {
            crt[crt_y as usize][crt_x as usize] = "#";
        }
    }

    crt.iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
