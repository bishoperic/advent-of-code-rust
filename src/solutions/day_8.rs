use crate::common::Solution;

// const digit_segments: [u8] = [0b01110111, 0b00010010, 0b01011101, 0b01011011];
const DIGIT_SEGMENTS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

pub struct Day8 {}

impl Solution for Day8 {
    fn part_a(&self, input: String) -> String {
        let data = input
            .lines()
            .map(|line| {
                let line = line
                    .split(" | ")
                    .map(|x| {
                        x.split(" ")
                            .map(|digit| Digit::new(digit))
                            .collect::<Vec<Digit>>()
                    })
                    .collect::<Vec<_>>();

                let ref patterns = line[0];
                let ref output = line[1];

                Display {
                    patterns: patterns.to_vec(),
                    output: output.to_vec(),
                }
            })
            .collect::<Vec<_>>();

        todo!()
    }
    fn part_b(&self, input: String) -> String {
        todo!()
    }
}

#[derive(Copy, Clone)]
struct Digit {
    segments: [bool; 7],
}
impl Digit {
    fn new(segments_active: &str) -> Digit {
        let mut segments = [false; 7];
        for char in segments_active.chars() {
            // not great
            match char {
                'a' => segments[0] = true,
                'b' => segments[1] = true,
                'c' => segments[2] = true,
                'd' => segments[3] = true,
                'e' => segments[4] = true,
                'f' => segments[5] = true,
                'g' => segments[6] = true,
                _ => (),
            }
        }
        Digit { segments }
    }
}

struct Display {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}
