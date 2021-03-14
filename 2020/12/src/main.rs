mod ship;
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
    let mut ship = ship::Spaceship::init();
    move_ship(&mut ship, lines);
    ship.manhattan_distance()
}

fn run_part_two(lines: Vec<&str>) -> u32 {
    let mut ship = ship::Spaceship::init();
    move_ship_p2(&mut ship, lines);
    ship.manhattan_distance()
}

fn move_ship(ship: &mut ship::Spaceship, lines: Vec<&str>) -> () {
    for line in &lines {
        let command = line.chars().next().unwrap();
        let magnitude = &line[1..].parse::<i32>().unwrap();
        ship.process_command(command, *magnitude);
    }
}

fn move_ship_p2(ship: &mut ship::Spaceship, lines: Vec<&str>) -> () {
    for line in &lines {
        let command = line.chars().next().unwrap();
        let magnitude = &line[1..].parse::<i32>().unwrap();
        ship.process_command_p2(command, *magnitude);
    }
}
