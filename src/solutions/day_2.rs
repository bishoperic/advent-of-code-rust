use crate::common::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn part_a(&self, input: String) -> String {
        let mut horiz_pos = 0;
        let mut depth = 0;

        let input: Vec<(&str, u32)> = input
            .lines()
            .map(|x| {
                let instr = x.split(" ").collect::<Vec<&str>>();
                (instr[0], instr[1].parse::<u32>().unwrap())
            })
            .collect();

        for instr in input {
            match instr.0 {
                "forward" => horiz_pos += instr.1,
                "up" => depth -= instr.1,
                "down" => depth += instr.1,
                _ => (),
            }
        }

        (horiz_pos * depth).to_string()
    }
    fn part_b(&self, input: String) -> String {
        let mut aim = 0;
        let mut horiz_pos = 0;
        let mut depth = 0;

        let input: Vec<(&str, u32)> = input
            .lines()
            .map(|x| {
                let instr = x.split(" ").collect::<Vec<&str>>();
                (instr[0], instr[1].parse::<u32>().unwrap())
            })
            .collect();

        for instr in input {
            match instr.0 {
                "forward" => {
                    horiz_pos += instr.1;
                    depth += aim * instr.1;
                }
                "up" => aim -= instr.1,
                "down" => aim += instr.1,
                _ => (),
            }
        }

        (horiz_pos * depth).to_string()
    }
}
