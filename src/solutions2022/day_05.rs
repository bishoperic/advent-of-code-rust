use crate::shared::Solution;

pub struct Day05 {}

impl Solution for Day05 {
    fn part_a(&self, input: String) -> String {
        let mut data = input.split("\n\n");

        let mut stacks = Stacks::new(data.next().unwrap());
        let instructions: Vec<Instruction> = data
            .next()
            .unwrap()
            .lines()
            .map(|line| Instruction::new(line))
            .collect();

        for instruction in instructions {
            stacks.move_stack(instruction, false);
        }

        stacks
            .get_stack_tops()
            .iter()
            .map(|container| container.0)
            .collect()
    }
    fn part_b(&self, input: String) -> String {
        let mut data = input.split("\n\n");

        let mut stacks = Stacks::new(data.next().unwrap());
        let instructions: Vec<Instruction> = data
            .next()
            .unwrap()
            .lines()
            .map(|line| Instruction::new(line))
            .collect();

        for instruction in instructions {
            stacks.move_stack(instruction, true);
        }

        stacks
            .get_stack_tops()
            .iter()
            .map(|container| container.0)
            .collect()
    }
}

#[derive(Debug)]
struct Stacks {
    stack_list: Vec<Vec<Container>>,
}
impl Stacks {
    fn new(input: &str) -> Self {
        let rows = input.lines();

        let containers: Vec<_> = rows
            .map(|row| {
                row.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|possible_crate| {
                        let container: String = possible_crate.iter().collect();
                        let container = container.trim();

                        if container.len() > 0 {
                            if container.starts_with("[") && container.ends_with("]") {
                                Some(Container(container.chars().nth(1).unwrap()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Option<Container>>>()
            })
            .collect();

        let stack_count = containers[0].len();
        let mut stack_list: Vec<Vec<Container>> = Vec::with_capacity(stack_count);

        for _ in 0..stack_count {
            stack_list.push(Vec::new());
        }

        for row in containers {
            for (stack, container) in row.into_iter().enumerate() {
                if let Some(container) = container {
                    stack_list[stack].push(container);
                }
            }
        }

        for stack in &mut stack_list {
            // reading from top to bottom puts the crates in the wrong order,
            // so we must reverse
            stack.reverse()
        }

        Self { stack_list }
    }
    fn move_stack(&mut self, instruction: Instruction, multi_move: bool) {
        let mut move_stack = {
            let starting_stack = self
                .stack_list
                .get_mut(instruction.start_column as usize)
                .unwrap();
            let stack_size = starting_stack.len();

            let move_stack = starting_stack.drain(stack_size - instruction.move_total as usize..);

            move_stack.collect::<Vec<_>>()
        };

        if !multi_move {
            move_stack.reverse();
        }

        let ending_stack = self
            .stack_list
            .get_mut(instruction.end_column as usize)
            .unwrap();

        ending_stack.append(&mut move_stack.to_vec());
    }
    fn get_stack_tops(&self) -> Vec<Container> {
        let mut tops = Vec::new();
        for stack in &self.stack_list {
            tops.push(stack.last().unwrap().to_owned());
        }

        tops
    }
}

#[derive(Debug, Clone, Copy)]
struct Container(char);

#[derive(Debug)]
struct Instruction {
    move_total: u64,
    start_column: u64,
    end_column: u64,
}
impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut instructions = instruction.split_whitespace();

        Self {
            move_total: instructions.nth(1).unwrap().parse().unwrap(),
            // don't forget to subtract 1, because the problem indexes stacks from 1
            start_column: instructions.nth(1).unwrap().parse::<u64>().unwrap() - 1,
            end_column: instructions.nth(1).unwrap().parse::<u64>().unwrap() - 1,
        }
    }
}
