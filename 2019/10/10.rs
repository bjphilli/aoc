use std::time::{Instant};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let asteroid_belt = 
".###..#######..####..##...#
########.#.###...###.#....#
###..#...#######...#..####.
.##.#.....#....##.#.#.....#
###.#######.###..##......#.
#..###..###.##.#.#####....#
#.##..###....#####...##.##.
####.##..#...#####.#..###.#
#..#....####.####.###.#.###
#..#..#....###...#####..#..
##...####.######....#.####.
####.##...###.####..##....#
#.#..#.###.#.##.####..#...#
..##..##....#.#..##..#.#..#
##.##.#..######.#..#..####.
#.....#####.##........#####
###.#.#######..#.#.##..#..#
###...#..#.#..##.##..#####.
.##.#..#...#####.###.##.##.
...#.#.######.#####.#.####.
#..##..###...###.#.#..#.#.#
.#..#.#......#.###...###..#
#.##.#.#..#.#......#..#..##
.##.##.##.#...##.##.##.#..#
#.###.#.#...##..#####.###.#
#.####.#..#.#.##.######.#..
.#.#####.##...#...#.##...#.";

    let part_one_answer = run_part_one(asteroid_belt);

    println!("Part 1 answer: {}", part_one_answer.asteroids_detected);
//
//    let part_two_answer = run_part_two(&_program);
//    for i in part_two_answer {
//        println!("Part 2 answer: {}", i);
//    }
//
    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn run_part_one(asteroid_field: &str) -> Point {
    let mut x = 0;
    let mut y = 0;
    let mut points = vec![];
    for line in asteroid_field.split('\n') {
        x = 0;
        for c in line.chars() {
            if c == '#' {
                let point = Point {
                    x: x,
                    y: y,
                    asteroids_detected: 0
                };

                points.push(point);
            }
            x = x + 1;
        } 
        y = y + 1;
    }

    let points_clone = points.clone();

    for mut point in &mut points {
        let mut angles = vec![];

        for point2 in &points_clone {
            if point2.x == point.x && point2.y == point.y {
                continue;
            }

            let angle = (point2.y as f32 - point.y as f32).atan2(point2.x as f32 - point.x as f32).to_degrees();

            if !angles.iter().any(|&i| i == angle) {
                point.asteroids_detected = point.asteroids_detected + 1;
                angles.push(angle);
            }
        }
        
        println!("Found asteroid at point ({}, {}) that can see {} other asteroids", point.x, point.y, point.asteroids_detected);
    }

    points.sort_by(|a, b| b.asteroids_detected.cmp(&a.asteroids_detected));
    points.first().cloned().unwrap()
}

fn run_part_two(program: &Vec<i64>) -> Point {
    Point {
        x: 0,
        y: 0,
        asteroids_detected: 0,
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: u32,
    y: u32,
    asteroids_detected: u32,
}

impl Default for Point {
    fn default() -> Point {
        Point {
            x: 0,
            y: 0,
            asteroids_detected: 0
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y
        && self.asteroids_detected == other.asteroids_detected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let asteroid_belt = 
".#..#
.....
#####
....#
...##";

        let output = run_part_one(asteroid_belt);
        let expected = Point {
            x: 3,
            y: 4,
            asteroids_detected: 8
        }; 

        assert_eq!(output, expected); 
    }

    #[test]
    fn test_part_one_answer() {

    }
}
