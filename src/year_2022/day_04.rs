use crate::shared::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_a(&self, input: String) -> String {
        let elves: Vec<_> = input
            .lines()
            .map(|line| {
                let pair_assignments: Vec<_> = line
                    .split(",")
                    .map(|range| {
                        let range: Vec<_> = range
                            .split("-")
                            .map(|num| num.parse::<i64>().unwrap())
                            .collect();
                        Assigments {
                            start: range[0],
                            end: range[1],
                        }
                    })
                    .collect();
                ElfPair {
                    winner: pair_assignments[0],
                    loser: pair_assignments[1],
                }
            })
            .collect();

        let mut count = 0;
        for pair in elves {
            if pair.winner.is_contained_within(pair.loser)
                || pair.loser.is_contained_within(pair.winner)
            {
                count += 1;
            }
            // println!(
            //     "Winner is contained in {:?} | Loser is contained within: {:?}",
            //     pair.winner.is_contained_within(pair.loser),
            //     pair.loser.is_contained_within(pair.winner)
            // )
        }

        count.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let elves: Vec<_> = input
            .lines()
            .map(|line| {
                let pair_assignments: Vec<_> = line
                    .split(",")
                    .map(|range| {
                        let range: Vec<_> = range
                            .split("-")
                            .map(|num| num.parse::<i64>().unwrap())
                            .collect();
                        Assigments {
                            start: range[0],
                            end: range[1],
                        }
                    })
                    .collect();
                ElfPair {
                    winner: pair_assignments[0],
                    loser: pair_assignments[1],
                }
            })
            .collect();

        let mut count = 0;
        for pair in elves {
            if pair.winner.overlaps_with(pair.loser) {
                count += 1;
            }
            // println!(
            //     "Winner is contained in {:?} | Loser is contained within: {:?}",
            //     pair.winner.is_contained_within(pair.loser),
            //     pair.loser.is_contained_within(pair.winner)
            // )
        }

        count.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
struct Assigments {
    start: i64,
    end: i64,
}
impl Assigments {
    fn is_contained_within(&self, other: Assigments) -> bool {
        let inside_upper_bounds = self.end - other.end <= 0;
        let inside_lower_bounds = self.start - other.start >= 0;

        inside_upper_bounds && inside_lower_bounds
    }
    fn overlaps_with(&self, other: Assigments) -> bool {
        let upper_and_lower_overlap = self.end - other.start >= 0;
        let lower_and_upper_overlap = self.start - other.end <= 0;

        upper_and_lower_overlap && lower_and_upper_overlap
    }
}

struct ElfPair {
    winner: Assigments,
    loser: Assigments,
}
