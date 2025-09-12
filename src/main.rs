mod common;
mod solutions;
mod solutions2022;
mod solutions2023;

use std::env;

fn main() {
    let mut args = env::args();
    let executable_name = args.next();

    let year = args
        .next()
        .ok_or("Missing argument for which year to run")
        .and_then(|day| day.parse::<i32>().map_err(|_| "Year was not a number"))
        .and_then(|n| {
            if (2015..=2024).contains(&n) {
                Ok(n)
            } else {
                Err("Year must be between 2015 and 2024")
            }
        });

    let day = args
        .next()
        .ok_or("Missing argument for which day to run")
        .and_then(|day| day.parse::<i32>().map_err(|_| "Day was not a number"))
        .and_then(|n| {
            if (1..=25).contains(&n) {
                Ok(n)
            } else {
                Err("Day must be between 1 and 25")
            }
        });

    match (year, day) {
        (Ok(year), Ok(day)) => {}
        _ => {
            let errors = [year.err(), day.err()].into_iter().filter_map(|r| r);

            eprintln!(
                "Usage: {} <year> <day>",
                executable_name.unwrap_or("aoc".to_string())
            );

            for error in errors {
                eprintln!("Error: {}", error);
            }
        }
    }
}
