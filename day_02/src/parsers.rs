use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::digit1,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

pub struct Game {
    pub game_number: u32,
    pub sets_of_cubes: Vec<SetOfCubes>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.game_number == other.game_number
            && self.sets_of_cubes.len() == other.sets_of_cubes.len()
            && self
                .sets_of_cubes
                .iter()
                .zip(&other.sets_of_cubes)
                .all(|(s, o)| *s == *o)
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("game_number", &self.game_number)
            .field(
                "sets_of_cubes",
                &self
                    .sets_of_cubes
                    .iter()
                    .map(|set| format!("{:?}", set))
                    .collect::<String>(),
            )
            .finish()
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct SetOfCubes {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub fn process_line(i: &str) -> IResult<&str, Game> {
    let (i, o) = delimited(tag("Game "), is_not(": "), tag(": "))(i)?;
    let game_number = o.parse::<u32>().unwrap();

    let (i, o) = separated_list0(tag("; "), is_not(";"))(i)?;
    let mut cubesets = Vec::<SetOfCubes>::new();

    for &item in &o {
        let mut cube_set = SetOfCubes::default();

        let (_, out) = separated_list0(tag(", "), is_not(","))(item)?;

        for set in out {
            let colors = alt((tag("red"), tag("green"), tag("blue")));
            let (_, (amount, col)) = separated_pair(digit1, tag(" "), colors)(set)?;
            match col {
                "red" => cube_set.r = amount.parse::<u32>().unwrap(),
                "green" => cube_set.g = amount.parse::<u32>().unwrap(),
                "blue" => cube_set.b = amount.parse::<u32>().unwrap(),
                _ => unreachable!(),
            }
        }
        cubesets.push(cube_set);
    }

    let game = Game {
        game_number,
        sets_of_cubes: cubesets,
    };

    Ok((i, game))
}

#[cfg(test)]
mod test {
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
