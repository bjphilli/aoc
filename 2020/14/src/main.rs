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

fn run_part_one(lines: Vec<&str>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: &str = "";

    for line in &lines {
        if line.contains("mask") {
            mask = line.split(" ").collect::<Vec<&str>>()[2];
        } else {
            let idx = line.split("[").collect::<Vec<&str>>()[1]
                .split("]")
                .collect::<Vec<&str>>()[0];

            let val = line.split(" ").collect::<Vec<&str>>()[2]
                .parse::<u64>()
                .unwrap();

            memory.insert(idx.parse::<u64>().unwrap(), apply_mask(val, mask));
        }
    }

    let mut sum = 0;
    for val in memory.values().collect::<Vec<&u64>>() {
        sum += val;
    }

    sum
}

fn apply_mask(val: u64, mask: &str) -> u64 {
    let fmt = format!("{:036b}", val);
    let mut res = String::with_capacity(fmt.len());

    for i in 0..fmt.len() {
        if mask.chars().nth(i).unwrap() == 'X' {
            res.push(fmt.chars().nth(i).unwrap());
            continue;
        }

        res.push(mask.chars().nth(i).unwrap());
    }

    u64::from_str_radix(&res, 2).unwrap()
}

fn run_part_two(lines: Vec<&str>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: &str = "";

    for line in &lines {
        if line.contains("mask") {
            mask = line.split(" ").collect::<Vec<&str>>()[2];
        } else {
            let idx = line.split("[").collect::<Vec<&str>>()[1]
                .split("]")
                .collect::<Vec<&str>>()[0];

            let val = line.split(" ").collect::<Vec<&str>>()[2]
                .parse::<u64>()
                .unwrap();

            apply_mask_p2(&mut memory, idx.parse::<u64>().unwrap(), val, mask);
        }
    }

    let mut sum = 0;
    for val in memory.values().collect::<Vec<&u64>>() {
        sum += val;
    }

    sum
}

fn apply_mask_p2(mem: &mut HashMap<u64, u64>, idx: u64, val: u64, mask: &str) {
    let fmt = format!("{:036b}", val);
    let fmt_idx = format!("{:036b}", idx);
    let mut mem_addresses = vec![];
    let mut new_addresses = vec![];
    mem_addresses.push(String::with_capacity(fmt.len()));

    for i in 0..fmt.len() {
        if mask.chars().nth(i).unwrap() == 'X' {
            for address in mem_addresses.iter_mut() {
                let mut new_addr = address.clone();
                address.push('0');
                new_addr.push('1');
                new_addresses.push(new_addr);
            }

            mem_addresses.append(&mut new_addresses);
            new_addresses.clear();
            continue;
        }

        for address in mem_addresses.iter_mut() {
            let mask_char = mask.chars().nth(i).unwrap();
            if mask_char == '1' {
                address.push(mask_char);
            } else {
                address.push(fmt_idx.chars().nth(i).unwrap());
            }
        }
    }

    for address in &mem_addresses {
        mem.insert(u64::from_str_radix(&address, 2).unwrap(), val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_test() {}

    #[test]
    fn p2_test_1() {
        let input = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];

        assert_eq!(run_part_two(input), 208);
    }
}
