use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Part {
    Part1,
    Part2,
}

fn apply_operator(numbers: &[i64], op: char) -> i64 {
    match op {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => panic!("Unknown operator: {op}"),
    }
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '*')
}

fn parse_grid(lines: &[&str]) -> Vec<Vec<char>> {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect()
}

fn read_column(grid: &[Vec<char>], col: usize) -> (String, Option<char>) {
    let mut digits = String::new();
    let mut operator = None;

    for row in grid {
        let c = row[col];
        if is_operator(c) {
            operator = Some(c);
        } else {
            digits.push(c);
        }
    }

    (digits, operator)
}

fn solve_part1(lines: &[&str]) -> i64 {
    let (&operator_line, number_lines) = lines
        .split_last()
        .expect("Input must have at least one line");

    let operators: Vec<char> = operator_line
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    let mut columns: Vec<Vec<i64>> = vec![Vec::new(); operators.len()];

    for line in number_lines {
        for (i, num) in line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .enumerate()
        {
            columns[i].push(num);
        }
    }

    columns
        .iter()
        .zip(&operators)
        .map(|(col, &op)| apply_operator(col, op))
        .sum()
}

fn solve_part2(lines: &[&str]) -> i64 {
    let grid = parse_grid(lines);
    let width = grid.first().map_or(0, |row| row.len());

    let mut answers = Vec::new();
    let mut equation = Vec::new();
    let mut current_op = None;

    for col in 0..width {
        let (digits, operator) = read_column(&grid, col);

        if let Some(op) = operator {
            current_op = Some(op);
        }

        match digits.trim().parse::<i64>() {
            Ok(num) => equation.push(num),
            Err(_) if !equation.is_empty() => {
                // Empty column = end of problem
                if let Some(op) = current_op {
                    answers.push(apply_operator(&equation, op));
                }
                equation.clear();
                current_op = None;
            }
            Err(_) => {}
        }
    }

    if let Some(op) = current_op {
        answers.push(apply_operator(&equation, op));
    }

    answers.iter().sum()
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = file_contents.lines().collect();

    match part {
        Part::Part1 => solve_part1(&lines),
        Part::Part2 => solve_part2(&lines),
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
        assert_eq!(value, 7858808482092);
    }
}
