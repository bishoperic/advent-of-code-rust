use crate::common::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn part_a(&self, input: String) -> String {
        let data: Vec<_> = input
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        let mut fishies = FishPopulation::new(data);

        fishies.simulate_days(80);

        fishies.total_fish().to_string()
    }
    fn part_b(&self, input: String) -> String {
        let data: Vec<_> = input
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        let mut fishies = FishPopulation::new(data);

        fishies.simulate_days(256);

        fishies.total_fish().to_string()
    }
}

struct FishPopulation {
    fishes: Vec<usize>,
}
impl FishPopulation {
    fn new(initial_population: Vec<usize>) -> Self {
        Self {
            fishes: initial_population,
        }
    }
    fn simulate_days(&mut self, days: usize) {
        for _ in 0..days {
            self.simulate_day();
        }
    }
    fn simulate_day(&mut self) {
        let mut reproduced = 0;

        self.fishes = self
            .fishes
            .iter()
            .map(|fish| {
                if *fish == 0 {
                    reproduced += 1;
                    6
                } else {
                    fish - 1
                }
            })
            .collect();

        for _ in 0..reproduced {
            self.fishes.push(8);
        }
    }
    fn total_fish(&self) -> usize {
        self.fishes.len()
    }
}
