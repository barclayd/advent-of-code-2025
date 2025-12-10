use crate::Part::{Part1, Part2};
use std::fs;
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

// create a Factor struct
// that has a vec of Machines
// Machine struct: [ indicator: vec<Boolean>, button: vec[vec[i32]], joltage: vec[i32] ]
// indicator ligths: # => on, . => off
// to start a machine: indicator lights must match the diagram
// its indicator lights are all initially off

// [.##.]
// 4 indicator lights
// initially off
// goal: first light off, 2nd + 3rd on, 4th off

// lights can be toggled by pushing any of the listed buttons
// button 0 toggles the first light

// fewest total presses

#[derive(Debug, Clone)]
struct Indicator {
    desired_state: bool,
    current_state: bool,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat(String),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseInt(e)
    }
}
#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<Indicator>,
    buttons: Vec<Vec<i32>>,
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
    /// Parse a single line into a Machine
    /// Format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    fn parse(line: &str) -> Result<Self, ParseError> {
        let line = line.trim();

        // Parse indicators: [.##.]
        let indicator_start = line
            .find('[')
            .ok_or_else(|| ParseError::InvalidFormat("Missing '[' for indicator".into()))?;
        let indicator_end = line
            .find(']')
            .ok_or_else(|| ParseError::InvalidFormat("Missing ']' for indicator".into()))?;

        let indicator_str = &line[indicator_start + 1..indicator_end];
        let indicators: Vec<Indicator> = indicator_str
            .chars()
            .map(parse_bool)
            .collect::<Result<_, _>>()?;

        let rest = &line[indicator_end + 1..];

        // Parse joltages: {3,5,4,7}
        let joltage_start = rest
            .find('{')
            .ok_or_else(|| ParseError::InvalidFormat("Missing '{' for joltage".into()))?;
        let joltage_end = rest
            .find('}')
            .ok_or_else(|| ParseError::InvalidFormat("Missing '}' for joltage".into()))?;

        let joltage_str = &rest[joltage_start + 1..joltage_end];
        let joltages: Vec<i32> = joltage_str
            .split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<_, _>>()?;

        // Parse buttons: (3) (1,3) (2) ...
        let buttons_str = &rest[..joltage_start];
        let buttons: Vec<Vec<i32>> = buttons_str
            .split(')')
            .filter_map(|s| {
                let s = s.trim();
                s.find('(').map(|start| &s[start + 1..])
            })
            .map(|s| {
                s.split(',')
                    .map(|n| n.trim().parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Machine {
            indicators,
            buttons,
            joltages,
        })
    }

    fn press_button(&mut self, button_group: &[i32]) {
        for &index in button_group {
            if let Some(indicator) = self.indicators.get_mut(index as usize) {
                indicator.current_state = !indicator.current_state;
            }
        }
    }

    /// Find minimum button presses using Gaussian elimination over GF(2)
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

    /// Build the augmented matrix [A | b]
    ///
    /// Each row represents an indicator light.
    /// Each column (except the last) represents a button.
    /// matrix[i][j] = true means "button j toggles indicator i"
    /// The last column is the target: desired_state XOR current_state
    /// (what needs to change for each indicator).
    fn build_augmented_matrix(&self) -> Vec<Vec<bool>> {
        let num_indicators = self.indicators.len();
        let num_buttons = self.buttons.len();

        let mut matrix = vec![vec![false; num_buttons + 1]; num_indicators];

        // Fill in which buttons affect which indicators
        for (btn_idx, button) in self.buttons.iter().enumerate() {
            for &indicator_idx in button {
                let idx = indicator_idx as usize;
                if idx < num_indicators {
                    matrix[idx][btn_idx] = true;
                }
            }
        }

        // Fill in the target column: what needs to flip
        for (i, indicator) in self.indicators.iter().enumerate() {
            matrix[i][num_buttons] = indicator.desired_state ^ indicator.current_state;
        }

        matrix
    }
}

/// Gaussian Elimination over GF(2)
///
/// Transforms the matrix into reduced row echelon form using XOR operations.
/// Returns the indices of pivot columns.
fn gaussian_eliminate(matrix: &mut [Vec<bool>], num_buttons: usize) -> Vec<usize> {
    let num_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..num_buttons {
        // Find a row with a 1 in this column
        let pivot_found = (pivot_row..num_rows).find(|&row| matrix[row][col]);

        if let Some(found) = pivot_found {
            matrix.swap(pivot_row, found);

            // Eliminate all other 1s in this column using XOR
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

/// XOR row `target` with row `source`
fn xor_rows(matrix: &mut [Vec<bool>], target: usize, source: usize) {
    let num_cols = matrix[target].len();
    for col in 0..num_cols {
        matrix[target][col] ^= matrix[source][col];
    }
}

/// Check if the system has a solution.
/// A row of all zeros with a 1 in the target column means no solution.
fn is_consistent(matrix: &[Vec<bool>], pivot_cols: &[usize], num_buttons: usize) -> bool {
    let num_pivots = pivot_cols.len();

    for row in matrix.iter().skip(num_pivots) {
        if row[num_buttons] {
            return false;
        }
    }

    true
}

/// Find free columns (buttons we can choose to press or not).
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

/// Compute a full solution given an assignment of free variables.
fn compute_solution(
    matrix: &[Vec<bool>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    free_assignment: u64,
    num_buttons: usize,
) -> Vec<bool> {
    let mut solution = vec![false; num_buttons];

    // Set free variables from the bit pattern
    for (i, &col) in free_cols.iter().enumerate() {
        solution[col] = (free_assignment >> i) & 1 == 1;
    }

    // Back-substitute to find pivot variables
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
