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

fn task_2(reader: &mut BufReader<File>) -> u32 {
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        // stole this part from chris biscardi
        let found_numbers = &line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();

        let combined_number = format!(
            "{}{}",
            found_numbers.first().unwrap().to_string(),
            found_numbers.last().unwrap().to_string()
        )
        .parse::<u32>()
        .expect("should be a u32 number");

        sum += combined_number;
    }
    sum
}

fn main() {
    let mut _example_input = BufReader::new(File::open("./example_1.txt").unwrap());
    let mut full_input = BufReader::new(File::open("./puzzle_input.txt").unwrap());

    let task_a_number = task_2(&mut full_input);
    println!("input: total sum is {}", task_a_number);
}
