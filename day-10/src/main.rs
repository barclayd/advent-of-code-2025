use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone)]
struct Indicator {
    desired_state: bool,
    current_state: bool,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    ParseInt,
}
#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<Indicator>,
    buttons: Vec<Vec<i32>>,
    #[allow(dead_code)]
    joltages: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Factory {
    machines: Vec<Machine>,
}

fn parse_bool(c: char) -> Result<Indicator, ParseError> {
    match c {
        '#' => Ok(Indicator {
            desired_state: true,
            current_state: false,
        }),
        '.' => Ok(Indicator {
            desired_state: false,
            current_state: false,
        }),
        _ => panic!("Unexpected char {}", c),
    }
}

impl Machine {
    fn parse(line: &str) -> Result<Self, ParseError> {
        let line = line.trim();

        let indicator_start = line.find('[').ok_or(ParseError::InvalidFormat)?;
        let indicator_end = line.find(']').ok_or(ParseError::InvalidFormat)?;

        let indicator_str = &line[indicator_start + 1..indicator_end];
        let indicators: Vec<Indicator> = indicator_str
            .chars()
            .map(parse_bool)
            .collect::<Result<_, _>>()?;

        let rest = &line[indicator_end + 1..];

        let joltage_start = rest.find('{').ok_or(ParseError::InvalidFormat)?;
        let joltage_end = rest.find('}').ok_or(ParseError::InvalidFormat)?;

        let joltage_str = &rest[joltage_start + 1..joltage_end];
        let joltages: Vec<i32> = joltage_str
            .split(',')
            .map(|s| s.trim().parse::<i32>().map_err(|_| ParseError::ParseInt))
            .collect::<Result<_, _>>()?;

        let buttons_str = &rest[..joltage_start];
        let buttons: Vec<Vec<i32>> = buttons_str
            .split(')')
            .filter_map(|s| {
                let s = s.trim();
                s.find('(').map(|start| &s[start + 1..])
            })
            .map(|s| {
                s.split(',')
                    .map(|n| n.trim().parse::<i32>().map_err(|_| ParseError::ParseInt))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Machine {
            indicators,
            buttons,
            joltages,
        })
    }

    fn min_presses(&self) -> Option<usize> {
        let num_buttons = self.buttons.len();

        if num_buttons == 0 {
            return if self
                .indicators
                .iter()
                .all(|i| i.current_state == i.desired_state)
            {
                Some(0)
            } else {
                None
            };
        }

        let mut matrix = self.build_augmented_matrix();
        let pivot_cols = gaussian_eliminate(&mut matrix, num_buttons);

        if !is_consistent(&matrix, &pivot_cols, num_buttons) {
            return None;
        }

        let free_cols = find_free_columns(num_buttons, &pivot_cols);

        Some(find_minimum_weight_solution(
            &matrix,
            &pivot_cols,
            &free_cols,
            num_buttons,
        ))
    }

    fn build_augmented_matrix(&self) -> Vec<Vec<bool>> {
        let num_indicators = self.indicators.len();
        let num_buttons = self.buttons.len();

        let mut matrix = vec![vec![false; num_buttons + 1]; num_indicators];

        for (btn_idx, button) in self.buttons.iter().enumerate() {
            for &indicator_idx in button {
                let idx = indicator_idx as usize;
                if idx < num_indicators {
                    matrix[idx][btn_idx] = true;
                }
            }
        }

        for (i, indicator) in self.indicators.iter().enumerate() {
            matrix[i][num_buttons] = indicator.desired_state ^ indicator.current_state;
        }

        matrix
    }
}

fn gaussian_eliminate(matrix: &mut [Vec<bool>], num_buttons: usize) -> Vec<usize> {
    let num_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..num_buttons {
        let pivot_found = (pivot_row..num_rows).find(|&row| matrix[row][col]);

        if let Some(found) = pivot_found {
            matrix.swap(pivot_row, found);

            for row in 0..num_rows {
                if row != pivot_row && matrix[row][col] {
                    xor_rows(matrix, row, pivot_row);
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    pivot_cols
}

fn xor_rows(matrix: &mut [Vec<bool>], target: usize, source: usize) {
    let num_cols = matrix[target].len();
    for col in 0..num_cols {
        matrix[target][col] ^= matrix[source][col];
    }
}

fn is_consistent(matrix: &[Vec<bool>], pivot_cols: &[usize], num_buttons: usize) -> bool {
    let num_pivots = pivot_cols.len();

    for row in matrix.iter().skip(num_pivots) {
        if row[num_buttons] {
            return false;
        }
    }

    true
}

fn find_free_columns(num_buttons: usize, pivot_cols: &[usize]) -> Vec<usize> {
    (0..num_buttons)
        .filter(|c| !pivot_cols.contains(c))
        .collect()
}

/// Find the solution with minimum Hamming weight (fewest button presses).
fn find_minimum_weight_solution(
    matrix: &[Vec<bool>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    num_buttons: usize,
) -> usize {
    let num_free = free_cols.len();

    if num_free > 25 {
        eprintln!(
            "Warning: {} free variables, enumeration may be slow",
            num_free
        );
    }

    let mut min_presses = usize::MAX;

    for free_assignment in 0u64..(1u64 << num_free) {
        let solution =
            compute_solution(matrix, pivot_cols, free_cols, free_assignment, num_buttons);

        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);

        if min_presses <= 1 {
            break;
        }
    }

    min_presses
}

fn compute_solution(
    matrix: &[Vec<bool>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    free_assignment: u64,
    num_buttons: usize,
) -> Vec<bool> {
    let mut solution = vec![false; num_buttons];

    for (i, &col) in free_cols.iter().enumerate() {
        solution[col] = (free_assignment >> i) & 1 == 1;
    }

    for (row_idx, &pivot_col) in pivot_cols.iter().enumerate().rev() {
        let mut val = matrix[row_idx][num_buttons];

        for col in (pivot_col + 1)..num_buttons {
            if matrix[row_idx][col] && solution[col] {
                val ^= true;
            }
        }

        solution[pivot_col] = val;
    }

    solution
}

impl Factory {
    fn new(input: &str) -> Result<Self, ParseError> {
        let machines: Vec<Machine> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Machine::parse)
            .collect::<Result<_, _>>()?;

        Ok(Factory { machines })
    }

    fn total_min_presses(&self) -> Option<i64> {
        self.machines
            .iter()
            .map(|m| m.min_presses().map(|x| x as i64))
            .sum()
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let factory = Factory::new(&file_contents).expect("Failed to parse input");

    if part == Part1 {
        factory.total_min_presses().unwrap_or_else(|| 0)
    } else {
        4
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
        assert_eq!(value, 7);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 524);
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
