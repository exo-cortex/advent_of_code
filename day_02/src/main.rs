use std::{fs::File, io::BufReader};
mod parsers;
mod part1;
mod part2;

use part1::process_1;
use part2::process_2;

pub const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    let a = process_1("./puzzle_input.txt");
    println!("task 1: sum of IDs with mathing cube numbers: {}", a);

    let b = process_2("./puzzle_input.txt");
    println!(
        "task 2: sum of products (`powers`) of minimal possible cubes numbers: {}",
        b
    );
}
