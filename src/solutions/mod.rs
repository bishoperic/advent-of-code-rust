mod day_1;
mod day_2;
mod day_3;
mod day_4;

use crate::common::Solution;

pub const ALL: [&dyn Solution; 4] = [
    &day_1::Day1 {},
    &day_2::Day2 {},
    &day_3::Day3 {},
    &day_4::Day4 {},
];
