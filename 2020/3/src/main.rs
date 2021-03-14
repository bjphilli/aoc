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
    run_core(lines, 1, 3)
}

fn run_core(lines: Vec<String>, x: usize, y: usize) -> u32 {
    let mut tree_count = 0;
    let len = &lines[0].len();
    let mut current_x = 0;
    let mut current_y = 0;

    while current_x < lines.len() {
        let line = &lines[current_x];

        if current_y >= *len {
            current_y = current_y - len;
        }

        let current_char = line.chars().nth(current_y).unwrap();
        if current_char == '#' {
            tree_count += 1;
        }

        current_x += x;
        current_y += y;
    }

    tree_count
}  

fn run_part_two(lines: Vec<String>) -> u32 {
    run_core(lines.clone(), 1, 1) *
    run_core(lines.clone(), 1, 3) *
    run_core(lines.clone(), 1, 5) *
    run_core(lines.clone(), 1, 7) *
    run_core(lines.clone(), 2, 1)
}

