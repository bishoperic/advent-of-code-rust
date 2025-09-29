mod day_01;
mod day_02;

use crate::shared::Solution;

pub const SOLUTIONS: [&dyn Solution; 2] = [&day_01::Day01 {}, &day_02::Day02 {}];
