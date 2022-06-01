use crate::common::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn part_a(&self, input: String) -> String {
        let data = input
            .lines()
            .map(|x| {
                let coords_set = x
                    .split(" -> ")
                    .map(|y| {
                        let coords = y
                            .split(",")
                            .map(|z| z.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        (coords[0], coords[1])
                    })
                    .collect::<Vec<_>>();

                (coords_set[0], coords_set[1])
            })
            .collect::<Vec<_>>();

        let mut map = Map::new(1000);

        for line in data {
            let straight_line = map.straight_line(
                line.0 .0 as isize,
                line.0 .1 as isize,
                line.1 .0 as isize,
                line.1 .1 as isize,
            );

            match straight_line {
                Some(x) => {
                    for point in x {
                        map.add_to_spot(point.0 as usize, point.1 as usize);
                    }
                }
                None => (),
            }
        }

        let mut sum = 0;
        for row in map.data {
            for num in row {
                if num > 1 {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let data = input
            .lines()
            .map(|x| {
                let coords_set = x
                    .split(" -> ")
                    .map(|y| {
                        let coords = y
                            .split(",")
                            .map(|z| z.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        (coords[0], coords[1])
                    })
                    .collect::<Vec<_>>();

                (coords_set[0], coords_set[1])
            })
            .collect::<Vec<_>>();

        let mut map = Map::new(1000);

        for line in data {
            map.line(
                line.0 .0 as isize,
                line.0 .1 as isize,
                line.1 .0 as isize,
                line.1 .1 as isize,
            );
        }

        let mut sum = 0;
        for row in map.data {
            for num in row {
                if num > 1 {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }
}
struct Map {
    data: Vec<Vec<usize>>,
    size: usize,
}

impl Map {
    fn new(size: usize) -> Map {
        Map {
            data: vec![vec![0; size]; size],
            size,
        }
    }
    fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        if let Some(line) = self.straight_line(x1, y1, x2, y2) {
            for point in line {
                self.add_to_spot(point.0 as usize, point.1 as usize);
            }
        } else if let Some(line) = self.diagonal_line(x1, y1, x2, y2) {
            for point in line {
                self.add_to_spot(point.0 as usize, point.1 as usize);
            }
        }
    }
    fn straight_line(
        &mut self,
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    ) -> Option<Vec<(isize, isize)>> {
        let mut points = vec![];
        if x1 == x2 {
            let x = x1;
            let range = if y1 <= y2 { y1..y2 + 1 } else { y2..y1 + 1 };

            for y in range {
                points.push((x, y));
            }
        } else if y1 == y2 {
            let y = y1;
            let range = if x1 <= x2 { x1..x2 + 1 } else { x2..x1 + 1 };

            for x in range {
                points.push((x, y));
            }
        }

        if !points.is_empty() {
            Some(points)
        } else {
            None
        }
    }
    fn diagonal_line(
        &mut self,
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    ) -> Option<Vec<(isize, isize)>> {
        let mut points = vec![];
        if (x1 - x2).abs() == (y1 - y2).abs() {
            let mut x = x1;
            let x_sign = -1 * (x1 - x2).signum();
            let mut y = y1;
            let y_sign = -1 * (y1 - y2).signum();

            loop {
                points.push((x, y));

                if x == x2 && y == y2 {
                    break;
                } else {
                    x += x_sign;
                    y += y_sign;
                }
            }
        }

        if !points.is_empty() {
            Some(points)
        } else {
            None
        }
    }
    fn add_to_spot(&mut self, x: usize, y: usize) {
        for input in [x, y] {
            if input > self.size {
                panic!("Coordinate out of bounds!");
            }
        }

        self.data[y][x] += 1;
    }
}
