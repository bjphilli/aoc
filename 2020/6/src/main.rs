use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, Read};
use std::collections::HashSet;

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
    let valid_passport_count: usize = passports.iter().map(|p| count_answers(p)).sum();
    valid_passport_count as u32
}

fn count_answers(answers: &Vec<String>) -> usize {
    let mut unique_chars = HashSet::new();
    for answer in answers {
        for c in answer.chars() {
            unique_chars.insert(c);
        }
    }

    unique_chars.len()
}

fn count_answers_p2(lines: &Vec<String>) -> usize {
    let mut unique_chars_vec = vec![];
    let first_line = &lines[0]; 
    for c in first_line.chars() {
        unique_chars_vec.push(c);
    }

        
    for line in lines {
        let mut current_line_chars = HashSet::new();
        for c in line.chars() {
            current_line_chars.insert(c);
        }

        unique_chars_vec.retain(|c| current_line_chars.contains(c));
    }
    unique_chars_vec.len()
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
    let valid_passport_count: usize = passports.iter().map(|p| count_answers_p2(p)).sum();
    valid_passport_count as u32
}

