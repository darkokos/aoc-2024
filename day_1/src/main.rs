use file_reader;
use std::collections::HashMap;

const INPUT_PATH: &str = "input.txt";
const COLUMN_DELIMITER: &str = "   ";

fn a(input: &str) {
    let mut first_list = Vec::with_capacity(1000);
    let mut second_list = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut numbers = line.split(COLUMN_DELIMITER);

        first_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
        second_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
    }

    first_list.sort_unstable();
    second_list.sort_unstable();

    let mut result: u32 = 0;
    for pair in first_list.iter().zip(second_list) {
        result += pair.0.abs_diff(pair.1);
    }

    println!("{result}");
}

fn b(input: &str) {
    let mut first_list = Vec::with_capacity(1000);
    let mut second_list_histogram = HashMap::new();
    for line in input.lines() {
        let mut numbers = line.split(COLUMN_DELIMITER);

        first_list.push(numbers.next().unwrap());
        second_list_histogram
            .entry(numbers.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut result: u32 = 0;
    for number in first_list {
        result += number.parse::<u32>().unwrap() * second_list_histogram.get(number).unwrap_or(&0);
    }

    println!("{result}");
}

fn main() {
    let input = file_reader::read_file(INPUT_PATH);
    a(&input);
    b(&input);
}
