use std::collections::VecDeque;

fn part_one() -> &'static str {
    let (crates, procedure) = include_str!("input").split_once("\n\n").unwrap();
    let crates = crates
        .replace("] ", "")
        .replace(" [", "")
        .replace("]", "")
        .replace("[", "")
        .replace("   ", "-")
        .replace(" ", "");

    let mut crates: Vec<Vec<char>> = crates.lines().map(|line| line.chars().collect()).collect();
    crates.pop();

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for i in 0..crates[0].len() {
        let mut stack: VecDeque<char> = VecDeque::new();
        for line in &crates {
            if line[i] != '-' {
                stack.push_back(line[i]);
            }
        }
        stacks.push(stack);
    }

    let p: Vec<(u32, usize, usize)> = procedure
        .lines()
        .map(|line| {
            let (count, range) = line[5..].split_once(" from ").unwrap();
            let (from, to) = range.split_once(" to ").unwrap();
            let from = from.parse::<usize>().unwrap() - 1;
            let to = to.parse::<usize>().unwrap() - 1;
            let count = count.parse::<u32>().unwrap();
            (count, from, to)
        })
        .collect();

    for (count, from, to) in p {
        for _ in 0..count {
            let a = stacks[from].pop_front().unwrap();
            stacks[to].push_front(a);
        }
    }

    for x in stacks.iter().map(|stack| stack.front().unwrap()) {
        print!("{}", x);
    }
    // VecDeque::drain

    ""
}

fn part_two() -> &'static str {
    let (crates, procedure) = include_str!("input").split_once("\n\n").unwrap();
    let crates = crates
        .replace("] ", "")
        .replace(" [", "")
        .replace("]", "")
        .replace("[", "")
        .replace("   ", "-")
        .replace(" ", "");

    let mut crates: Vec<Vec<char>> = crates.lines().map(|line| line.chars().collect()).collect();
    crates.pop();

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for i in 0..crates[0].len() {
        let mut stack: VecDeque<char> = VecDeque::new();
        for line in &crates {
            if line[i] != '-' {
                stack.push_back(line[i]);
            }
        }
        stacks.push(stack);
    }

    let p: Vec<(u32, usize, usize)> = procedure
        .lines()
        .map(|line| {
            let (count, range) = line[5..].split_once(" from ").unwrap();
            let (from, to) = range.split_once(" to ").unwrap();
            let from = from.parse::<usize>().unwrap() - 1;
            let to = to.parse::<usize>().unwrap() - 1;
            let count = count.parse::<u32>().unwrap();
            (count, from, to)
        })
        .collect();

    for (count, from, to) in p {
        let mut res: Vec<char> = vec![];
        for _ in 0..count {
            let a = stacks[from].pop_front().unwrap();
            res.push(a)
            // stacks[to].push_back(a);
        }
        res.reverse();
        for x in res {
            stacks[to].push_front(x);
        }
    }

    for x in stacks.iter().map(|stack| stack.front().unwrap()) {
        print!("{}", x);
    }
    // VecDeque::drain

    ""
}

fn main() {
    println!("{}", part_two());
}
