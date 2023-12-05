use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        print!("{}, ", &line);
        let numbers = &line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| (*c.to_string()).parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        print!("{:?}, ", numbers);
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        println!("[first: {}, last: {}]", first, last);
    }
}
