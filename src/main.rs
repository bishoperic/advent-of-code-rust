mod shared;
mod solutions;
mod solutions2022;
mod solutions2023;

use std::env;

use color_eyre::{Result, eyre::eyre};

use crate::shared::load_or_pull_data;

fn main() -> Result<()> {
    let mut args = env::args();
    let _ = args.next();

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
        })
        .map_err(|err| eyre!(err))?;

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
        })
        .map_err(|err| eyre!(err))?;

    let data = load_or_pull_data(year, day);
    println!("{}", data);

    Ok(())
}
