use crate::common::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_a(&self, input: String) -> String {
        let result: Option<Vec<_>> = input
            .lines()
            .map(|line| Game::try_from(line).ok())
            .collect();

        let result = result.unwrap();

        let max_cubes = Reveal {
            red: 12,
            green: 13,
            blue: 14,
        };

        let result: u32 = result
            .iter()
            .map(|game| {
                (
                    game.id,
                    game.reveals.iter().any(|reveal| {
                        reveal.red > max_cubes.red
                            || reveal.green > max_cubes.green
                            || reveal.blue > max_cubes.blue
                    }),
                )
            })
            .filter(|game| !game.1)
            .map(|game| game.0 as u32)
            .sum();

        result.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let result: Option<Vec<_>> = input
            .lines()
            .map(|line| Game::try_from(line).ok())
            .collect();

        let result = result.unwrap();

        let result: u32 = result
            .iter()
            .map(|game| {
                let max_red = game.reveals.iter().map(|reveal| reveal.red).max().unwrap() as u32;
                let max_green = game
                    .reveals
                    .iter()
                    .map(|reveal| reveal.green)
                    .max()
                    .unwrap() as u32;
                let max_blue = game.reveals.iter().map(|reveal| reveal.blue).max().unwrap() as u32;

                max_red * max_green * max_blue
            })
            .sum();

        result.to_string()
    }
}

struct Game {
    id: u8,
    reveals: Vec<Reveal>,
}
impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (game_id, reveals) = value.split_once(": ").ok_or(())?;

        let id: u8 = game_id
            .strip_prefix("Game ")
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;

        let reveals: Option<Vec<_>> = reveals
            .split("; ")
            .map(|round| Round::try_from(round).ok())
            .map(|round| round.map(|round| Reveal::from(round)))
            .collect();

        let reveals = reveals.ok_or(())?;

        Ok(Game { id, reveals })
    }
}

struct Round {
    cubes: Vec<(u8, CubeColor)>,
}
impl TryFrom<&str> for Round {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cubes: Option<Vec<_>> = value
            .split(", ")
            .map(|cube_type| {
                cube_type
                    .split_once(" ")
                    .map(|(count, color)| (count.parse::<u8>(), CubeColor::try_from(color)))
            })
            .map(|cube_type| {
                cube_type.and_then(|cube_type| match cube_type {
                    (Ok(count), Ok(color)) => Some((count, color)),
                    _ => None,
                })
            })
            .collect();

        match cubes {
            Some(cubes) => Ok(Round { cubes }),
            None => Err(()),
        }
    }
}

enum CubeColor {
    Red,
    Green,
    Blue,
}
impl TryFrom<&str> for CubeColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

struct Reveal {
    red: u8,
    green: u8,
    blue: u8,
}
impl From<Round> for Reveal {
    fn from(value: Round) -> Self {
        let mut result = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in value.cubes {
            match cube.1 {
                CubeColor::Red => result.red += cube.0,
                CubeColor::Green => result.green += cube.0,
                CubeColor::Blue => result.blue += cube.0,
            }
        }

        result
    }
}
