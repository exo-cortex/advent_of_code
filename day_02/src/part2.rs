use crate::parsers::{process_line, SetOfCubes};

use std::io::{BufRead, BufReader};

pub fn process_2(filename: &str) -> u32 {
    let input_file = std::fs::File::open(&filename).unwrap();
    let buffer = BufReader::new(input_file);

    let a = buffer
        .lines()
        .map(|line| {
            let game = process_line(&line.unwrap()).unwrap().1;
            let min_values =
                game.sets_of_cubes
                    .iter()
                    .fold(SetOfCubes::default(), |mut acc, set| {
                        acc.r = acc.r.max(set.r);
                        acc.g = acc.g.max(set.g);
                        acc.b = acc.b.max(set.b);
                        acc
                    });

            let power = min_values.r * min_values.g * min_values.b;
            power
        })
        .sum::<u32>();

    a
}

#[cfg(test)]
mod test {
    use crate::parsers::{Game, SetOfCubes};

    use super::*;

    #[test]
    fn game_numbers() -> Result<(), String> {
        assert_eq!(
            process_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            (
                "",
                Game {
                    game_number: 1,
                    sets_of_cubes: vec![
                        SetOfCubes { r: 4, g: 0, b: 3 },
                        SetOfCubes { r: 1, g: 2, b: 6 },
                        SetOfCubes { r: 0, g: 2, b: 0 }
                    ]
                }
            )
        );
        assert_eq!(
            process_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap(),
            (
                "",
                Game {
                    game_number: 2,
                    sets_of_cubes: vec![
                        SetOfCubes { r: 0, g: 2, b: 1 },
                        SetOfCubes { r: 1, g: 3, b: 4 },
                        SetOfCubes { r: 0, g: 1, b: 1 }
                    ]
                }
            )
        );
        assert_eq!(
            process_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
            .unwrap(),
            (
                "",
                Game {
                    game_number: 3,
                    sets_of_cubes: vec![
                        SetOfCubes { r: 20, g: 8, b: 6 },
                        SetOfCubes { r: 4, g: 13, b: 5 },
                        SetOfCubes { r: 1, g: 5, b: 0 }
                    ]
                }
            )
        );
        assert_eq!(
            process_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )
            .unwrap(),
            (
                "",
                Game {
                    game_number: 4,
                    sets_of_cubes: vec![
                        SetOfCubes { r: 3, g: 1, b: 6 },
                        SetOfCubes { r: 6, g: 3, b: 0 },
                        SetOfCubes { r: 14, g: 3, b: 15 }
                    ]
                }
            )
        );
        assert_eq!(
            process_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
            (
                "",
                Game {
                    game_number: 5,
                    sets_of_cubes: vec![
                        SetOfCubes { r: 6, g: 3, b: 1 },
                        SetOfCubes { r: 1, g: 2, b: 2 },
                    ]
                }
            )
        );

        Ok(())
    }
}
