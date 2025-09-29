use crate::shared::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_a(&self, input: String) -> String {
        let reports = parse(input);

        struct ReportState {
            safe: bool,
            decreasing: Option<bool>,
            prev: Option<i64>,
        }

        reports
            .into_iter()
            .map(|report| {
                report.into_iter().fold(
                    ReportState {
                        safe: true,
                        decreasing: None,
                        prev: None,
                    },
                    |mut state, curr| {
                        let safe_distance = match state.prev {
                            None => true,
                            Some(prev) => {
                                let distance = prev.abs_diff(curr);
                                distance >= 1 && distance <= 3
                            }
                        };

                        let same_direction = match (state.prev, state.decreasing) {
                            (Some(prev), Some(decreasing)) => {
                                decreasing && curr < prev || !decreasing && curr > prev
                            }
                            (Some(prev), None) => {
                                state.decreasing = Some(curr < prev);
                                true
                            }
                            _ => true,
                        };

                        ReportState {
                            safe: state.safe && safe_distance && same_direction,
                            decreasing: state.decreasing,
                            prev: Some(curr),
                        }
                    },
                )
            })
            .map(|report| if report.safe { 1 } else { 0 })
            .sum::<i64>()
            .to_string()
    }

    fn part_b(&self, input: String) -> String {
        todo!()
    }
}

fn parse(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i64>().expect("Failed to parse item as number"))
                .collect()
        })
        .collect()
}
