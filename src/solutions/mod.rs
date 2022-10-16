mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::common::Solution;

pub const ALL: [&dyn Solution; 9] = [
    &day_1::Day1 {},
    &day_2::Day2 {},
    &day_3::Day3 {},
    &day_4::Day4 {},
    &day_5::Day5 {},
    &day_6::Day6 {},
    &day_7::Day7 {},
    &day_8::Day8 {},
    &day_9::Day9 {},
];
