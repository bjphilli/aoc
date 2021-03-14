use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let _ints = parse_file(File::open("input.txt")?)?;

    let sum: u32 = _ints.iter().map(|s| {
        (s / 3) - 2
    }).sum();

    println!("Part 1 answer: {}", sum);

    let sum2: u32 = _ints.iter().map(|s| {
        calculate_fuel(s)
    }).sum();

    println!("Part 2 answer: {}", sum2);

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

fn calculate_fuel(mass: &u32) -> u32 {
    let mut fuel_to_haul = ((mass / 3) - 2) as i32;
    let mut total_fuel = fuel_to_haul as i32;

   loop {
        let fuel: i32 = (fuel_to_haul / 3) - 2;
        if fuel <= 0 {
            break;
        }

        total_fuel = total_fuel + fuel;
        fuel_to_haul = fuel;
    }

    total_fuel as u32
}