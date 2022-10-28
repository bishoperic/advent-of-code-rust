use std::collections::{HashMap, HashSet};

use crate::common::Solution;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_a(&self, input: String) -> String {
        let mut data: Vec<_> = input
            .lines()
            .map(|line| {
                let line = line.split_once(" | ");

                match line {
                    Some((pattern, output)) => {
                        let pattern: Vec<_> = pattern
                            .split(" ")
                            .map(|activated_wires| DigitPossibility::new(activated_wires))
                            .collect();
                        let output: Vec<_> = output
                            .split(" ")
                            .map(|activated_wires| DigitPossibility::new(activated_wires))
                            .collect();

                        (pattern, output)
                    }
                    None => panic!("Couldn't find delimiter!"),
                }
            })
            .collect();

        let digit_sets: [HashSet<char>; 10] = [
            "abcefg".chars().collect(),
            "cf".chars().collect(),
            "acdeg".chars().collect(),
            "acdfg".chars().collect(),
            "bcdf".chars().collect(),
            "abdfg".chars().collect(),
            "abdefg".chars().collect(),
            "acf".chars().collect(),
            "abcdefg".chars().collect(),
            "abcdfg".chars().collect(),
        ];

        let mut count = 0;
        for line in &mut data {
            for output in &mut line.1 {
                output.collapse_by_num_activated(&digit_sets);

                if output.is_collapsed() {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let mut data: Vec<_> = input
            .lines()
            .map(|line| {
                let line = line.split_once(" | ");

                match line {
                    Some((pattern, output)) => {
                        let pattern: Vec<_> = pattern
                            .split(" ")
                            .map(|activated_wires| DigitPossibility::new(activated_wires))
                            .collect();
                        let output: Vec<_> = output
                            .split(" ")
                            .map(|activated_wires| DigitPossibility::new(activated_wires))
                            .collect();

                        (pattern, output)
                    }
                    None => panic!("Couldn't find delimiter!"),
                }
            })
            .collect();

        let digit_sets: [HashSet<char>; 10] = [
            "abcefg".chars().collect(),
            "cf".chars().collect(),
            "acdeg".chars().collect(),
            "acdfg".chars().collect(),
            "bcdf".chars().collect(),
            "abdfg".chars().collect(),
            "abdefg".chars().collect(),
            "acf".chars().collect(),
            "abcdefg".chars().collect(),
            "abcdfg".chars().collect(),
        ];

        let mut outputs_sum = 0;
        let mut count = 1;
        for line in &mut data {
            let mut wire_possibilities = SegmentPossibilities::new();

            for digit in &mut line.0 {
                digit.collapse_by_num_activated(&digit_sets);
            }

            println!("Problem {}", count);
            count += 1;

            wire_possibilities.collapse_possibilities(&line.0, &digit_sets);

            let signal_wire_to_segment_map: HashMap<_, _> = wire_possibilities
                .segments
                .iter()
                .map(|mapping| {
                    if mapping.1.len() != 1 {
                        panic!("Multiple possibilities remaining while trying to map");
                    }
                    (*mapping.1.iter().next().unwrap(), *mapping.0)
                })
                .collect();

            let mut output_value = 0;
            for digit in &mut line.1 {
                output_value =
                    output_value * 10 + digit.get_digit(&signal_wire_to_segment_map, &digit_sets);
            }

            outputs_sum += output_value;
        }

        outputs_sum.to_string()
    }
}

// for mapping the signal wires outputs to their respective segment inputs
#[derive(Debug)]
struct SegmentPossibilities {
    segments: HashMap<char, HashSet<char>>,
}
impl SegmentPossibilities {
    fn new() -> Self {
        let possibilities: HashSet<_> = ('a'..='g').collect();

        let mut segments = HashMap::new();
        for segment in 'a'..='g' {
            segments.insert(segment, possibilities.clone());
        }

        Self { segments }
    }
    fn collapse_possibilities(
        &mut self,
        digits: &Vec<DigitPossibility>,
        digit_sets: &[HashSet<char>; 10],
    ) {
        for digit in digits {
            // get all of the outputs shared by the digit's remaining possibilities
            let shared_outputs = digit
                .possibilities
                .iter()
                .map(|number| digit_sets[*number].clone())
                .reduce(|accum, number| {
                    accum
                        .intersection(&number)
                        .map(|x| *x)
                        .collect::<HashSet<char>>()
                })
                .unwrap();

            // get all of the signal wire inputs shared by all digits that have the same remaining possibilities
            let shared_inputs = digits
                .clone()
                .iter()
                .filter(|number| number.possibilities == digit.possibilities)
                .map(|number| number.activated_wires.clone())
                .fold(digit.activated_wires.clone(), |accum, current| {
                    accum
                        .intersection(&current)
                        .map(|x| *x)
                        .collect::<HashSet<char>>()
                });

            println!(
                "| {:?}\n| {:?}\n| {:?}\n",
                digit, shared_outputs, shared_inputs
            );

            self.collapse_segments(&shared_outputs, &shared_inputs);
        }
    }
    fn remove_possibilities(&mut self, segment: char, possibilities: &HashSet<char>) {
        let difference: HashSet<char> = self
            .segments
            .get(&segment)
            .unwrap()
            .difference(possibilities)
            .map(|x| *x)
            .collect();

        *self.segments.get_mut(&segment).unwrap() = difference;
    }
    fn collapse_segment(&mut self, segment: char, possibilities: &HashSet<char>) {
        let intersection: HashSet<char> = self
            .segments
            .get(&segment)
            .unwrap()
            .intersection(&possibilities)
            .map(|x| *x)
            .collect();

        *self.segments.get_mut(&segment).unwrap() = intersection;
    }
    fn collapse_segments(&mut self, segments: &HashSet<char>, possibilities: &HashSet<char>) {
        for segment in segments {
            self.collapse_segment(*segment, &possibilities);
        }

        let unused_segments = self
            .segments
            .clone()
            .into_keys()
            .filter(|segment| !segments.contains(segment));

        for segment in unused_segments {
            self.remove_possibilities(segment, possibilities)
        }
    }
}

// for mapping digit outputs to their respective signal inputs
#[derive(Debug)]
struct DigitPossibility {
    activated_wires: HashSet<char>,
    possibilities: HashSet<usize>,
}
impl DigitPossibility {
    fn new(activated_wires: &str) -> Self {
        let activated_wires: HashSet<_> = activated_wires.chars().collect();
        let possibilities = (0..=9).collect();
        Self {
            activated_wires,
            possibilities,
        }
    }
    fn is_collapsed(&self) -> bool {
        self.possibilities.len() == 1
    }
    fn collapse_by_num_activated(&mut self, digit_sets: &[HashSet<char>; 10]) {
        for (i, digit) in digit_sets.iter().enumerate() {
            if self.activated_wires.len() != digit.len() {
                self.possibilities.remove(&i);
            }
        }
    }
    fn signal_to_output(
        input: &HashSet<char>,
        mapping: &HashMap<char, char>,
    ) -> Option<HashSet<char>> {
        let mut transformed = HashSet::new();
        for wire in input {
            transformed.insert(*mapping.get(&wire).unwrap());
        }

        Some(transformed)
    }
    fn output_to_digit(input: &HashSet<char>, digit_sets: &[HashSet<char>; 10]) -> Option<u32> {
        for (i, digit) in digit_sets.iter().enumerate() {
            if digit == input {
                return Some(i as u32);
            }
        }

        None
    }
    fn get_digit(&self, mapping: &HashMap<char, char>, digit_sets: &[HashSet<char>; 10]) -> u32 {
        let output_segments = Self::signal_to_output(&self.activated_wires, mapping)
            .expect("Couldn't map signal wires to output segments!");
        let output_digit = Self::output_to_digit(&output_segments, digit_sets)
            .expect("Output segments did not match a digit");

        output_digit
    }
}
