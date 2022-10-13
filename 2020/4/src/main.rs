use std::string::ParseError;

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
            "byr" => {
                let year = value.parse::<i32>();
                match year {
                    Ok(value) => {
                        if value < 1920 && value > 2002 {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }
            }
            "iyr" => {
                let year = value.parse::<i32>();
                match year {
                    Ok(value) => {
                        if value < 2010 && value > 2020 {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }
            }
            "eyr" => {
                let year = value.parse::<i32>();
                match year {
                    Ok(value) => {
                        if value < 2020 && value > 2030 {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }
            }
            "hgt" => {}
            "hcl" => {
                if 

            }
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
