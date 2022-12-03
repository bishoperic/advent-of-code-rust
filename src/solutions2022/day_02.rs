use crate::common::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_a(&self, input: String) -> String {
        let games = input.lines().map(|line| {
            let mut options = line.split(" ");
            let opponent = options.next().unwrap();
            let you = options.next().unwrap();

            let opponent = match opponent {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissors,
                _err => panic!("{:#?}", _err),
            };
            let you = match you {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Scissors,
                _err => panic!("{:#?}", _err),
            };
            Game { opponent, you }
        });

        let total_score: i64 = games.map(|game| game.score()).sum();

        total_score.to_string()
    }

    fn part_b(&self, input: String) -> String {
        todo!()
    }
}

struct Game {
    opponent: Choice,
    you: Choice,
}
impl Game {
    fn score(&self) -> i64 {
        match (&self.you, &self.opponent) {
            (Choice::Rock, Choice::Rock) => 3 + 1,
            (Choice::Rock, Choice::Paper) => 0 + 1,
            (Choice::Rock, Choice::Scissors) => 6 + 1,
            (Choice::Paper, Choice::Rock) => 6 + 2,
            (Choice::Paper, Choice::Paper) => 3 + 2,
            (Choice::Paper, Choice::Scissors) => 0 + 2,
            (Choice::Scissors, Choice::Rock) => 0 + 3,
            (Choice::Scissors, Choice::Paper) => 6 + 3,
            (Choice::Scissors, Choice::Scissors) => 3 + 3,
        }
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}
