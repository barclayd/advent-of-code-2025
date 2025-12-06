use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn apply_operator(numbers: &[i64], op: char) -> i64 {
    match op {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => panic!("Unknown operator: {}", op),
    }
}

fn build_column_numbers(number_strings: &[&str], right_align: bool) -> Vec<i64> {
    let chars: Vec<Vec<char>> = number_strings
        .iter()
        .map(|s| {
            if right_align {
                s.chars().rev().collect()
            } else {
                s.chars().collect()
            }
        })
        .collect();

    let max_len = chars.iter().map(|v| v.len()).max().unwrap_or(0);

    let positions: Box<dyn Iterator<Item = usize>> = if right_align {
        Box::new((0..max_len).rev())
    } else {
        Box::new(0..max_len)
    };

    positions
        .map(|pos| {
            let digits: String = chars.iter().filter_map(|c| c.get(pos).copied()).collect();
            digits.parse().unwrap_or(0)
        })
        .collect()
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = file_contents.lines().collect::<Vec<&str>>();

    let (operator_line, number_lines) = lines
        .split_last()
        .expect("Input must have at least one line");

    let operators: Vec<char> = operator_line
        .split_whitespace()
        .map(|s| s.chars().next().expect("Operator expected"))
        .collect();

    let columns: Vec<Vec<i64>> = {
        let mut numbers: Vec<Vec<i64>> = vec![Vec::new(); operators.len()];

        for line in number_lines {
            for (i, num_str) in line.split_whitespace().enumerate() {
                if let Ok(num) = num_str.parse::<i64>() {
                    numbers[i].push(num);
                }
            }
        }
        numbers
    };

    if part == Part1 {
        columns
            .iter()
            .zip(operators.iter())
            .map(|(col, &op)| apply_operator(col, op))
            .sum()
    } else {
        let groups: Vec<Vec<&str>> = {
            let mut groups: Vec<Vec<&str>> = vec![Vec::new(); operators.len()];

            for line in number_lines {
                for (i, num_str) in line.split_whitespace().enumerate() {
                    groups[i].push(num_str);
                }
            }
            groups
        };

        groups
            .iter()
            .zip(operators.iter())
            .map(|(group, &op)| {
                let column_numbers = build_column_numbers(group, op == '*');
                apply_operator(&column_numbers, op)
            })
            .sum()
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
        assert_eq!(value, 4277556);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 4412382293768);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 3263827);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
