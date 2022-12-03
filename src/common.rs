use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub trait Solution {
    fn part_a(&self, input: String) -> String;
    fn part_b(&self, input: String) -> String;
}

pub fn get_input(day: u32) -> String {
    let filepath = "data2022/";
    let filepath = format!("{}day_{}.txt", filepath, day);
    let mut file = File::open(&filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("Input file not found, creating now.");
            File::create(&filepath).expect("Error while creating file")
        } else {
            panic!("Error while opening file");
        }
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error while reading file to string");

    contents
}
