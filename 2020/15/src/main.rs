use std::collections::HashMap;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let begin = Instant::now();

    let _ints = vec![14, 3, 1, 0, 9, 5];
    let part_one_answer = run_part_one(_ints.clone());

    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = run_part_two(_ints.clone());
    println!("Part two answer: {}", part_two_answer);

    let end = Instant::now();
    let diff = end.duration_since(begin);
    println!("Total run time: {:?}", diff);
    Ok(())
}

fn run_part_one(ints: Vec<usize>) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    let _threshold = ints.len();

    let mut counter = 0;
    let mut input = 0;

    while counter < 2020 {
        let next_val = find_next_val(&input, &counter, &ints, &memory);
        println!("Found next value {} on turn {}", next_val + 1, counter);
        memory.insert(next_val, counter);
        input = next_val;
        counter += 1;
    }

    input
}

fn find_next_val(
    input: &usize,
    counter: &usize,
    initial_seq: &Vec<usize>,
    memory: &HashMap<usize, usize>,
) -> usize {
    if *counter < initial_seq.len() {
        return initial_seq[*counter];
    }

    if !memory.contains_key(input) {
        return 0;
    }

    counter - memory.get(input).unwrap()
}

fn run_part_two(lines: Vec<usize>) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    0
}

mod test {
    use super::*;

    #[test]
    fn p1_test_1() {
        let input = vec![0, 3, 6];

        assert_eq!(run_part_one(input), 436);
    }
}
