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

fn run_part_one(lines: Vec<&str>) -> u32 {
    let target_time: u32 = lines.first().unwrap().parse().unwrap();
    let bus_ids: Vec<&str> = lines[1].split(",").collect();

    let bus_ids_parsed: Vec<u32> = bus_ids
        .iter()
        .filter(|&b| *b != "x")
        .copied()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&b| b.parse::<u32>().unwrap())
        .collect();

    find_bus_id(target_time, bus_ids_parsed)
}

fn run_part_two(lines: Vec<&str>) -> u64 {
    let bus_ids: Vec<&str> = lines[1].split(",").collect();
    find_bus_id_p2(bus_ids)
}

fn find_bus_id(target_time: u32, bus_ids: Vec<u32>) -> u32 {
    let mut closest_value = 999999999;
    let mut closest_bus_id = 0;

    for bus in bus_ids {
        let mut i = 0;
        while i * bus < target_time {
            i += 1;
        }

        if (i * bus) - target_time < closest_value {
            closest_value = (i * bus) - target_time;
            closest_bus_id = bus;
        }
    }
    closest_value * closest_bus_id
}

fn find_bus_id_p2(bus_ids: Vec<&str>) -> u64 {
    let mut buses = vec![];
    let mut offset_distance = 0;

    for bus_id in &bus_ids[0..] {
        if bus_id == &"x" {
            offset_distance += 1;
            continue;
        }

        buses.push((offset_distance, bus_id.parse::<u64>().unwrap()));
        offset_distance += 1;
    }

    buses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut timestamp = 0;
    let mut inc = buses[0].1;

    for &(i, bus) in &buses[1..] {
        while (timestamp + i) % bus != 0 {
            timestamp += inc;
        }

        inc *= bus;
    }

    timestamp
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_test() {
        let target_time = 939;
        let bus_ids = vec![7, 13, 59, 31, 19];
        assert_eq!(295, find_bus_id(target_time, bus_ids));
    }

    #[test]
    fn p2_test_1() {
        let bus_ids = vec!["17", "x", "13", "19"];
        assert_eq!(3417, find_bus_id_p2(bus_ids));
    }

    #[test]
    fn p2_test_2() {
        let bus_ids = vec!["67", "7", "59", "61"];
        assert_eq!(754018, find_bus_id_p2(bus_ids));
    }

    #[test]
    fn p2_test_3() {
        let bus_ids = vec!["67", "x", "7", "59", "61"];
        assert_eq!(779210, find_bus_id_p2(bus_ids));
    }

    #[test]
    fn p2_test_4() {
        let bus_ids = vec!["67", "7", "x", "59", "61"];
        assert_eq!(1261476, find_bus_id_p2(bus_ids));
    }

    #[test]
    fn p2_test_5() {
        let bus_ids = vec!["1789", "37", "47", "1889"];
        assert_eq!(1202161486, find_bus_id_p2(bus_ids));
    }
}
