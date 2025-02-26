use regex::Regex;

const INPUT_PATH: &str = "input.txt";

fn solve_muls(memory_chunk: &str) -> u32 {
    let mut result: u32 = 0;
    let mul_pattern = Regex::new(r"mul\((0|[1-9][0-9]{0,2}),(0|[1-9][0-9]{0,2})\)").unwrap();
    for m in mul_pattern.find_iter(memory_chunk).map(|m| m.as_str()) {
        let comma_index = m.find(',').unwrap();
        result += &m[4..comma_index].parse::<u32>().unwrap()
            * &m[comma_index + 1..m.len() - 1].parse().unwrap();
    }

    result
}

fn a(input: &str) {
    println!("{}", solve_muls(input));
}

fn b(input: &str) {
    let mut result: u32 = 0;
    let do_split = input.split("do()");
    for memory_chunk in do_split {
        result += solve_muls(memory_chunk.split("don't()").next().unwrap());
    }

    println!("{result}");
}

fn main() {
    let input = file_reader::read_file(INPUT_PATH);
    a(&input);
    b(&input);
}
