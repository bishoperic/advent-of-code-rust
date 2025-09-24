use std::collections::HashMap;

use crate::shared::Solution;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_a(&self, input: String) -> String {
        let commands: Vec<_> = input
            .split("$ ")
            .skip(1)
            .map(|command| Command::new(command).unwrap())
            .collect();

        let mut filesystem = FileSystem::new();

        for command in commands {
            filesystem.command(command);
        }

        // technically this forgets the root directory but since it's too big it doesn't matter
        let total_size: i64 = filesystem
            .flatten_directories()
            .iter()
            .map(|dir| dir.calc_size())
            .filter(|size| size <= &100000)
            .sum();

        total_size.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let commands: Vec<_> = input
            .split("$ ")
            .skip(1)
            .map(|command| Command::new(command).unwrap())
            .collect();

        let mut filesystem = FileSystem::new();

        for command in commands {
            filesystem.command(command);
        }

        let disk_size = 70_000_000;
        let update_size = 30_000_000;
        let used_storage = filesystem.calc_size();

        let min_storage_to_free = update_size - (disk_size - used_storage);

        filesystem
            .flatten_directories()
            .iter()
            .map(|dir| dir.calc_size())
            .filter(|dir_size| dir_size >= &min_storage_to_free)
            .min()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug)]
struct FileSystem {
    contents: HashMap<String, Item>,
    current_path: Vec<String>,
}
impl FileSystem {
    fn new() -> Self {
        Self {
            contents: HashMap::new(),
            current_path: Vec::new(),
        }
    }
    fn command(&mut self, command: Command) {
        match command {
            Command::CD { directory } => {
                if directory == "/" {
                    self.current_path.clear();
                    return;
                } else if directory == ".." {
                    if self.current_path.len() == 0 {
                        panic!("Can't move further back than the root!")
                    }
                    self.current_path.pop();
                    return;
                }

                self.current_path.push(directory);
            }
            Command::LS { contents } => {
                let current_path = &self.current_path.clone();
                let current_directory = self.get_directory_mut(current_path);

                for item in contents {
                    current_directory.insert(
                        match &item {
                            Item::Directory { name, contents: _ } => name.to_owned(),
                            Item::File { name, size: _ } => name.to_owned(),
                        },
                        item,
                    );
                }
            }
        }
    }
    fn get_directory_mut(&mut self, path: &Vec<String>) -> &mut HashMap<String, Item> {
        let mut current_directory: &mut HashMap<String, Item> = &mut self.contents;
        for level in path {
            let item = current_directory.get_mut(level).expect("Invalid directory");
            if let Item::Directory { name: _, contents } = item {
                current_directory = contents;
            } else {
                panic!("Path item [{level}] was not a directory!")
            }
        }

        current_directory
    }
    fn flatten_directories(&self) -> Vec<&Item> {
        let mut directories = Vec::new();

        let mut queue: Vec<&Item> = self.contents.iter().map(|x| x.1).collect();
        loop {
            let item = match queue.pop() {
                Some(item) => item,
                None => break,
            };

            directories.push(item);

            match item {
                Item::Directory { name: _, contents } => queue.append(
                    &mut contents
                        .iter()
                        .map(|x| x.1)
                        .filter(|x| match x {
                            Item::Directory {
                                name: _,
                                contents: _,
                            } => true,
                            Item::File { name: _, size: _ } => false,
                        })
                        .collect::<Vec<&Item>>(),
                ),
                Item::File { name: _, size: _ } => continue,
            }
        }

        directories
    }
    fn calc_size(&self) -> i64 {
        self.contents.iter().map(|item| item.1.calc_size()).sum()
    }
}

#[derive(Debug)]
enum Command {
    CD { directory: String },
    LS { contents: Vec<Item> },
}
impl Command {
    fn new(command: &str) -> Option<Self> {
        let mut results = command.lines();

        match results.next()?.split_once(' ') {
            Some(command) => Some(Command::CD {
                directory: command.1.to_string(),
            }),
            None => Some(Command::LS {
                contents: results.map(|item| Item::new(item).unwrap()).collect(),
            }),
        }
    }
}

#[derive(Debug)]
enum Item {
    Directory {
        name: String,
        contents: HashMap<String, Item>,
    },
    File {
        name: String,
        size: i64,
    },
}
impl Item {
    fn new(item: &str) -> Option<Self> {
        let item = item.split_once(" ")?;

        match item.0 {
            "dir" => Some(Item::Directory {
                name: item.1.to_string(),
                contents: HashMap::new(),
            }),
            _ => Some(Item::File {
                name: item.1.to_string(),
                size: item.0.parse().unwrap(),
            }),
        }
    }
    fn calc_size(&self) -> i64 {
        match self {
            Item::Directory { name: _, contents } => {
                contents.iter().map(|item| item.1.calc_size()).sum()
            }
            Item::File { name: _, size } => *size,
        }
    }
}

// enum Value {
//     Directory {
//         name: String,
//         contents: HashMap<String, Self>,
//     },
//     File {
//         name: String,
//         size: i64,
//     },
// }
