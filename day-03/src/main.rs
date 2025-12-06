use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn two_largest_sequential(nums: &[i64]) -> i64 {
    let mut max_after = i64::MIN;
    let mut best = i64::MIN;

    for i in (0..nums.len() - 1).rev() {
        max_after = max_after.max(nums[i + 1]);
        let combined = nums[i] * 10 + max_after;
        best = best.max(combined);
    }

    best
}

fn largest_k_sequential(nums: &[i64], k: usize) -> i64 {
    let n = nums.len();
    if n < k {
        return 0;
    }

    let mut result: i64 = 0;
    let mut start: usize = 0;

    for remaining in (1..=k).rev() {
        let end = n - remaining;

        let mut max_val = i64::MIN;
        let mut max_idx = start;
        for i in start..=end {
            if nums[i] > max_val {
                max_val = nums[i];
                max_idx = i;
            }
        }

        result = result * 10 + max_val;
        start = max_idx + 1;
    }

    result
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let sum: i32 = file_contents
    //     .lines()
    //     .map(|line| {
    //         let digits: Vec<i32> = line.chars()
    //             .map(|c| c.to_digit(10).unwrap() as i32)
    //             .collect();
    //         return two_largest_sequential(&digits);
    //     })
    //     .sum();

    return if part == Part1 {
        file_contents
            .lines()
            .map(|line| {
                let digits: Vec<i64> = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect();
                return two_largest_sequential(&digits);
            })
            .sum()
    } else {
        file_contents
            .lines()
            .map(|line| {
                let digits: Vec<i64> = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect();
                return largest_k_sequential(&digits, 12);
            })
            .sum()
    };
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
        assert_eq!(value, 3121910778619);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
