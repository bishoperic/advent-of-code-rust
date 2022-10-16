use std::collections::HashSet;

use crate::common::Solution;

const KERNEL: [Point; 4] = [
    Point { x: 0, y: -1 },
    // (1, -1),
    Point { x: 1, y: 0 },
    // (1, 1),
    Point { x: 0, y: 1 },
    // (-1, 1),
    Point { x: -1, y: 0 },
    // (-1, -1),
];

pub struct Day9 {}

impl Solution for Day9 {
    fn part_a(&self, input: String) -> String {
        let heightmap = input
            .lines()
            .flat_map(|line| {
                let mut temp = Vec::new();
                for num in line.split("") {
                    if let Ok(num) = num.parse::<i32>() {
                        temp.push(num);
                    }
                }
                temp
            })
            .collect::<Vec<_>>();

        let width = input.lines().next().unwrap().len();
        let height = heightmap.len() / width;

        let coords = CoordinatesUtil {
            //            width: width as i32,
            height: height as i32,
        };

        let mut relative_heightmap = Vec::new();

        for (i, value) in heightmap.iter().enumerate() {
            let location = coords.index_to_coords(i as i32);
            let mut num_lower_neighbors = 0;

            for neighbor_dir in KERNEL {
                let neighbor_pos = (location.0 + neighbor_dir.x, location.1 + neighbor_dir.y);
                if neighbor_pos.0 < 0
                    || neighbor_pos.0 > (width - 1) as i32
                    || neighbor_pos.1 < 0
                    || neighbor_pos.1 > (height - 1) as i32
                {
                    continue;
                }

                let neightbor_index = coords.coords_to_index(neighbor_pos);

                if let Some(neightbor_val) = heightmap.get(neightbor_index as usize) {
                    if neightbor_val <= value {
                        num_lower_neighbors += 1;
                    }
                }
            }

            relative_heightmap.push(num_lower_neighbors);
        }

        let mut count = 0;
        for (i, value) in relative_heightmap.iter().enumerate() {
            if *value == 0 {
                count += 1 + heightmap[i];
            }
        }

        count.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let heightmap = input
            .lines()
            .map(|line| {
                let mut temp = Vec::new();
                for num in line.split("") {
                    if let Ok(num) = num.parse::<i32>() {
                        temp.push(num);
                    }
                }
                temp
            })
            .collect::<Vec<_>>();

        let heightmap = Map::new(heightmap);

        let neighbor_height_diff: Vec<Vec<i32>> = heightmap
            .values
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, value)| {
                        let position = Point::new(x as i32, y as i32);

                        let mut num_lower_neighbors = 0;
                        for offset in KERNEL {
                            if let Some(neighbor) = heightmap.get_value(position.translate(offset))
                            {
                                if neighbor <= *value {
                                    num_lower_neighbors += 1;
                                }
                            }
                        }
                        num_lower_neighbors
                    })
                    .collect()
            })
            .collect();

        let mut low_points = Vec::new();
        for (y, row) in neighbor_height_diff.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == 0 {
                    low_points.push(Point::new(x as i32, y as i32));
                }
            }
        }

        let mut sizes = Vec::new();

        for point in low_points {
            let area = heightmap.floodfill(point);
            sizes.push(area.len());
        }

        sizes.sort_unstable();
        sizes.reverse();

        (sizes.get(0).unwrap() * sizes.get(1).unwrap() * sizes.get(2).unwrap()).to_string()
    }
}

struct CoordinatesUtil {
    //    width: i32,
    height: i32,
}
impl CoordinatesUtil {
    fn coords_to_index(&self, coords: (i32, i32)) -> i32 {
        coords.0 + (coords.1 * self.height)
    }
    fn index_to_coords(&self, index: i32) -> (i32, i32) {
        (index % self.height, index / self.height)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn translate(&self, translation: Self) -> Self {
        Self {
            x: self.x + translation.x,
            y: self.y + translation.y,
        }
    }
}

struct Map {
    values: Vec<Vec<i32>>,
    // height: u32,
    // width: u32,
}
impl Map {
    fn new(values: Vec<Vec<i32>>) -> Self {
        //        let row = values.get(0).expect("Couldn't get a row from the map");
        //         let height = values.len() as u32;
        //         let width = row.len() as u32;

        Self {
            values,
            // height,
            // width,
        }
    }
    fn get_value(&self, location: Point) -> Option<i32> {
        // get the row, which is determined by the y-value
        if let Some(row) = self.values.get(location.y as usize) {
            // then the column, which is determined by the x-value
            if let Some(column) = row.get(location.x as usize) {
                Some(*column)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn floodfill(&self, initial_point: Point) -> Vec<Point> {
        let mut inside: HashSet<Point> = HashSet::new();
        let mut queue = Vec::new();

        queue.push(initial_point);

        loop {
            if queue.is_empty() {
                break;
            }

            let current_value = match queue.pop() {
                Some(value) => value,
                None => break,
            };

            for direction in KERNEL {
                let neighbor = current_value.translate(direction);

                match self.get_value(neighbor) {
                    Some(value) => {
                        if inside.contains(&neighbor) {
                            continue;
                        }
                        if value == 9 {
                            continue;
                        }

                        inside.insert(neighbor);
                        queue.push(neighbor);
                    }
                    None => (),
                }
            }
        }

        inside.into_iter().collect()
    }
}
