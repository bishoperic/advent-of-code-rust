use crate::shared::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_a(&self, input: String) -> String {
        let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let mut numbers = line.split_whitespace();
                let first = numbers.next();
                let second = numbers.next();

                match (first, second) {
                    (Some(first), Some(second)) => (first, second),
                    _ => panic!("Line did not contain two numbers"),
                }
            })
            .map(|(lhs, rhs)| {
                (
                    lhs.parse::<i64>()
                        .expect("Left side could not be parsed as a number"),
                    rhs.parse::<i64>()
                        .expect("Right side could not be parsed as a number"),
                )
            })
            .unzip();

        left_list.sort();
        right_list.sort();

        left_list
            .into_iter()
            .zip(right_list)
            .map(|(lhs, rhs)| lhs.abs_diff(rhs))
            .sum::<u64>()
            .to_string()
    }

    fn part_b(&self, input: String) -> String {
        todo!()
    }
}
