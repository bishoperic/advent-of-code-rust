mod consts;
mod shared;
mod year_2021;
mod year_2022;
mod year_2023;
mod year_2024;

use std::env;

use color_eyre::{Result, eyre::eyre};

use crate::{
    consts::{DAY_RANGE, YEAR_RANGE},
    shared::load_or_pull_data,
};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let year = args
        .next()
        .ok_or("Missing argument for which year to run")
        .and_then(|day| day.parse::<i32>().map_err(|_| "Year was not a number"))
        .and_then(|n| {
            if YEAR_RANGE.contains(&n) {
                Ok(n)
            } else {
                Err("Year must be between 2015 and 2024")
            }
        })
        .map_err(|err| eyre!(err))?;

    let day = args
        .next()
        .ok_or("Missing argument for which day to run")
        .and_then(|day| day.parse::<i32>().map_err(|_| "Day was not a number"))
        .and_then(|n| {
            if DAY_RANGE.contains(&n) {
                Ok(n)
            } else {
                Err("Day must be between 1 and 25")
            }
        })
        .map_err(|err| eyre!(err))?;

    let data = load_or_pull_data(year, day);

    let solutions = match year {
        2021 => year_2021::SOLUTIONS.as_slice(),
        2022 => year_2022::SOLUTIONS.as_slice(),
        2023 => year_2023::SOLUTIONS.as_slice(),
        2024 => year_2024::SOLUTIONS.as_slice(),
        _ => &[],
    };

    let solution = solutions
        .get(day as usize - 1)
        .ok_or(eyre!("That day doesn't have a solution"))?;

    let part_a = solution.part_a(data.clone());

    println!("Solution for Part A:");
    println!("{part_a}");

    let part_b = solution.part_b(data);

    println!("Solution for Part B:");
    println!("{part_b}");

    Ok(())
}
