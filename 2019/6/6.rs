use std::io::Result;
use std::collections::HashMap;
use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Read};

fn main() -> Result<()> {
    let begin = Instant::now();
    let input = parse_file(File::open("input.txt")?)?;

    let mut hash_map = HashMap::new();

    for line in &input {
        hash_map.insert(&line[4..7], &line[0..3]);
    }

    let mut orbits = 0;

    for (k, _) in &hash_map {
        let mut new_key = k;
        while hash_map.contains_key(new_key) {
            orbits = orbits + 1;
            let parent = hash_map.get(new_key).unwrap();
            new_key = parent;
        }
    }

    println!("Part 1 answer: {}", orbits);

    let part_two_answer = part_2(hash_map);
    println!("Part 2 answer: {}", part_two_answer);

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<String>> {
    let br = BufReader::new(io);
    let mut return_lines = Vec::new();

    for line in br.lines() {
        let lines = line.unwrap();
        return_lines.push(lines);
    }

    Ok(return_lines)
}

fn part_2(orbits: HashMap<&str, &str>) -> u32 {
    let mut you_orbits = HashMap::new();
    let mut santa_orbits = HashMap::new();

    let mut you_key = "YOU";
    let mut you_counter : u32 = 0;
    while orbits.contains_key(you_key) {
        you_orbits.insert(you_key, you_counter);
        let parent = orbits.get(you_key).unwrap();
        you_key = parent;
        you_counter = you_counter + 1;
    }

    let mut santa_key = "SAN";
    let mut santa_counter : u32 = 0;

    while orbits.contains_key(santa_key) {
        santa_orbits.insert(santa_key, santa_counter);
        let parent = orbits.get(santa_key).unwrap();
        santa_key = parent;
        santa_counter = santa_counter + 1;
    }

    let mut intersection_point = "";
    let mut shortest_distance : u32 = 5000;

    for (k, v) in &you_orbits {
        if santa_orbits.contains_key(k) {
            if *v < shortest_distance {
                intersection_point = k;
                shortest_distance = *v;
            }
        }
    }

    return you_orbits.get(intersection_point).unwrap() + santa_orbits.get(intersection_point).unwrap() - 2;
}