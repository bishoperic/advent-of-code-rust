use std::collections::HashSet;

use crate::common::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_a(&self, input: String) -> String {
        let sacks = input.lines().map(|line| Rucksack::new(line));

        let sacks = sacks.map(|sack| {
            let common_item = sack.find_common().unwrap();
            item_to_priority(common_item).unwrap() as i64
        });

        sacks.sum::<i64>().to_string()
    }

    fn part_b(&self, input: String) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}
impl Rucksack {
    fn new(contents: &str) -> Self {
        let (compartment1, compartment2) = contents.split_at(contents.len() / 2);
        let compartment1 = compartment1.to_owned();
        let compartment2 = compartment2.to_owned();

        Self {
            compartment1,
            compartment2,
        }
    }
    fn find_common(&self) -> Option<char> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();

        for item in self.compartment1.chars() {
            set1.insert(item);
        }
        for item in self.compartment2.chars() {
            set2.insert(item);
        }

        let shared: Vec<&char> = set1.intersection(&set2).collect();

        return if shared.len() == 1 {
            Some(*shared[0])
        } else {
            None
        };
    }
}

fn item_to_priority(character: char) -> Option<u32> {
    if character.is_ascii_digit() {
        None
    } else if character.is_ascii_lowercase() {
        character.to_digit(36).map(|x| x - 10 + 1)
    } else if character.is_ascii_uppercase() {
        character.to_digit(36).map(|x| x - 10 + 1 + 26)
    } else {
        None
    }
}
