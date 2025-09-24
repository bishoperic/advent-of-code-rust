use std::{
    fs::{self},
    io::Read,
    path::PathBuf,
};

use color_eyre::eyre::{Result, eyre};
use reqwest::{Url, blocking::Client, header::COOKIE};

pub trait Solution {
    fn part_a(&self, input: String) -> String;
    fn part_b(&self, input: String) -> String;
}

fn pull_data(year: i32, day: i32) -> String {
    let url = construct_aoc_input_url(year, day);
    let session_token = dotenvy::var("SESSION_TOKEN").unwrap();
    let client = Client::new();
    let request = client
        .get(url)
        .header(COOKIE, format!("session={}", &session_token))
        .build();
    let mut response = client.execute(request.unwrap()).unwrap();
    let mut data = String::new();
    response.read_to_string(&mut data).unwrap();

    fs::create_dir_all(format_year_dir_path(year)).unwrap();
    fs::write(format_filepath(year, day), &data).unwrap();

    data
}

fn load_data(year: i32, day: i32) -> Result<String> {
    let path = construct_input_filepath(year, day);
    fs::read_to_string(path).map_err(|err| eyre!(err))
}

fn format_year_dir_path(year: i32) -> String {
    format!("data/{year}")
}

fn format_filepath(year: i32, day: i32) -> String {
    format!("{}/day_{day:02}.txt", format_year_dir_path(year))
}

fn construct_input_filepath(year: i32, day: i32) -> PathBuf {
    PathBuf::from(format_filepath(year, day))
}

fn construct_aoc_input_url(year: i32, day: i32) -> Url {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    Url::parse(&url).unwrap()
}

pub fn load_or_pull_data(year: i32, day: i32) -> String {
    match load_data(year, day) {
        Ok(data) => data,
        Err(_) => pull_data(year, day),
    }
}

// fn create_code() {}

// fn run_code() {}

// fn run_day(year: i32, day: i32) {
//     let data = load_or_pull_data(year, day);
//     run_code(data);
// }
