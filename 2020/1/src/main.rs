use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

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

fn parse_file<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn run_part_one(ints: Vec<u32>) -> u32 {
    let _ints2 = ints.clone();

    for i in &ints {
        for j in &_ints2 {
            if i == j {continue};

            if i + j == 2020 {
                println!("Found 2 numbers that sum to 2020: {}, {}", i, j);
                return i * j;
            }
        }
    }

    println!("No numbers found that sum up to 2020. Returning 0");
    0
}

fn run_part_two(ints: Vec<u32>) -> u32 {
    let _ints2 = ints.clone();
    let _ints3 = ints.clone();

    for i in &ints {
        for j in &_ints2 {
            for k in &_ints3 {
                if i == j {continue};
                if j == k {continue};
                if j == k {continue};

                if i + j + k == 2020 {
                    println!("Found 3 numbers that sum to 2020: {}, {}, {}", i, j, k);
                    return i * j * k;
                }
            }

        }
    }

    println!("No numbers found that sum up to 2020. Returning 0");
    0
}


