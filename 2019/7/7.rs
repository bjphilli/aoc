extern crate itertools;

use itertools::Itertools;
use std::convert::TryFrom;
use std::fs::File;
use std::collections::vec_deque::VecDeque;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();
    let _program = parse_file(File::open("input.txt")?)?;

    let mut max_thruster_output = 0;

    for i in (0..5).permutations(5) {
        let mut amp1 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };

        let mut amp2 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp3 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp4 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp5 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };

        let mut inputs = VecDeque::new();
        inputs.push_back(i[0]);
        inputs.push_back(0);

        let amp1signal = amp1.run_program(&mut inputs).unwrap();

        let mut inputs2 = VecDeque::new();
        inputs2.push_back(i[1]);
        inputs2.push_back(amp1signal);

        let amp2signal = amp2.run_program(&mut inputs2).unwrap();

        let mut inputs3 = VecDeque::new();
        inputs3.push_back(i[2]);
        inputs3.push_back(amp2signal);

        let amp3signal = amp3.run_program(&mut inputs3).unwrap();

        let mut inputs4 = VecDeque::new();
        inputs4.push_back(i[3]);
        inputs4.push_back(amp3signal);

        let amp4signal = amp4.run_program(&mut inputs4).unwrap();

        let mut inputs5 = VecDeque::new();
        inputs5.push_back(i[4]);
        inputs5.push_back(amp4signal);

        let amp5signal = amp5.run_program(&mut inputs5).unwrap();

        if amp5signal > max_thruster_output {
            max_thruster_output = amp5signal;
        }
    }

    println!("Part 1 answer: {}", max_thruster_output);

    let mut max_thruster_output_p2 = 0;

    for i in (5..10).permutations(5) {
        let mut amp1 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };

        let mut amp2 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp3 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp4 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };
        
        let mut amp5 = Amplifier {
            memory: _program.clone(),
            pointer: 0,
            halted: false
        };

        let mut amp5signal;

        let mut inputs = VecDeque::new();
        inputs.push_back(i[0]);
        inputs.push_back(0);

        let mut inputs2 = VecDeque::new();
        inputs2.push_back(i[1]);

        let mut inputs3 = VecDeque::new();
        inputs3.push_back(i[2]);

        let mut inputs4 = VecDeque::new();
        inputs4.push_back(i[3]);

        let mut inputs5 = VecDeque::new();
        inputs5.push_back(i[4]);

        while !amp5.halted {
            let amp1signal = amp1.run_program(&mut inputs).unwrap();
            inputs2.push_back(amp1signal);
    
            let amp2signal = amp2.run_program(&mut inputs2).unwrap();
            inputs3.push_back(amp2signal);
    
            let amp3signal = amp3.run_program(&mut inputs3).unwrap();
            inputs4.push_back(amp3signal);
    
            let amp4signal = amp4.run_program(&mut inputs4).unwrap();
            inputs5.push_back(amp4signal);
    
            amp5signal = amp5.run_program(&mut inputs5).unwrap();
            inputs.push_back(amp5signal);

            if amp5signal > max_thruster_output_p2 {
                max_thruster_output_p2 = amp5signal;
            }
        }
    }

    println!("Part 2 answer: {}", max_thruster_output_p2);
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

struct Instruction {
    parameter_one_mode: u32,
    parameter_two_mode: u32,
    op_code: u32,
}

impl Instruction {
    pub fn new(instr_str: i64) -> Instruction {
        let input_str = instr_str.to_string();
        let formatted_str = format!("{:0>5}", input_str);

        Instruction {
            parameter_one_mode: formatted_str.chars().nth(2).unwrap().to_digit(10).unwrap(),
            parameter_two_mode: formatted_str.chars().nth(1).unwrap().to_digit(10).unwrap(),
            op_code: formatted_str[3..5].parse::<u32>().unwrap(),
        }
    }
}

struct Amplifier {
    memory: Vec<i64>,
    pointer: usize,
    halted: bool
}

impl Amplifier {
    pub fn run_program(&mut self, input: &mut VecDeque<i64>) -> std::io::Result<i64> {
        let mut output: i64 = 0;
    
        loop {
            let instruction = Instruction::new(self.memory[self.pointer]);
    
            match instruction.op_code {
                1 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let target = usize::try_from(self.memory[self.pointer + 3]).unwrap();
                    self.memory[target] = param1 + param2;
                    self.pointer += 4;
                },
                2 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let target = usize::try_from(self.memory[self.pointer + 3]).unwrap();
                    self.memory[target] = param1 * param2;
                    self.pointer += 4;
                },
                3 => {
                    let position = usize::try_from(self.memory[self.pointer + 1]).unwrap();
                    self.memory[position] = input.pop_front().unwrap();
                    self.pointer += 2;
                },
                4 => {
                    let position = usize::try_from(self.memory[self.pointer + 1]).unwrap();
                    output = self.memory[position];
                    self.pointer += 2;
                    return Ok(output);
                },
                5 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    if param1 != 0 {
                        self.pointer = usize::try_from(param2).unwrap();
                    } else {
                        self.pointer += 3;
                    }
                },
                6 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    if param1 == 0 {
                        self.pointer = usize::try_from(param2).unwrap();
                    } else {
                        self.pointer += 3;
                    }
                },
                7 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3pos = usize::try_from(self.memory[self.pointer + 3]).unwrap();
                    if param1 < param2 {
                        self.memory[param3pos] = 1;
                    } else {
                        self.memory[param3pos] = 0;
                    }
                    self.pointer += 4;
                },
                8 => {
                    let param1 = self.parse_parameter_mode(instruction.parameter_one_mode, self.pointer + 1);
                    let param2 = self.parse_parameter_mode(instruction.parameter_two_mode, self.pointer + 2);
                    let param3pos = usize::try_from(self.memory[self.pointer + 3]).unwrap();
                    if param1 == param2 {
                        self.memory[param3pos] = 1;
                    } else {
                        self.memory[param3pos] = 0;
                    }
                    self.pointer += 4;
                },
                99 => {
                    self.halted = true;
                    break;
                }
                _ => println!("Unknown op code {} at position {}", instruction.op_code, self.pointer)
            }
        }
    
        Ok(output)
    }

    fn parse_parameter_mode(&self, parameter_mode: u32, parameter: usize) -> i64 {
        if parameter_mode == 0 {
            let position = usize::try_from(self.memory[parameter]).unwrap();
            return self.memory[position];
        }
    
        self.memory[parameter]
    }
}