use std::convert::TryFrom;
use std::collections::VecDeque;
use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();
    let _program = parse_file(File::open("input.txt")?)?;

    let part_one_answer = run_part_one(&_program);

    for i in part_one_answer {
        println!("Part 1 answer: {}", i);
    }

    let part_two_answer = run_part_two(&_program);
    for i in part_two_answer {
        println!("Part 2 answer: {}", i);
    }

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;
    line.split(",")
        .map(|l| l.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

fn run_part_one(program: &Vec<i64>) -> Vec<i64> {
    let mut computer = IntCodeComputer::new((&program).to_vec());

    let mut input = VecDeque::new();
    input.push_back(1);

    let mut output = vec![];

    while !computer.halted {
        let output_val = computer.run_program(&mut input).unwrap();

        if !computer.halted {
            output.push(output_val);
        }
    }

    output
}

fn run_part_two(program: &Vec<i64>) -> Vec<i64> {
    let mut computer = IntCodeComputer::new((&program).to_vec());

    let mut input = VecDeque::new();
    input.push_back(2);

    let mut output = vec![];

    while !computer.halted {
        let output_val = computer.run_program(&mut input).unwrap();

        if !computer.halted {
            output.push(output_val);
        }
    }

    output
}

struct Instruction {
    parameter_one_mode: u32,
    parameter_two_mode: u32,
    parameter_three_mode: u32,
    op_code: u32,
}

impl Instruction {
    pub fn new(instr_str: i64) -> Instruction {
        let input_str = instr_str.to_string();
        let formatted_str = format!("{:0>5}", input_str);

        Instruction {
            parameter_three_mode: formatted_str.chars().nth(0).unwrap().to_digit(10).unwrap(),
            parameter_two_mode: formatted_str.chars().nth(1).unwrap().to_digit(10).unwrap(),
            parameter_one_mode: formatted_str.chars().nth(2).unwrap().to_digit(10).unwrap(),
            op_code: formatted_str[3..5].parse::<u32>().unwrap(),
        }
    }
}

struct IntCodeComputer {
    memory: Vec<i64>,
    pointer: usize,
    relative_base: i64,
    halted: bool,
}

impl IntCodeComputer {
    pub fn new(mem: Vec<i64>) -> IntCodeComputer {
        let mut computer = IntCodeComputer {
            memory: mem,
            pointer: 0,
            relative_base: 0,
            halted: false,
        };

        computer.memory.resize(10000000, 0);
        assert_eq!(computer.memory.len(), 10000000);
        computer
    }

    pub fn run_program(&mut self, input: &mut VecDeque<i64>) -> std::io::Result<i64> {
        let mut output: i64 = 0;
    
        loop {
            let instruction = Instruction::new(self.memory[self.pointer]);

            match instruction.op_code {
                1 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3 = self.parse_parameter_mode(instruction.parameter_three_mode, self.pointer + 3);

                    self.memory[param3] = self.memory[param1] + self.memory[param2];
                    self.pointer += 4;
                },
                2 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3 = self.parse_parameter_mode(instruction.parameter_three_mode, self.pointer + 3);
                    
                    self.memory[param3] = self.memory[param1] * self.memory[param2];
                    self.pointer += 4;
                },
                3 => {
                    let position = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let input_val = input.pop_front().unwrap();
                    self.memory[position] = input_val;
                    self.pointer += 2;
                },
                4 => {
                    let position = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    output = self.memory[position];
                    self.pointer += 2;
                    return Ok(output);
                },
                5 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    if self.memory[param1] != 0 {
                        self.pointer = usize::try_from(self.memory[param2]).unwrap();
                    } else {
                        self.pointer += 3;
                    }
                },
                6 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    if self.memory[param1] == 0 {
                        self.pointer = usize::try_from(self.memory[param2]).unwrap();
                    } else {
                        self.pointer += 3;
                    }
                },
                7 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3 = self.parse_parameter_mode(instruction.parameter_three_mode, self.pointer + 3);
                    if self.memory[param1] < self.memory[param2] {
                        self.memory[param3] = 1;
                    } else {
                        self.memory[param3] = 0;
                    }
                    self.pointer += 4;
                },
                8 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3 = self.parse_parameter_mode(instruction.parameter_three_mode, self.pointer + 3);
                    if self.memory[param1] == self.memory[param2] {
                        self.memory[param3] = 1;
                    } else {
                        self.memory[param3] = 0;
                    }
                    self.pointer += 4;
                },
                9 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    self.relative_base += self.memory[param1];
                    self.pointer += 2;
                },
                99 => {
                    self.halted = true;
                    break;
                },
                _ => println!("Unknown op code {} at position {}", instruction.op_code, self.pointer)
            }
        }
    
        Ok(output)
    }

    fn parse_parameter_mode(&self, parameter_mode: u32, parameter: usize) -> usize {
        if parameter_mode == 0 {
            return usize::try_from(self.memory[parameter]).unwrap();
        } else if parameter_mode == 1 {
            return parameter;
        }

        let position2 = self.memory[parameter];
        usize::try_from(position2 + self.relative_base).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_copy_itself() {
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];

        let output = run_part_one(&program);
        assert_eq!(output, program);
    }

    #[test]
    fn test_part_one_large_number() {
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        assert_eq!(run_part_one(&program), vec![1219070632396864]);
    }

    #[test]
    fn test_part_one_large_number_2() {
        let program = vec![104,1125899906842624,99];
        assert_eq!(run_part_one(&program), vec![1125899906842624]);
    }
}
