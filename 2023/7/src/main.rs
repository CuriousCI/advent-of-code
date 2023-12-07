// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456

// for hand in hands {
//     println!("{:?}", hand);
// }

// hands.sort_by(|f|);
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456

//     So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

fn part_one() -> usize {
    let mut hands: Vec<_> = include_str!("input")
        .lines()
        .filter_map(|hand| hand.split_once(' '))
        .map(|(hand, bid)| {
            let bid: u64 = bid.parse().unwrap();

            let mut hand: Vec<_> = hand
                .chars()
                .map(|symbol| match symbol {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'J' => 3,
                    'T' => 4,
                    digit => 16 - digit.to_digit(10).unwrap(),
                })
                .collect();
            hand.sort_unstable();

            let mut counter = vec![0; 17];
            for &card in hand.iter() {
                counter[card as usize] += 1;
            }

            let mut kind = hand.clone();
            kind.dedup();

            let kind = match kind.len() {
                1 => 0,
                2 => match counter[kind[0] as usize].max(counter[kind[1] as usize]) {
                    4 => 1, // four of a kind
                    3 => 2, // full house
                    _ => unreachable!(),
                },
                3 => match counter[kind[0] as usize]
                    .max(counter[kind[1] as usize])
                    .max(counter[kind[2] as usize])
                {
                    2 => 4,
                    _ => 3,
                },
                4 => 5, // one pair
                5 => 6, // high card
                _ => unreachable!(),
            };

            ((kind, hand), bid)
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * *bid as usize)
        .sum()

    // too high
}

fn main() {
    println!("{}", part_one());
}
