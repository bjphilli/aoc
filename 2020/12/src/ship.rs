pub struct Spaceship {
    direction: Direction,
    x: i32,
    y: i32,
    waypoint_rel_x: i32,
    waypoint_rel_y: i32,
}

pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Spaceship {
    pub fn init() -> Spaceship {
        Spaceship {
            direction: Direction::East,
            x: 0,
            y: 0,
            waypoint_rel_x: 10,
            waypoint_rel_y: 1,
        }
    }

    pub fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    pub fn rotate_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }

    pub fn rotate_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
        }
    }

    pub fn process_command(&mut self, command: char, magnitude: i32) -> () {
        match command {
            'N' => self.y += magnitude,
            'E' => self.x += magnitude,
            'S' => self.y -= magnitude,
            'W' => self.x -= magnitude,
            'R' => {
                let num_rotations = magnitude / 90;
                for _ in 0..num_rotations {
                    self.rotate_right();
                }
            }
            'L' => {
                let num_rotations = magnitude / 90;
                for _ in 0..num_rotations {
                    self.rotate_left();
                }
            }
            'F' => match self.direction {
                Direction::North => self.y += magnitude,
                Direction::East => self.x += magnitude,
                Direction::South => self.y -= magnitude,
                Direction::West => self.x -= magnitude,
            },
            _ => panic!("Unknown command {}", command),
        }
    }

    pub fn rotate_right_p2(&mut self) {
        let tmp = self.waypoint_rel_x;
        self.waypoint_rel_x = self.waypoint_rel_y;
        self.waypoint_rel_y = -1 * tmp;
    }

    pub fn rotate_left_p2(&mut self) {
        let tmp = self.waypoint_rel_x;
        self.waypoint_rel_x = -1 * self.waypoint_rel_y;
        self.waypoint_rel_y = tmp;
    }

    pub fn process_command_p2(&mut self, command: char, magnitude: i32) -> () {
        match command {
            'N' => self.waypoint_rel_y += magnitude,
            'E' => self.waypoint_rel_x += magnitude,
            'S' => self.waypoint_rel_y -= magnitude,
            'W' => self.waypoint_rel_x -= magnitude,
            'R' => {
                let num_rotations = magnitude / 90;
                for _ in 0..num_rotations {
                    self.rotate_right_p2();
                }
            }
            'L' => {
                let num_rotations = magnitude / 90;
                for _ in 0..num_rotations {
                    self.rotate_left_p2();
                }
            }
            'F' => {
                self.x += magnitude * self.waypoint_rel_x;
                self.y += magnitude * self.waypoint_rel_y;
            }
            _ => panic!("Unknown command {}", command),
        }
    }
}
