use std::fs;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn two_largest_sequential(nums: &[i32]) -> i32 {
    let mut max_after = i32::MIN;
    let mut best = i32::MIN;

    for i in (0..nums.len() - 1).rev() {
        max_after = max_after.max(nums[i + 1]);
        let combined = nums[i] * 10 + max_after;
        best = best.max(combined);
    }

    best
}

fn get_value(file_path: &str, part: Part) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sum: i32 = file_contents
        .lines()
        .map(|line| {
            let digits: Vec<i32> = line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();
            return two_largest_sequential(&digits);
        })
        .sum();

   if part == Part1 { sum } else { 4 }
}

fn main() {
    println!("Part 1 value: {}", get_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_value;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_value("./test.txt", Part1);
        assert_eq!(value, 357);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 17427);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}