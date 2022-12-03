mod common;
mod solutions;
mod solutions2022;

use std::env;

fn main() {
    let day = env::args().nth(1).expect("Day not specified!");
    let part = env::args().nth(2).expect("Part not specified!");

    let day: usize = day.parse().expect("Day was not a number!");

    let solution = solutions2022::ALL
        .iter()
        .nth(day - 1)
        .expect("Couldn't find specified day!");

    let input = common::get_input(day as u32);

    println!(
        "Solution for Day {}, Part {}:\n\n{}\n",
        day,
        part.to_uppercase(),
        match part.as_str() {
            "a" => solution.part_a(input),
            "b" => solution.part_b(input),
            _ => panic!("Part was not correctly specified! (a | b)"),
        }
    );
}
