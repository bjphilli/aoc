use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let _ints = parse_file(File::open("input.txt")?)?;
    let part_one_answer = run_part_one(_ints.iter().map(AsRef::as_ref).collect());

    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = run_part_two(_ints.iter().map(AsRef::as_ref).collect());
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

fn run_part_one(lines: Vec<&str>) -> usize {
    let _ints = map_to_ints(lines);

    for i in 25.._ints.len() - 1 {
        let candidates = &_ints[i - 25..i];
        if !is_valid(candidates, _ints[i]) {
            return _ints[i];
        }
    }

    panic!("Not supposed to get here, no value found");
}

fn is_valid(candidates: &[usize], test: usize) -> bool {
    for i in 0..candidates.len() {
        for j in i..candidates.len() {
            if &candidates[i] + &candidates[j] == test {
                return true;
            }
        }
    }

    false
}

fn run_part_two(lines: Vec<&str>) -> usize {
    let _ints = map_to_ints(lines);

    for i in 0.._ints.len() - 1 {
        for j in i.._ints.len() - 1 {
            let slice = &_ints[i..j];
            let sum: usize = slice.iter().sum();
            if sum == 70639851 {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            }
        }
    }

    panic!("Not supposed to get here, no value found");
}

fn map_to_ints(lines: Vec<&str>) -> Vec<usize> {
    lines.iter().map(|l| l.parse::<usize>().unwrap()).collect()
}
