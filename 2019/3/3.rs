use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let _file_input = parse_file(File::open("input.txt")?)?;
    let mut visited = HashSet::new();
    let mut intersections = HashSet::new();
    let mut intersections2 = HashSet::new();

    let instructions_path1: Vec<Instruction> = _file_input[0]
        .iter()
        .map(|i| {
            let mut j = i.to_string();
            trim_newline(&mut j);

            Instruction {
                direction: j.chars().nth(0).unwrap(),
                step_count: j[1..].parse::<u32>().unwrap()
            }
        })
        .collect();

    let instructions_path2: Vec<Instruction> = _file_input[1]
        .iter()
        .map(|i| {
            let mut j = i.to_string();
            trim_newline(&mut j);

            Instruction {
                direction: j.chars().nth(0).unwrap(),
                step_count: j[1..].parse::<u32>().unwrap()
            }
        })
        .collect();

    walk_path(instructions_path1, &mut visited, &mut intersections, true);
    walk_path(instructions_path2, &mut visited, &mut intersections2, false);

    let min_value = intersections2
        .iter()
        .map(|i: &Point| {
            i.x.abs() + i.y.abs()
        })
        .min()
        .unwrap();

    println!("Found closest intersection: {}", min_value);

    let min_distance = intersections2
        .iter()
        .map(|i: &Point| {
            i.distance
        })
        .min()
        .unwrap();

    println!("Found shortest distance: {}", min_distance);

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);

    Ok(())
}

fn walk_path(instructions: Vec<Instruction>, visited: &mut HashSet<Point>, intersections: &mut HashSet<Point>, is_first_wire: bool) {
    let mut current_x : i32 = 0;
    let mut current_y : i32 = 0;
    let mut distance: u32 = 0;

    for i in instructions.iter() {
        match i.direction {
            'R' => {
                for _j in 0..i.step_count {
                    distance = distance + 1;
                    current_x = current_x + 1;

                    let mut _point = Point{
                        x: current_x,
                        y: current_y,
                        distance: distance
                    };

                    if is_first_wire {
                        visited.insert(_point);
                    } else {
                        if visited.contains(&_point) {
                            _point.distance = _point.distance + visited.get(&_point).unwrap().distance;
                            intersections.insert(_point);
                        }
                    }
                }
            },
            'L' => {
                for _j in 0..i.step_count {
                    distance = distance + 1;
                    current_x = current_x - 1;

                    let mut _point = Point{
                        x: current_x,
                        y: current_y,
                        distance: distance
                    };

                    if is_first_wire {
                        visited.insert(_point);
                    } else {
                        if visited.contains(&_point) {
                            _point.distance = _point.distance + visited.get(&_point).unwrap().distance;
                            intersections.insert(_point);
                        }
                    }
                }
            },
            'D' => {
                for _j in 0..i.step_count {
                    distance = distance + 1;
                    current_y = current_y - 1;

                    let mut _point = Point{
                        x: current_x,
                        y: current_y,
                        distance: distance
                    };

                    if is_first_wire {
                        visited.insert(_point);
                    } else {
                        if visited.contains(&_point) {
                            _point.distance = _point.distance + visited.get(&_point).unwrap().distance;
                            intersections.insert(_point);
                        }
                    }
                }
            },
            'U' => {
                for _j in 0..i.step_count {
                    distance = distance + 1;
                    current_y = current_y + 1;

                    let mut _point = Point{
                        x: current_x,
                        y: current_y,
                        distance: distance
                    };

                    if is_first_wire {
                        visited.insert(_point);
                    } else {
                        if visited.contains(&_point) {
                            _point.distance = _point.distance + visited.get(&_point).unwrap().distance;
                            intersections.insert(_point);
                        }
                    }
                }
            },
            _ => println!("Unknown direction - {}", i.direction)
        }
    }
}

fn parse_file<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let br = BufReader::new(io);
    let mut return_lines = Vec::new();

    for line in br.lines() {
        let lines = line.unwrap().split(",").map(|l| String::from(l)).collect();
        return_lines.push(lines);
    }
    Ok(return_lines)
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

struct Instruction {
    direction: char,
    step_count: u32,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    distance: u32
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}