mod day_1;
mod day_2;

use crate::common::Solution;

pub const ALL: [&dyn Solution; 2] = [&day_1::Day1 {}, &day_2::Day2 {}];
