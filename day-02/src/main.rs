use crate::Part::{Part1, Part2};
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        Ok(Range {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        })
    }
}

impl Range {
    fn sum_invalid(&self) -> usize {
        (self.start..=self.end)
            .filter(|num| {
                let half_digits = num.ilog10().div_ceil(2);
                let mod_val = 10usize.pow(half_digits);
                let lower_half = num % mod_val;
                let upper_half = num / mod_val;
                lower_half == upper_half
            })
            .sum()
    }

    fn sum_multi_invalid(&self) -> usize {
        let mut invalid_sum = 0;

        for num in self.start..=self.end {
            let half_digits = num.ilog10().div_ceil(2);
            for digit_count in 1..=half_digits {
                let mod_val = 10usize.pow(digit_count);
                let last_n_digits = num % mod_val;
                let mut test_num = num / mod_val;
                if last_n_digits == 0 || last_n_digits.ilog(10) + 1 != digit_count {
                    continue;
                }
                let mut found = true;
                while test_num > 0 {
                    found = test_num % mod_val == last_n_digits;
                    if !found {
                        break;
                    }
                    test_num /= mod_val;
                }

                if found {
                    invalid_sum += num;
                    break;
                }
            }
        }

        invalid_sum
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let line: &str = lines[0];

    let ranges: Vec<Range> = line
        .split(',')
        .map(|range| range.parse().unwrap())
        .collect();

    if part == Part1 {
        ranges.iter().map(Range::sum_invalid).sum()
    } else {
        ranges.iter().map(Range::sum_multi_invalid).sum()
    }
}

fn main() {
    println!("Part 1 value: {}", get_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::Part::{Part1, Part2};
    use crate::get_value;

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_value("./test.txt", Part1);
        assert_eq!(value, 1227775554);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 8);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 4174379265);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
