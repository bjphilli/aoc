
use std::fs::File;
use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> std::io::Result<()> {
    let begin = Instant::now();
    let _img = parse_file(File::open("input.txt")?)?;

    let img_len = 25;
    let img_width = 6;

    let mut counter = 0;
    let mut layer_counter = 0;
    let mut layer = Layer{zero_count: 0, one_count: 0, two_count: 0};
    let mut lowest_zero_count = 9999999999999;
    let mut answer1 = 0;

    let mut answer2_array: [char; 25 * 6] = ['2'; 25 * 6];

    for i in _img.iter() {
        if counter >= img_len * img_width {
            if layer.zero_count < lowest_zero_count {
                lowest_zero_count = layer.zero_count;
                answer1 = layer.one_count * layer.two_count;
            }

            counter = 0;
            layer_counter = layer_counter + 1;
            layer = Layer{zero_count: 0, one_count: 0, two_count: 0};
        }

        match i {
            '0' => layer.zero_count = layer.zero_count + 1,
            '1' => layer.one_count = layer.one_count + 1,
            '2' => layer.two_count = layer.two_count + 1,
            _ => {}
        }

        if answer2_array[counter] == '2' {
            answer2_array[counter] = *i;
        }

        counter = counter + 1;
    }

    println!("Part 1 answer: {}", answer1);
    println!("Part 2 answer:");

    for i in 0..6 {
        for j in 0..25 {
            if (answer2_array[(i * 25) + j] == '0') {
                print!("{}", '\u{2588}');
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<char>, Error> {
    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;
    Ok(line.chars().collect())
}

struct Layer {
    zero_count: u64,
    one_count: u64,
    two_count: u64
}