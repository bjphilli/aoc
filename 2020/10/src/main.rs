use std::collections::HashMap;
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
    let mut ints = map_to_ints(lines);
    ints.sort();

    let mut one_count = 0;
    let mut three_count = 0;
    for i in 0..ints.len() - 1 {
        if ints[i] + 1 == ints[i + 1] {
            one_count += 1;
        }
        if ints[i] + 3 == ints[i + 1] {
            three_count += 1;
        }
    }

    (one_count + 1) * (three_count + 1)
}

fn run_part_two(lines: Vec<&str>) -> usize {
    let mut ints = map_to_ints(lines);
    ints.sort();
    let mut paths = HashMap::new();
    paths.insert(0, 1);

    for i in &ints {
        let mut one_count = 0;
        let mut two_count = 0;
        let mut three_count = 0;

        if paths.contains_key(&(i - 1)) {
            one_count = paths[&(i - 1)];
        }
        if paths.contains_key(&(i - 2)) {
            two_count = paths[&(i - 2)];
        }
        if paths.contains_key(&(i - 3)) {
            three_count = paths[&(i - 3)];
        }

        paths.insert(*i, one_count + two_count + three_count);
    }

    paths[ints.iter().max().unwrap()]
}

fn map_to_ints(lines: Vec<&str>) -> Vec<usize> {
    lines.iter().map(|l| l.parse::<usize>().unwrap()).collect()
}
