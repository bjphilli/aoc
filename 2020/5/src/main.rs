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
    lines.iter().map(|l| parse_boarding_pass_string(l)).max().unwrap()
}

fn parse_boarding_pass_string(line: &String) -> u32 {
    let section = &line[..7];
    let row = &line[7..];
    let bin_section = &section.replace('F', "0").replace('B', "1");
    let bin_row = &row.replace('L', "0").replace('R', "1");
    let section_num = u32::from_str_radix(bin_section, 2).unwrap();
    let row_num = u32::from_str_radix(bin_row, 2).unwrap();
    (section_num * 8) + row_num
}

fn run_part_two(lines: Vec<String>) -> u32 {
    let mut all_seat_ids: Vec<u32> = lines.iter().map(|l| parse_boarding_pass_string(l)).collect();
    all_seat_ids.sort();
    let mut last_value = 0;

    for line in all_seat_ids {
        if last_value != line - 1 && last_value != 0 {
            break;
        }
        last_value = line;
    }

    last_value + 1
}

