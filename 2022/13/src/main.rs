use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, PartialOrd)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<Item> = vec![];
        let mut token: String = String::new();
        let mut is_list_analyzed = false;
        let mut open = 0;

        for c in s[1..s.len() - 1].chars() {
            match c {
                '[' => {
                    if !is_list_analyzed {
                        is_list_analyzed = true;
                        token.clear();
                    }

                    open += 1;
                    token.push(c);
                }
                ']' => {
                    token.push(c);
                    if is_list_analyzed {
                        open -= 1;
                    }

                    if open == 0 {
                        items.push(token.parse().unwrap());
                        token.clear();
                        is_list_analyzed = false;
                    }
                }
                _ => {
                    if is_list_analyzed {
                        token.push(c);
                    } else if c == ',' {
                        if token.len() > 0 {
                            items.push(Item::Number(token.parse().unwrap()));
                        }
                        token.clear();
                    } else {
                        token.push(c)
                    }
                }
            }
        }

        if token.len() > 0 {
            if is_list_analyzed {
                items.push(token.parse().unwrap());
            } else {
                items.push(Item::Number(token.parse().unwrap()))
            }
        }

        Ok(Self::List(items))
    }
}

fn part_one() -> usize {
    include_str!("example")
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(left, right)| {
            (
                left.parse::<Item>().unwrap(),
                right.parse::<Item>().unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index)
        .sum()

    // println!("{:?}", pairs);
}

fn main() {
    println!("{}", part_one());
}
