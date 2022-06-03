use crate::common::Solution;

pub struct Day7 {}

impl Solution for Day7 {
    fn part_a(&self, input: String) -> String {
        let data: Vec<usize> = input
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        let mut fleet = CrabFleet::new(data);
        let position = fleet.old_cheapest_position();

        let fuel_used = fleet.old_fuel_use(position);

        fuel_used.to_string()
    }
    fn part_b(&self, input: String) -> String {
        let data: Vec<usize> = input
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        let mut fleet = CrabFleet::new(data);
        let position = fleet.cheapest_position();

        let fuel_used = fleet.fuel_use(position);

        fuel_used.to_string()
    }
}

struct CrabFleet {
    positions: Vec<usize>,
}
impl CrabFleet {
    fn new(positions: Vec<usize>) -> CrabFleet {
        CrabFleet { positions }
    }
    fn old_cheapest_position(&mut self) -> usize {
        find_median(&mut self.positions)
    }
    fn old_fuel_use(&self, position: usize) -> usize {
        let mut fuel_used = 0;

        for sub in &self.positions {
            fuel_used += (position as isize - *sub as isize).abs() as usize;
        }

        fuel_used
    }
    fn cheapest_position(&mut self) -> usize {
        let mut position_fuel_costs = vec![];
        let min = self.positions.iter().min().unwrap();
        let max = self.positions.iter().max().unwrap();

        for position in *min..=*max {
            position_fuel_costs.push((position, self.fuel_use(position)));
        }

        position_fuel_costs.iter().min_by_key(|x| x.1).unwrap().0
    }
    fn fuel_use(&self, position: usize) -> usize {
        let mut fuel_used = 0;

        let distance_calc = |x1: usize, x2: usize| (x1 as isize - x2 as isize).abs() as usize;
        let fuel_calc = |x: usize| x * (x + 1) / 2;

        for sub in &self.positions {
            fuel_used += fuel_calc(distance_calc(position, *sub));
        }

        fuel_used
    }
}

fn find_median(list: &mut Vec<usize>) -> usize {
    list.sort();
    if list.len() % 2 == 1 {
        list[list.len() / 2]
    } else {
        (list[list.len() / 2] + list[list.len() / 2 - 1]) / 2
    }
}
