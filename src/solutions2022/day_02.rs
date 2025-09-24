use crate::shared::Solution;

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
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _err => panic!("{:#?}", _err),
            };

            let you = opponent.shape_for_outcome(&you);

            Game { opponent, you }
        });

        let total_score: i64 = games.map(|game| game.score()).sum();

        total_score.to_string()
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

impl Choice {
    fn shape_for_outcome(&self, outcome: &Outcome) -> Choice {
        match (self, outcome) {
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Rock, Outcome::Draw) => Choice::Rock,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Paper, Outcome::Draw) => Choice::Paper,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
            (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}
