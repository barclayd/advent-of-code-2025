use std::fs;
use crate::Part::{Part1, Part2};

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

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = file_contents.lines().collect::<Vec<&str>>();

    let (operator_line, number_lines) = lines.split_last().expect("Input must have at least one line");

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
    } else { 4 }
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
        assert_eq!(value, 4277556);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 8);
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