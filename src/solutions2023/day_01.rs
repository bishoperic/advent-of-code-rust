use crate::common::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_a(&self, input: String) -> String {
        let output: u32 = input
            .lines()
            .map(|line| CalibrationValue::new(line.to_owned()))
            .map(|value| value.get_first_and_last_digits().unwrap())
            .sum();

        output.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let output: u32 = input
            .lines()
            .map(|line| CalibrationValue::new(line.to_owned()))
            .map(|value| value.convert_spelled_to_digits())
            .map(|value| value.get_first_and_last_digits().unwrap())
            .sum();

        output.to_string()
    }
}

struct CalibrationValue {
    value: String,
}

impl CalibrationValue {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn get_first_and_last_digits(&self) -> Option<u32> {
        let digits_only: Vec<_> = self
            .value
            .chars()
            .filter(|char| char.is_numeric())
            .collect();

        if digits_only.len() == 0 {
            return None;
        }

        let first = digits_only.get(0).and_then(|digit| digit.to_digit(10));
        let last = digits_only
            .get(digits_only.len() - 1)
            .and_then(|digit| digit.to_digit(10));

        let (Some(first), Some(last)) = (first, last) else {
            return None;
        };

        let result = first * 10 + last;

        Some(result)
    }

    fn convert_spelled_to_digits(self) -> Self {
        const SPELLED: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut result = String::new();

        let mut iter = self.value.chars().enumerate();
        while let Some((i, char)) = iter.next() {
            for (spell_index, spelling) in SPELLED.iter().enumerate() {
                let Some(lookahead) = self.value.get(i..i + spelling.len()) else {
                    continue;
                };

                if lookahead == *spelling {
                    let digit = spell_index + 1;
                    result.push_str(&digit.to_string());
                }
            }

            result.push(char);
        }

        Self { value: result }
    }
}

#[cfg(test)]
mod test {
    use super::CalibrationValue;

    #[test]
    fn should_extract_digits() {
        let input = String::from("12452139");
        let value = CalibrationValue::new(input);
        let digits = value.get_first_and_last_digits();

        assert_eq!(digits, Some(19))
    }

    #[test]
    fn should_extract_digits_with_characters() {
        let input = String::from("asdf124adsf52asdf1asdf39asdf");
        let value = CalibrationValue::new(input);
        let digits = value.get_first_and_last_digits();

        assert_eq!(digits, Some(19))
    }

    #[test]
    fn should_use_same_digit_if_only_one() {
        let input = String::from("asd4asdf");
        let value = CalibrationValue::new(input);
        let digits = value.get_first_and_last_digits();

        assert_eq!(digits, Some(44))
    }

    #[test]
    fn should_return_none_if_no_digits() {
        let input = String::from("asdasdf");
        let value = CalibrationValue::new(input);
        let digits = value.get_first_and_last_digits();

        assert_eq!(digits, None)
    }

    #[test]
    fn should_handle_overlapping_spellings() {
        let input = String::from("eightwo");
        let value = CalibrationValue::new(input);
        let value = value.convert_spelled_to_digits();
        let value = value.get_first_and_last_digits().unwrap();

        assert_eq!(value, 82)
    }

    #[test]
    fn should_not_skip_over_spelling_characters() {
        let input = String::from("eightwo");
        let value = CalibrationValue::new(input);
        let value = value.convert_spelled_to_digits();

        assert_eq!(value.value, "8eigh2two")
    }

    #[test]
    fn should_follow_aoc_example_part_one() {
        let input = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        let value: u32 = input
            .lines()
            .map(|value| CalibrationValue::new(value.to_owned()))
            .map(|value| value.get_first_and_last_digits().unwrap())
            .sum();

        assert_eq!(value, 142)
    }

    #[test]
    fn should_follow_aoc_example_part_two() {
        let input = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        let value: u32 = input
            .lines()
            .map(|value| CalibrationValue::new(value.to_owned()))
            .map(|value| value.convert_spelled_to_digits())
            .map(|value| value.get_first_and_last_digits().unwrap())
            .sum();

        assert_eq!(value, 281)
    }
}
