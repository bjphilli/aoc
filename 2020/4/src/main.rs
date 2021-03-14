use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let _ints = parse_file(File::open("input.txt")?)?;
    let part_one_answer = run_part_one(_ints.clone());

    println!("Part one answer: {}", part_one_answer);
    
    let part_two_answer = run_part_two(_ints);
    println!("Part two answer: {}", part_two_answer);

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn run_part_one(lines: Vec<String>) -> u32 {
    let mut passports: Vec<Vec<String>> = vec![];
    let mut current_passport: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            passports.push(current_passport.clone());
            current_passport.clear();
        }
        else {
            current_passport.push(line);
        }
    }

    passports.push(current_passport.clone());
    let valid_passport_count = passports.iter().filter(|p| is_valid_passport(p)).count();
    valid_passport_count as u32
}

fn is_valid_passport(passport: &Vec<String>) -> bool {
    let mut keys: Vec<String> = vec![];
    for passport_line in passport {
        let passport_keys = passport_line.split(" ");
        for passport_key in passport_keys {
            keys.push(passport_key.split(":").collect::<Vec<&str>>()[0].to_string());
        }
    }

    let is_valid = keys.contains(&"byr".to_string())
        && keys.contains(&"iyr".to_string())
        && keys.contains(&"eyr".to_string())
        && keys.contains(&"hgt".to_string())
        && keys.contains(&"hcl".to_string())
        && keys.contains(&"ecl".to_string())
        && keys.contains(&"pid".to_string());

    is_valid
}

fn is_valid_passport_p2(passport: &Vec<String>) -> bool {
    let mut keys: Vec<String> = vec![];
    let mut byr = "0".to_string();
    let mut iyr = "0".to_string();
    let mut eyr = "0".to_string();
    let mut hgt = "0".to_string();
    let mut hcl = "0".to_string();
    let mut ecl = "0".to_string();
    let mut pid = "0".to_string();

    for passport_line in passport {
        let passport_keys = passport_line.split(" ");
        for passport_key in passport_keys {
            let passport_entry = passport_key.split(":").collect::<Vec<&str>>();
            let key = passport_entry[0].to_string();

            if key == "byr".to_string() {
                byr = passport_entry[1].to_string();
            } else if key == "iyr".to_string() {
                iyr = passport_entry[1].to_string();
            } else if key == "eyr".to_string() {
                eyr = passport_entry[1].to_string();
            } else if key == "hgt".to_string() {
                hgt = passport_entry[1].to_string();
            } else if key == "hcl".to_string() {
                hcl = passport_entry[1].to_string();
            } else if key == "ecl".to_string() {
                ecl = passport_entry[1].to_string();
            } else if key == "pid".to_string() {
                pid = passport_entry[1].to_string();
            }
            
            keys.push(passport_key.split(":").collect::<Vec<&str>>()[0].to_string());
        }
    }

    let has_all_fields = keys.contains(&"byr".to_string())
        && keys.contains(&"iyr".to_string())
        && keys.contains(&"eyr".to_string())
        && keys.contains(&"hgt".to_string())
        && keys.contains(&"hcl".to_string())
        && keys.contains(&"ecl".to_string())
        && keys.contains(&"pid".to_string());

    if !has_all_fields {
        return false;
    }

    let is_valid_byr = byr.parse::<u32>().unwrap() >= 1920 && byr.parse::<u32>().unwrap() <= 2002;
    let is_valid_iyr = iyr.parse::<u32>().unwrap() >= 2010 && iyr.parse::<u32>().unwrap() <= 2020;
    let is_valid_eyr = eyr.parse::<u32>().unwrap() >= 2020 && eyr.parse::<u32>().unwrap() <= 2030;
    let mut is_valid_hgt = false;

    if hgt.ends_with("in") {
        let hgt_amount = &hgt[..hgt.len() - 2].parse::<u32>().unwrap();
        is_valid_hgt = *hgt_amount >= 59 && *hgt_amount <= 76;
    } else if hgt.ends_with("cm") {
        let hgt_amount = &hgt[..hgt.len() - 2].parse::<u32>().unwrap();
        is_valid_hgt = *hgt_amount >= 150 && *hgt_amount <= 193;
    }

    let mut is_valid_hcl = false;

    if hcl.starts_with("#") {
        let filtered_hcl = hcl.replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'][..], "");
        is_valid_hcl = filtered_hcl.len() == 1;
    }

    let is_valid_ecl = ecl == "amb".to_string()
        || ecl == "blu".to_string()
        || ecl == "brn".to_string()
        || ecl == "gry".to_string()
        || ecl == "grn".to_string()
        || ecl == "hzl".to_string()
        || ecl == "oth".to_string();
    let is_valid_pid = pid.len() == 9;

    is_valid_byr
        && is_valid_iyr
        && is_valid_eyr
        && is_valid_hgt
        && is_valid_hcl
        && is_valid_ecl
        && is_valid_pid
}

fn run_part_two(lines: Vec<String>) -> u32 {
    let mut passports: Vec<Vec<String>> = vec![];
    let mut current_passport: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            passports.push(current_passport.clone());
            current_passport.clear();
        }
        else {
            current_passport.push(line);
        }
    }

    passports.push(current_passport.clone());
    let valid_passport_count = passports.iter().filter(|p| is_valid_passport_p2(p)).count();
    valid_passport_count as u32
}

