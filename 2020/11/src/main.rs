use std::collections::HashSet;
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
    let mut floor_space = map_input_to_floor(lines);

    let mut change_count = -1;

    while change_count != 0 {
        let new_floor_space = floor_space.clone();
        change_count = 0;

        for i in 0..new_floor_space.len() {
            let current_row = &new_floor_space[i];
            for j in 0..current_row.len() {
                let current_status = &current_row[j];
                if current_status == &GridStatus::Floor {
                    continue;
                }

                let neighbor_occupied = count_neighbors(&new_floor_space, i, j);
                if current_status == &GridStatus::Open && neighbor_occupied == 0 {
                    floor_space[i][j] = GridStatus::Occupied;
                    change_count += 1;
                } else if current_status == &GridStatus::Occupied && neighbor_occupied >= 4 {
                    floor_space[i][j] = GridStatus::Open;
                    change_count += 1;
                }
            }
        }
    }

    floor_space
        .iter()
        .flatten()
        .collect::<Vec<&GridStatus>>()
        .iter()
        .filter(|f| ***f == GridStatus::Occupied)
        .collect::<Vec<&&GridStatus>>()
        .len()
}

fn run_part_two(lines: Vec<&str>) -> usize {
    let mut floor_space = map_input_to_floor(lines);

    let mut change_count = -1;

    while change_count != 0 {
        let new_floor_space = floor_space.clone();
        change_count = 0;

        for i in 0..new_floor_space.len() {
            let current_row = &new_floor_space[i];
            for j in 0..current_row.len() {
                let current_status = &current_row[j];
                if current_status == &GridStatus::Floor {
                    continue;
                }

                let neighbor_occupied = count_neighbors_p2(&new_floor_space, i, j);
                if current_status == &GridStatus::Open && neighbor_occupied == 0 {
                    floor_space[i][j] = GridStatus::Occupied;
                    change_count += 1;
                } else if current_status == &GridStatus::Occupied && neighbor_occupied >= 5 {
                    floor_space[i][j] = GridStatus::Open;
                    change_count += 1;
                }
            }
        }
    }

    floor_space
        .iter()
        .flatten()
        .collect::<Vec<&GridStatus>>()
        .iter()
        .filter(|f| ***f == GridStatus::Occupied)
        .collect::<Vec<&&GridStatus>>()
        .len()
}

fn count_neighbors(floor: &Vec<Vec<GridStatus>>, x: usize, y: usize) -> usize {
    let mut neighbor_count = 0;

    let mut neighbors = HashSet::new();
    neighbors.insert(0);
    neighbors.insert(1);
    neighbors.insert(2);
    neighbors.insert(3);
    neighbors.insert(4);
    neighbors.insert(5);
    neighbors.insert(6);
    neighbors.insert(7);

    if x <= 0 {
        neighbors.remove(&0);
        neighbors.remove(&1);
        neighbors.remove(&2);
    }

    if &x == &(floor.len() - 1) {
        neighbors.remove(&5);
        neighbors.remove(&6);
        neighbors.remove(&7);
    }

    if y <= 0 {
        neighbors.remove(&0);
        neighbors.remove(&3);
        neighbors.remove(&5);
    }

    if &y == &(floor[x].len() - 1) {
        neighbors.remove(&2);
        neighbors.remove(&4);
        neighbors.remove(&7);
    }

    if neighbors.contains(&0) {
        if &floor[x - 1][y - 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&1) {
        if &floor[x - 1][y] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&2) {
        if &floor[x - 1][y + 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&3) {
        if &floor[x][y - 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&4) {
        if &floor[x][y + 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&5) {
        if &floor[x + 1][y - 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&6) {
        if &floor[x + 1][y] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    if neighbors.contains(&7) {
        if &floor[x + 1][y + 1] == &GridStatus::Occupied {
            neighbor_count += 1;
        }
    }

    neighbor_count
}

fn count_neighbors_p2(floor: &Vec<Vec<GridStatus>>, x: usize, y: usize) -> usize {
    let mut neighbor_count = 0;

    let mut curr_x = x;
    let mut curr_y = y;

    //upper left
    while curr_x > 0 && curr_y > 0 {
        curr_x -= 1;
        curr_y -= 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //straight up
    while curr_x > 0 {
        curr_x -= 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //upper right
    while curr_x > 0 && curr_y < &floor[x].len() - 1 {
        curr_x -= 1;
        curr_y += 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //straight right
    while curr_y < &floor[x].len() - 1 {
        curr_y += 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //lower right
    while curr_x < &floor.len() - 1 && curr_y < &floor[x].len() - 1 {
        curr_x += 1;
        curr_y += 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //straight down
    while curr_x < &floor.len() - 1 {
        curr_x += 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //lower left
    while curr_x < &floor.len() - 1 && curr_y > 0 {
        curr_x += 1;
        curr_y -= 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }

    curr_x = x;
    curr_y = y;

    //straight left
    while curr_y > 0 {
        curr_y -= 1;

        if &floor[curr_x][curr_y] == &GridStatus::Open {
            break;
        }

        if &floor[curr_x][curr_y] == &GridStatus::Occupied {
            neighbor_count += 1;
            break;
        }
    }
    neighbor_count
}

fn map_input_to_floor(lines: Vec<&str>) -> Vec<Vec<GridStatus>> {
    let mut floor = vec![];

    for line in &lines {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '.' => row.push(GridStatus::Floor),
                '#' => row.push(GridStatus::Occupied),
                'L' => row.push(GridStatus::Open),
                _ => panic!("Unknown seat status: {}", c),
            }
        }

        floor.push(row);
    }

    floor
}

#[derive(Clone, PartialEq)]
enum GridStatus {
    Floor,
    Occupied,
    Open,
}
