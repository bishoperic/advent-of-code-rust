mod common;
mod solutions;

use std::env;

fn main() {
    let day = env::args().nth(1).expect("Day not specified!");

    solutions::all.map(|day| {
        println!("{}", day.part_a());
    });
}
