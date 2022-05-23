use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

pub trait Solution {
    fn part_a(&self) -> String;
    fn part_b(&self) -> String;
}

pub fn get_input(day: u32) -> String {
    let filepath = "data/";
    let filepath = format!("{}day_{}.txt", filepath, day);
    let mut file = File::open(&filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let req =
                reqwest::blocking::get(format!("https://adventofcode.com/2021/day/{}/input", day))
                    .expect("Error while fetching puzzle input");

            let mut file = File::create(&filepath).expect("Error while creating file");

            file.write_all(
                req.text()
                    .expect("Error reading fetched puzzle response")
                    .as_bytes(),
            )
            .expect("Error while writing to file");

            file
        } else {
            panic!("Error while opening file");
        }
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error while reading file to string");

    contents
}
