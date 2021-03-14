use std::collections::HashSet;
use std::convert::TryFrom;
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

fn run_part_one(lines: Vec<&str>) -> usize {
    let instructions = parse_instructions(lines);
    run_core(instructions)
}

fn run_core(instructions: Vec<Instruction>) -> usize {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut idx: i32 = 0;
    let mut accumulator: i32 = 0;

    while !visited.contains(&idx) {
        visited.insert(idx);
        let curr_instr = &instructions[idx as usize];

        match curr_instr.op_code {
            OpCode::NoOp => idx += 1,
            OpCode::Jump => idx += curr_instr.arg_1,
            OpCode::Acc => {
                accumulator += curr_instr.arg_1;
                idx += 1;
            }
        }
    }

    accumulator as usize
}

fn run_core_p2(instructions: Vec<Instruction>) -> usize {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut idx: i32 = 0;
    let mut accumulator: i32 = 0;

    while !visited.contains(&idx) {
        if usize::try_from(idx).unwrap() == instructions.len() {
            println!(
                "Found match, with idx {} and instruction length {}",
                idx,
                instructions.len()
            );
            return accumulator as usize;
        }

        visited.insert(idx);
        let curr_instr = &instructions[idx as usize];

        match curr_instr.op_code {
            OpCode::NoOp => idx += 1,
            OpCode::Jump => idx += curr_instr.arg_1,
            OpCode::Acc => {
                accumulator += curr_instr.arg_1;
                idx += 1;
            }
        }
    }

    if usize::try_from(idx).unwrap() != instructions.len() {
        return 0;
    }

    accumulator as usize
}

fn parse_instructions(lines: Vec<&str>) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in &lines {
        let split_line: Vec<&str> = line.split(' ').collect();
        let instr = Instruction {
            op_code: to_op_code(split_line[0]),
            arg_1: split_line[1].parse::<i32>().unwrap(),
        };

        instructions.push(instr);
    }

    instructions
}

fn run_part_two(lines: Vec<&str>) -> usize {
    let instructions = parse_instructions(lines);

    let mut idx = 0;
    for i in &instructions {
        if i.op_code == OpCode::Acc {
            idx += 1;
            continue;
        }

        if i.op_code == OpCode::NoOp {
            let mut cloned = instructions.clone();
            cloned[idx].op_code = OpCode::Jump;
            let res = run_core_p2(cloned);

            if res != 0 {
                return res;
            }

            idx += 1;
        }

        if i.op_code == OpCode::Jump {
            let mut cloned = instructions.clone();
            cloned[idx].op_code = OpCode::NoOp;
            let res = run_core_p2(cloned);

            if res != 0 {
                return res;
            }

            idx += 1;
        }
    }
    0
}

#[derive(Clone)]
struct Instruction {
    op_code: OpCode,
    arg_1: i32,
}

#[derive(PartialEq, Clone)]
enum OpCode {
    NoOp,
    Acc,
    Jump,
}

fn to_op_code(instr: &str) -> OpCode {
    match instr {
        "nop" => return OpCode::NoOp,
        "jmp" => return OpCode::Jump,
        "acc" => return OpCode::Acc,
        _ => panic!("Unknown op code: {}", instr),
    }
}
