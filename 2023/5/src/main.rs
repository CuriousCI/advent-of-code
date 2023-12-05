use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

pub fn binary_search<T: Ord>(array: &Vec<(u64, u64)>, value: &u64) -> usize {
    let mut step = array.len();
    let mut index = 0;

    while step > 0 {
        let mut next = index + step;

        while next < array.len() {
            match array.get(next) {
                Some((l, r)) => {
                    if l <= value && value < r {
                        return next;
                    } else if r <= value {
                        index = next;
                    } else if l >= value {
                        break;
                    }
                }
                None => break,
            };

            next += step;
        }

        step /= 2;
    }

    index
}

fn part_two() -> u64 {
    let (tx, rx) = channel();

    thread::scope(|scope| {
        let mut sections = include_str!("input").split("\n\n");

        let seeds: Vec<u64> = sections
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(&str::parse)
            .map(Result::unwrap)
            .collect();

        let sections: Vec<(Vec<_>, HashMap<_, _>)> = sections
            .map(|section| {
                let (mut vec, map): (Vec<_>, HashMap<_, _>) = section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let v: Vec<u64> = line
                            .split_whitespace()
                            .map(&str::parse)
                            .map(Result::unwrap)
                            .collect();

                        ((v[1], v[1] + v[2]), ((v[1], v[1] + v[2]), v[0]))
                    })
                    .unzip();

                vec.sort_unstable();

                (vec, map)
            })
            .collect();

        for chunk in seeds.chunks(2) {
            let tx = tx.clone();
            let range = chunk[0]..chunk[0] + chunk[1];
            let sections = sections.clone();

            scope.spawn(move || {
                let min = range
                    .map(|seed| {
                        let mut translation = seed;

                        for (ranges, translate) in sections.iter() {
                            let index = binary_search::<(u64, u64)>(ranges, &translation);
                            let translated_range_start = translate[&ranges[index]];
                            let (left, right) = ranges[index];
                            if translation >= left && translation < right {
                                let offset = translation - left;
                                translation = translated_range_start + offset;
                            }
                        }

                        translation
                    })
                    .min()
                    .unwrap();

                tx.send(min).unwrap();
            });
        }
    });

    drop(tx);
    rx.iter().min().unwrap()
}

fn main() {
    println!("{}", part_two());
}

// Save index in third place?
// Save to HashMap?
// Maybe use a single big tuple for all the sections in reverse order, and
// (even if it seems impossible?)
// search a (seed, 0, 0, 0, ..., 0)

// let seed_to_soil;
// let soil_to_fertilizer;
// let fertilizer_to_water;
// let water_to_light;
// let light_to_temperature;
// let temperature_to_humidity;
// let humidity_to_location;
// let seeds: Vec<u64> = seeds
//     .chunks(2)
//     .map(|chunk| chunk[0]..chunk[0] + chunk[1])
//     .flatten()
//     .collect();
// translations HashMap?
