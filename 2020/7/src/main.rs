use std::collections::HashMap;
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
    let bags = get_bags(lines);

    bags.iter()
        .filter(|b| b.color != "shiny gold" && contains_shiny_gold(b.color, &bags))
        .count()
}

fn contains_shiny_gold(color: &str, bags: &Vec<Bag>) -> bool {
    if color == "shiny gold" {
        return true;
    }
    let bag = bags.iter().find(|c| c.color == color).unwrap();

    bag.bags.keys().any(|c| contains_shiny_gold(c, &bags))
}

fn get_bags(lines: Vec<&str>) -> Vec<Bag> {
    let mut bags: Vec<Bag> = vec![];

    for line in &lines {
        let split_lines: Vec<&str> = line.split(" bags contain ").collect::<Vec<&str>>();
        let bag_color = split_lines.first().unwrap();

        let mut bag = Bag {
            color: bag_color,
            bags: HashMap::new(),
        };

        let second = split_lines[1];

        if second == "no other bags." {
            bags.push(bag);
            continue;
        }

        let split_suffix: Vec<&str> = second.split(",").collect();

        for suff in &split_suffix {
            let split_by_bag = &suff.trim().split("bag").collect::<Vec<&str>>();

            let color_and_number = split_by_bag.first().unwrap();

            let color_and_number_string = &color_and_number.to_string();
            let number = &color_and_number_string[0..1];
            let color = &color_and_number_string[2..].trim().to_string();

            &bag.bags
                .insert(color.to_string(), number.parse::<usize>().unwrap());
        }

        bags.push(bag);
    }

    bags
}

fn run_part_two(lines: Vec<&str>) -> usize {
    let bags = get_bags(lines);
    count_bags("shiny gold", &bags) - 1
}

fn count_bags(color: &str, bags: &Vec<Bag>) -> usize {
    let bag = bags.iter().find(|c| c.color == color).unwrap();

    if bag.bags.keys().count() == 0 {
        return 1;
    }

    let sum: usize = bag
        .bags
        .iter()
        .map(|(col, num)| num * count_bags(col, &bags))
        .sum();

    sum + 1
}

struct Bag<'a> {
    color: &'a str,
    bags: HashMap<String, usize>,
}
