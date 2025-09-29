use crate::shared::Solution;

pub struct Day06 {}

impl Solution for Day06 {
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
    fish_groups: Vec<usize>,
    fish_buffer: Vec<usize>,
}
impl FishPopulation {
    fn new(initial_population: Vec<usize>) -> Self {
        let mut fish_groups = vec![0; 7];
        let fish_buffer = vec![0; 2];

        for fish in initial_population {
            fish_groups[fish] += 1;
        }

        Self {
            fish_groups,
            fish_buffer,
        }
    }
    fn simulate_days(&mut self, days: usize) {
        for _ in 0..days {
            self.simulate_day();
        }
    }
    fn simulate_day(&mut self) {
        let new_fish = self.fish_groups.remove(0);
        self.fish_groups.push(new_fish);
        self.fish_buffer.push(new_fish);

        let last_fish = self.fish_groups.len() - 1;
        let added_fish = self.fish_buffer.remove(0);
        self.fish_groups[last_fish] += added_fish;
    }
    fn total_fish(&self) -> usize {
        self.fish_groups.iter().sum::<usize>() + self.fish_buffer.iter().sum::<usize>()
    }
}
