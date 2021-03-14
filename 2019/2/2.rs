use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();
    let _program = parse_file(File::open("input.txt")?)?;

    let part_one_answer = run_program(_program.clone(), 12, 2)?;
    println!("Part 1 answer: {}", part_one_answer);

    'outer: for i in 0..99 {
        'inner: for j in 0..99 {
            let part_two_answer = run_program(_program.clone(), i, j)?;
            if part_two_answer == 19690720 {
                println!("Part 2 answer: {}", 100 * i + j);
                break 'outer;
            }
        }
    }

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;
    line.split(",")
        .map(|l| l.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

fn run_program(mut program:  Vec<usize>, noun: usize, verb: usize) -> std::io::Result<(usize)> {
    let mut pointer = 0;

    program[1] = noun;
    program[2] = verb;

    loop {
        let instruction = program[pointer];

        match instruction {
            1 => {
                let param1 = program[pointer + 1];
                let param2 = program[pointer + 2];
                let target = program[pointer + 3];
                program[target] = program[param2] + program[param1]
            },
            2 => {
                let param1 = program[pointer + 1];
                let param2 = program[pointer + 2];
                let target = program[pointer + 3];
                program[target] = program[param2] * program[param1]
            },
            99 => break,
            _ => println!("Unknown op code {} at position {}", program[pointer], pointer)
        };

        pointer += 4;
    }

    Ok(program[0])
}