use std::collections::HashSet;

use crate::common::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_a(&self, input: String) -> String {
        let start_packed_len = 4;
        let start_packed_index = input
            .chars()
            .collect::<Vec<_>>()
            .windows(start_packed_len)
            .map(|window| {
                let mut set = HashSet::new();
                for char in window {
                    set.insert(char);
                }

                set.len()
            })
            .position(|set_len| set_len == start_packed_len)
            .unwrap()
            + start_packed_len;

        start_packed_index.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let start_packed_len = 14;
        let start_packed_index = input
            .chars()
            .collect::<Vec<_>>()
            .windows(start_packed_len)
            .map(|window| {
                let mut set = HashSet::new();
                for char in window {
                    set.insert(char);
                }

                set.len()
            })
            .position(|set_len| set_len == start_packed_len)
            .unwrap()
            + start_packed_len;

        start_packed_index.to_string()
    }
}
