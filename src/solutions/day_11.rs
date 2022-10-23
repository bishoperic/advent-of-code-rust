use crate::common::Solution;
//use colored::Colorize;

pub struct Day11 {}

impl Solution for Day11 {
    fn part_a(&self, input: String) -> String {
        let map: Vec<Vec<i32>> = input
            .lines()
            .map(|row| {
                row.split("")
                    .filter(|x| *x != "")
                    .map(|octopus| octopus.parse::<i32>().expect("Input was not a number"))
                    .collect()
            })
            .collect();

        let mut map = OctopusGrid::new(map);

        //        for i in 0..100 {
        //            println!("***** Step {}: *****", i + 1);
        //            map.step(1);
        //            for row in &map.grid {
        //                for x in row {
        //                    let x = if *x == 0 {
        //                        x.to_string().red()
        //                    } else {
        //                        x.to_string().bold()
        //                    };
        //                    print!("{}", x);
        //                }
        //                println!("");
        //            }
        //            println!("");
        //        }

        map.step(100);

        map.flashes.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let map: Vec<Vec<i32>> = input
            .lines()
            .map(|row| {
                row.split("")
                    .filter(|x| *x != "")
                    .map(|octopus| octopus.parse::<i32>().expect("Input was not a number"))
                    .collect()
            })
            .collect();

        let mut map = OctopusGrid::new(map);

        const MAX_FLASHES: i32 = 10 * 10;

        let mut prev_flashes = 0;
        let mut step = 0;

        loop {
            map.step(1);
            step += 1;

            if map.flashes - prev_flashes == MAX_FLASHES as u32 {
                break;
            }

            prev_flashes = map.flashes;
        }

        step.to_string()
    }
}

const FLASH_ENERGY: i32 = 10;

#[derive(Debug)]
struct OctopusGrid {
    grid: Vec<Vec<i32>>,
    flashes: u32,
}
impl OctopusGrid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { grid, flashes: 0 }
    }
    fn step(&mut self, num_steps: u32) {
        for _step in 0..num_steps {
            let mut flashing_octopuses: Vec<Point> = Vec::new();

            // loop over entire grid, increasing energy levels, and then adding flashed neighbors to a list
            for (y, row) in self.grid.iter_mut().enumerate() {
                for (x, octopus) in row.iter_mut().enumerate() {
                    *octopus += 1;

                    if *octopus == FLASH_ENERGY {
                        flashing_octopuses.push(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                        self.flashes += 1;
                    }
                }
            }

            for octopus in flashing_octopuses {
                self.flash(octopus);
            }

            // reset all octopuses for next step
            for row in &mut self.grid {
                for octopus in row {
                    if *octopus >= FLASH_ENERGY {
                        *octopus = 0;
                    }
                }
            }
        }

        // end of fn
    }

    fn flash(&mut self, position: Point) {
        let mut flashed_neighbors = position.get_neighbors();

        loop {
            // remove current octopus from flashed neighbors, keeping its value
            // add 1 to the octopus, check if its equal to 9
            // if it is then add its neighbors to flashed_neighbors
            let current_octopus_pos = match flashed_neighbors.pop() {
                Some(value) => value,
                None => break,
            };

            let current_octopus = match self.get_mut_value(current_octopus_pos) {
                Some(value) => value,
                None => continue,
            };

            *current_octopus += 1;

            if *current_octopus != FLASH_ENERGY {
                continue;
            }

            flashed_neighbors.append(&mut current_octopus_pos.get_neighbors());
            self.flashes += 1;
        }
    }
}
impl Grid for OctopusGrid {
    fn get_value(&self, location: Point) -> Option<i32> {
        // don't mix up the x & y values for rows and columns
        if let Some(row) = self.grid.get(location.y as usize) {
            if let Some(column) = row.get(location.x as usize) {
                Some(*column)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut_value(&mut self, location: Point) -> Option<&mut i32> {
        match self
            .grid
            .get_mut(location.y as usize)
            .map(|row| row.get_mut(location.x as usize))
        {
            Some(value) => value,
            None => None,
        }
    }
}

// SECTION: Points and Maps

const KERNEL: [Point; 8] = [
    // Adjacent: from top clockwise
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    // Diagonal: from top-right clockwise
    Point { x: 1, y: -1 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: -1 },
];

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn translate(&self, translation: Self) -> Self {
        Self {
            x: self.x + translation.x,
            y: self.y + translation.y,
        }
    }
    fn get_neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for neighbor in KERNEL {
            neighbors.push(self.translate(neighbor));
        }

        neighbors
    }
}

trait Grid {
    fn get_value(&self, location: Point) -> Option<i32>;
    fn get_mut_value(&mut self, location: Point) -> Option<&mut i32>;
}
