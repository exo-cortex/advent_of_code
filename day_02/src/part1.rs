use crate::parsers::{process_line, SetOfCubes};

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub fn process_1(filename: &str) -> u32 {
    let input_file = std::fs::File::open(&filename).unwrap();
    let buffer = BufReader::new(input_file);

    let a = buffer
        .lines()
        .map(|line| {
            let (_, game) = process_line(&line.unwrap()).unwrap();
            let max_values =
                game.sets_of_cubes
                    .iter()
                    .fold(SetOfCubes::default(), |mut acc, set| {
                        acc.r = acc.r.max(set.r);
                        acc.g = acc.g.max(set.g);
                        acc.b = acc.b.max(set.b);
                        acc
                    });
            if max_values.r <= 12 && max_values.g <= 13 && max_values.b <= 14 {
                game.game_number
            } else {
                0
            }
        })
        .sum::<u32>();

    a
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::{Game, SetOfCubes};

    // #[test]
}
