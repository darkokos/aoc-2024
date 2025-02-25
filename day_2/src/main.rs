const INPUT_PATH: &str = "input.txt";
const COLUMN_DELIMITER: &str = " ";

fn validate_report(numbers: &[&str]) -> bool {
    fn diff_predicate(diff: u8) -> bool {
        1 <= diff && diff <= 3
    }

    let mut last_number = 0;
    let mut is_ascending = false;
    for (i, number) in numbers.iter().enumerate() {
        if i == 0 {
            last_number = number.parse::<u8>().unwrap();
        } else {
            let current_number = number.parse::<u8>().unwrap();
            let diff = last_number as i8 - current_number as i8;
            if i == 1 {
                is_ascending = diff < 0;
            }
            if !diff_predicate(diff.abs() as u8) || (diff < 0) != is_ascending {
                return false;
            }

            last_number = current_number;
        }
    }

    true
}

fn a(input: &str) {
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers = line.split(COLUMN_DELIMITER);

        if validate_report(&numbers.collect::<Vec<&str>>()) {
            safe_count += 1;
        }
    }

    println!("{safe_count}");
}

fn b(input: &str) {
    let mut safe_count = 0;
    'outer: for line in input.lines() {
        let numbers = line.split(COLUMN_DELIMITER);

        if validate_report(&numbers.clone().collect::<Vec<&str>>()) {
            safe_count += 1;
            continue;
        }

        for (i, _) in numbers.clone().enumerate() {
            let mut numbers_copy: Vec<&str> = numbers.clone().collect();
            numbers_copy.remove(i);
            if validate_report(&numbers_copy) {
                safe_count += 1;
                continue 'outer;
            }
        }
    }

    println!("{safe_count}");
}

fn main() {
    let input = file_reader::read_file(INPUT_PATH);
    a(&input);
    b(&input);
}
