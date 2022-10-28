use crate::common::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_a(&self, input: String) -> String {
        let mut sum: [usize; 12] = [0; 12];

        let lines = input.lines();
        let mut length = 0;

        for number in lines {
            length += 1;
            for (i, bit) in number.chars().enumerate() {
                sum[i] += match bit {
                    '1' => 1,
                    '0' => 0,
                    _ => 0,
                }
            }
        }

        let gamma = u32::from_str_radix(
            sum.iter()
                .map(|x| if x < &(length / 2) { '0' } else { '1' })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        let epsilon = u32::from_str_radix(
            sum.iter()
                .map(|x| if x > &(length / 2) { '0' } else { '1' })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        (gamma * epsilon).to_string()
    }
    fn part_b(&self, input: String) -> String {
        let oxygen_generator = filter_data(&input, 12, 0);
        let co2_scrubber = filter_data(&input, 12, 1);

        (oxygen_generator * co2_scrubber).to_string()
    }
}

fn filter_data(input: &String, num_len: usize, mode: u8) -> u32 {
    let mut data = input.clone();

    for i in 0..num_len {
        let mut sum = (0, 0);
        for num in data.lines().collect::<Vec<&str>>() {
            let num = num.chars().nth(i).unwrap();
            if num == '1' {
                sum.0 += 1;
            }

            sum.1 += 1;
        }

        if sum.1 == 1 {
            return u32::from_str_radix(&data.trim(), 2).unwrap();
        }

        let most_common = match mode {
            0 => {
                // budget round-up on int division
                if sum.0 >= (sum.1 + 1) / 2 {
                    '1'
                } else {
                    '0'
                }
            }

            1 => {
                if sum.0 < sum.1 / 2 {
                    '1'
                } else {
                    '0'
                }
            }
            _ => panic!("Incorrect mode specified!"),
        };

        data = data
            .lines()
            .filter(|x| {
                let num = x.chars().nth(i).unwrap();

                num == most_common
            })
            .fold(String::new(), |total: String, current| {
                if total.is_empty() {
                    format!("{}", current)
                } else {
                    format!("{}\n{}", total, current)
                }
            });
    }

    u32::from_str_radix(&data.trim(), 2).unwrap()
}
