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
    let mut valid_passwords = 0;

    for i in lines {
        let split_string : Vec<&str> = i.split(" ").collect();
        let split_bounds : Vec<&str> = split_string[0].split("-").collect();
        let lower_bound = split_bounds[0].parse().unwrap();
        let upper_bound = split_bounds[1].parse().unwrap();
        let test_char = split_string[1].replace(":", "");
        let match_vector: Vec<&str> = split_string[2].matches(&test_char).collect();
        let matches = match_vector.len();
        if matches >= lower_bound && matches <= upper_bound {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn run_part_two(lines: Vec<String>) -> u32 {
    let mut valid_passwords = 0;

    for i in lines {
        let split_string: Vec<&str> = i.split(" ").collect();
        let split_bounds: Vec<&str> = split_string[0].split("-").collect();
        let lower_bound: usize = split_bounds[0].parse().unwrap();
        let upper_bound: usize = split_bounds[1].parse().unwrap();
        let test_char = split_string[1].replace(":", "").chars().nth(0).unwrap();
        let password = split_string[2];
        let lower_test_char = password.chars().nth(lower_bound - 1).unwrap();
        let upper_test_char = password.chars().nth(upper_bound - 1).unwrap();
        if test_char == lower_test_char && test_char != upper_test_char {
            valid_passwords += 1;
        } else if test_char != lower_test_char && test_char == upper_test_char {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

