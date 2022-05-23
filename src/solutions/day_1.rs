use crate::common::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn part_a(&self, input: String) -> String {
        let data: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

        data.windows(2).filter(|x| x[1] > x[0]).count().to_string()
    }
    fn part_b(&self, input: String) -> String {
        let data: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

        data.windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|x| x[1] > x[0])
            .count()
            .to_string()
    }
}
