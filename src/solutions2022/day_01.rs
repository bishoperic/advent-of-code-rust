use crate::common::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_a(&self, input: String) -> String {
        let mut elf_calories: Vec<Vec<i64>> = Vec::new();

        for line in input.split("\n\n") {
            elf_calories.push(Vec::new());

            if let Some(current_elf) = elf_calories.last_mut() {
                for calories in line.lines() {
                    current_elf.push(calories.trim().parse().unwrap());
                }
            }
        }

        let elf_calories: Vec<i64> = elf_calories
            .iter()
            .map(|elf| elf.iter().sum::<i64>())
            .collect();

        elf_calories.iter().max().unwrap().to_string()
    }

    fn part_b(&self, input: String) -> String {
        let mut elf_calories: Vec<Vec<i64>> = Vec::new();

        for line in input.split("\n\n") {
            elf_calories.push(Vec::new());

            if let Some(current_elf) = elf_calories.last_mut() {
                for calories in line.lines() {
                    current_elf.push(calories.trim().parse().unwrap());
                }
            }
        }

        let mut elf_calories: Vec<i64> = elf_calories
            .iter()
            .map(|elf| elf.iter().sum::<i64>())
            .collect();

        elf_calories.sort();
        let len = elf_calories.len();
        let sum = elf_calories[len - 1] + elf_calories[len - 2] + elf_calories[len - 3];

        sum.to_string()
    }
}
