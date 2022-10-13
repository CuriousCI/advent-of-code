fn is_passport_valid(passport: &Vec<&str>) -> bool {
    let mut has_cid = false;
    for information in passport {
        if information.contains("cid:") {
            has_cid = true;
            break;
        }
    }

    passport.len() == 8 || (!has_cid && passport.len() == 7)
}

fn part_one() -> usize {
    include_str!("input")
        .trim()
        .split("\n\n")
        .map(|passport| passport.split_whitespace().collect::<Vec<&str>>())
        .filter(is_passport_valid)
        .count()
}

fn are_fields_valid(passport: &Vec<&str>) -> bool {
    for information in passport {
        let (key, value) = information.split_once(':').unwrap();
        match key {
            "byr" => {}
            "iyr" => {}
            "eyr" => {}
            "hgt" => {}
            "hcl" => {}
            "ecl" => {}
            "pid" => {}
            "cid" => {}
            _ => unreachable!(),
        };
    }

    true
}

fn part_two() -> usize {
    include_str!("input")
        .trim()
        .split("\n\n")
        .map(|passport| passport.split_whitespace().collect::<Vec<&str>>())
        .filter(is_passport_valid)
        .filter(are_fields_valid)
        .count()
}

fn main() {
    println!("{}", part_two());
}
