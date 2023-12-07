use std::fs::File;
use std::io::{BufRead, BufReader};

fn task_1(reader: &mut BufReader<File>) -> u32 {
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // print!("{}, ", &line);
        let found_numbers = &line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();
        let combined_number = format!(
            "{}{}",
            found_numbers.first().unwrap().to_string(),
            found_numbers.last().unwrap().to_string()
        )
        .parse::<u32>()
        .unwrap();

        sum += combined_number;
    }
    sum
}

fn main() {
    let mut example_input = BufReader::new(File::open("./example_1.txt").unwrap());
    let mut full_input = BufReader::new(File::open("./puzzle_input.txt").unwrap());

    println!("example data: total sum is {}", task_1(&mut example_input));

    let task_a_number = task_1(&mut full_input);
    println!("input: total sum is {}", task_a_number);
}
