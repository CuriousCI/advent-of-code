fn part_one() -> u64 {
    let input = include_str!("input");

    let mut sections = input.split("\r\n\r\n");

    let seeds: Vec<u64> = sections
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();

    let sections: Vec<(Vec<_>, Vec<_>)> = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                // enumerate()
                .map(|line| {
                    let v: Vec<u64> = line
                        .split_whitespace()
                        .map(&str::parse)
                        .map(Result::unwrap)
                        .collect();

                    // Save index in third place?
                    // Save to HashMap?
                    // Maybe use a single big tuple for all the sections in reverse order, and
                    // (even if it seems impossible?)
                    // search a (seed, 0, 0, 0, ..., 0)
                    ((v[1], v[1] + v[2]), (v[0], v[0] + v[2]))
                })
                .unzip()
        })
        .collect();

    0
}

fn main() {
    println!("{}", part_one());
}

// let seed_to_soil;
// let soil_to_fertilizer;
// let fertilizer_to_water;
// let water_to_light;
// let light_to_temperature;
// let temperature_to_humidity;
// let humidity_to_location;
