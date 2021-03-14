use std::convert::TryFrom;
use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();
    let _program = parse_file(File::open("input.txt")?)?;

    let part_one_answer = run_program(_program.clone(), 1)?;
    println!("Part 1 answer: {}", part_one_answer);

    let part_two_answer = run_program(_program.clone(), 5)?;
    println!("Part 2 answer: {}", part_two_answer);

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;
    line.split(",")
        .map(|l| l.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

fn run_program(mut program:  Vec<i32>, input: i32) -> std::io::Result<i32> {
    let mut pointer : usize = 0;
    let mut output: i32 = 0;

    loop {
        let instruction = Instruction::new(program[pointer]);

        match instruction.op_code {
            1 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                let target = usize::try_from(program[pointer + 3]).unwrap();
                program[target] = param1 + param2;
                pointer += 4;
            },
            2 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                let target = usize::try_from(program[pointer + 3]).unwrap();
                program[target] = param1 * param2;
                pointer += 4;
            },
            3 => {
                let position = usize::try_from(program[pointer + 1]).unwrap();
                program[position] = input;
                pointer += 2;
            },
            4 => {
                let position = usize::try_from(program[pointer + 1]).unwrap();
                output = program[position];
                pointer += 2;
            },
            5 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                if param1 != 0 {
                    pointer = usize::try_from(param2).unwrap();
                } else {
                    pointer += 3;
                }
            },
            6 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                if param1 == 0 {
                    pointer = usize::try_from(param2).unwrap();
                } else {
                    pointer += 3;
                }
            },
            7 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                let param3pos = usize::try_from(program[pointer + 3]).unwrap();
                if param1 < param2 {
                    program[param3pos] = 1;
                } else {
                    program[param3pos] = 0;
                }
                pointer += 4;
            },
            8 => {
                let param1 = parse_parameter_mode(&program, instruction.parameter_one_mode, pointer + 1);
                let param2 = parse_parameter_mode(&program, instruction.parameter_two_mode, pointer + 2);
                let param3pos = usize::try_from(program[pointer + 3]).unwrap();
                if param1 == param2 {
                    program[param3pos] = 1;
                } else {
                    program[param3pos] = 0;
                }
                pointer += 4;
            }
            99 => break,
            _ => println!("Unknown op code {} at position {}", instruction.op_code, pointer)
        };
    }

    Ok(output)
}

fn parse_parameter_mode(program: &Vec<i32>, parameter_mode: u32, parameter: usize) -> i32 {
    if parameter_mode == 0 {
        let position = usize::try_from(program[parameter]).unwrap();
        return program[position];
    }

    program[parameter]
}

struct Instruction {
    parameter_one_mode: u32,
    parameter_two_mode: u32,
    op_code: u32,
}

impl Instruction {
    pub fn new(instr_str: i32) -> Instruction {
        let input_str = instr_str.to_string();
        let formatted_str = format!("{:0>5}", input_str);

        Instruction {
            parameter_one_mode: formatted_str.chars().nth(2).unwrap().to_digit(10).unwrap(),
            parameter_two_mode: formatted_str.chars().nth(1).unwrap().to_digit(10).unwrap(),
            op_code: formatted_str[3..5].parse::<u32>().unwrap(),
        }
    }
}