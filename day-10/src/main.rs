use crate::Part::{Part1, Part2};
use std::fs;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    ParseInt,
}

#[derive(Debug, Clone)]
struct Indicator {
    desired_state: bool,
    current_state: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational {
    num: i64,
    den: i64,
}

impl Default for Rational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rational {
    const ZERO: Self = Self { num: 0, den: 1 };

    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "Division by zero");

        if num == 0 {
            return Self::ZERO;
        }

        let g = gcd(num.abs(), den.abs());
        let sign = if den < 0 { -1 } else { 1 };

        Self {
            num: sign * num / g,
            den: sign * den / g,
        }
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn is_non_negative(self) -> bool {
        self.num >= 0
    }

    fn to_i64(self) -> Option<i64> {
        if self.num % self.den == 0 {
            Some(self.num / self.den)
        } else {
            None
        }
    }
}

const fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Self {
        Self { num: n, den: 1 }
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.num, self.den)
    }
}

fn parse_indicator(c: char) -> Result<Indicator, ParseError> {
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

        let indicators = Self::parse_indicators(line)?;
        let rest = &line[line.find(']').ok_or(ParseError::InvalidFormat)? + 1..];

        let joltage_start = rest.find('{').ok_or(ParseError::InvalidFormat)?;
        let joltages = Self::parse_joltages(&rest[joltage_start..])?;
        let buttons = Self::parse_buttons(&rest[..joltage_start])?;

        Ok(Machine {
            indicators,
            buttons,
            joltages,
        })
    }

    fn parse_indicators(line: &str) -> Result<Vec<Indicator>, ParseError> {
        let start = line.find('[').ok_or(ParseError::InvalidFormat)?;
        let end = line.find(']').ok_or(ParseError::InvalidFormat)?;

        line[start + 1..end].chars().map(parse_indicator).collect()
    }

    fn parse_joltages(s: &str) -> Result<Vec<i32>, ParseError> {
        let start = s.find('{').ok_or(ParseError::InvalidFormat)?;
        let end = s.find('}').ok_or(ParseError::InvalidFormat)?;

        s[start + 1..end]
            .split(',')
            .map(|n| n.trim().parse().map_err(|_| ParseError::ParseInt))
            .collect()
    }

    fn parse_buttons(s: &str) -> Result<Vec<Vec<i32>>, ParseError> {
        s.split(')')
            .filter_map(|part| part.find('(').map(|i| &part[i + 1..]))
            .map(|part| {
                part.split(',')
                    .map(|n| n.trim().parse().map_err(|_| ParseError::ParseInt))
                    .collect()
            })
            .collect()
    }
}

impl Factory {
    fn new(input: &str) -> Result<Self, ParseError> {
        let machines = input
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

    fn total_min_joltage_presses(&self) -> Option<i64> {
        self.machines.iter().map(|m| m.min_joltage_presses()).sum()
    }
}

mod gf2 {
    pub fn gaussian_eliminate(matrix: &mut [Vec<bool>], num_cols: usize) -> Vec<usize> {
        let num_rows = matrix.len();
        let mut pivot_cols = Vec::new();
        let mut pivot_row = 0;

        for col in 0..num_cols {
            let Some(found) = (pivot_row..num_rows).find(|&r| matrix[r][col]) else {
                continue;
            };

            matrix.swap(pivot_row, found);

            for row in 0..num_rows {
                if row != pivot_row && matrix[row][col] {
                    xor_row(matrix, row, pivot_row);
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }

        pivot_cols
    }

    fn xor_row(matrix: &mut [Vec<bool>], target: usize, source: usize) {
        for col in 0..matrix[target].len() {
            matrix[target][col] ^= matrix[source][col];
        }
    }

    pub fn is_consistent(matrix: &[Vec<bool>], num_pivots: usize, augment_col: usize) -> bool {
        matrix.iter().skip(num_pivots).all(|row| !row[augment_col])
    }

    pub fn find_min_weight_solution(
        matrix: &[Vec<bool>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        num_vars: usize,
    ) -> usize {
        let num_free = free_cols.len();

        (0u64..(1u64 << num_free))
            .map(|assignment| {
                let solution =
                    compute_solution(matrix, pivot_cols, free_cols, assignment, num_vars);
                solution.iter().filter(|&&x| x).count()
            })
            .min()
            .unwrap_or(0)
    }

    fn compute_solution(
        matrix: &[Vec<bool>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        free_assignment: u64,
        num_vars: usize,
    ) -> Vec<bool> {
        let mut solution = vec![false; num_vars];
        let augment_col = num_vars;

        for (i, &col) in free_cols.iter().enumerate() {
            solution[col] = (free_assignment >> i) & 1 == 1;
        }

        for (row, &pivot_col) in pivot_cols.iter().enumerate().rev() {
            let mut val = matrix[row][augment_col];

            for col in (pivot_col + 1)..num_vars {
                if matrix[row][col] && solution[col] {
                    val ^= true;
                }
            }

            solution[pivot_col] = val;
        }

        solution
    }
}

impl Machine {
    fn min_presses(&self) -> Option<usize> {
        let num_buttons = self.buttons.len();

        if num_buttons == 0 {
            return self
                .indicators
                .iter()
                .all(|i| i.current_state == i.desired_state)
                .then_some(0);
        }

        let mut matrix = self.build_gf2_matrix();
        let pivot_cols = gf2::gaussian_eliminate(&mut matrix, num_buttons);

        if !gf2::is_consistent(&matrix, pivot_cols.len(), num_buttons) {
            return None;
        }

        let free_cols = free_columns(num_buttons, &pivot_cols);
        Some(gf2::find_min_weight_solution(
            &matrix,
            &pivot_cols,
            &free_cols,
            num_buttons,
        ))
    }

    fn build_gf2_matrix(&self) -> Vec<Vec<bool>> {
        let num_indicators = self.indicators.len();
        let num_buttons = self.buttons.len();

        let mut matrix = vec![vec![false; num_buttons + 1]; num_indicators];

        for (btn, button) in self.buttons.iter().enumerate() {
            for &idx in button {
                if (idx as usize) < num_indicators {
                    matrix[idx as usize][btn] = true;
                }
            }
        }

        for (i, ind) in self.indicators.iter().enumerate() {
            matrix[i][num_buttons] = ind.desired_state ^ ind.current_state;
        }

        matrix
    }
}

mod rational_la {
    use super::Rational;

    pub fn gaussian_eliminate(matrix: &mut [Vec<Rational>], num_cols: usize) -> Vec<usize> {
        let num_rows = matrix.len();
        let mut pivot_cols = Vec::new();
        let mut pivot_row = 0;

        for col in 0..num_cols {
            let Some(found) = (pivot_row..num_rows).find(|&r| !matrix[r][col].is_zero()) else {
                continue;
            };

            matrix.swap(pivot_row, found);

            let pivot_val = matrix[pivot_row][col];
            for c in 0..matrix[pivot_row].len() {
                matrix[pivot_row][c] = matrix[pivot_row][c] / pivot_val;
            }

            for row in 0..num_rows {
                if row != pivot_row && !matrix[row][col].is_zero() {
                    let factor = matrix[row][col];
                    for c in 0..matrix[row].len() {
                        matrix[row][c] = matrix[row][c] - factor * matrix[pivot_row][c];
                    }
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }

        pivot_cols
    }

    pub fn is_consistent(matrix: &[Vec<Rational>], num_pivots: usize, num_vars: usize) -> bool {
        matrix.iter().skip(num_pivots).all(|row| {
            let all_zero = (0..num_vars).all(|c| row[c].is_zero());
            !all_zero || row[num_vars].is_zero()
        })
    }

    pub fn find_min_solution(
        matrix: &[Vec<Rational>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        num_vars: usize,
    ) -> Option<i64> {
        if free_cols.is_empty() {
            return compute_unique_solution(matrix, pivot_cols, num_vars);
        }

        let bound = compute_search_bound(matrix, num_vars);
        let mut min_result: Option<i64> = None;

        search_free_vars(
            matrix,
            pivot_cols,
            free_cols,
            num_vars,
            0,
            &mut vec![0; free_cols.len()],
            bound,
            &mut min_result,
        );

        min_result
    }

    fn compute_unique_solution(
        matrix: &[Vec<Rational>],
        pivot_cols: &[usize],
        num_vars: usize,
    ) -> Option<i64> {
        let mut total = 0i64;

        for (row, _) in pivot_cols.iter().enumerate() {
            let val = matrix[row][num_vars];

            if !val.is_non_negative() {
                return None;
            }

            total += val.to_i64()?;
        }

        Some(total)
    }

    fn compute_search_bound(matrix: &[Vec<Rational>], num_vars: usize) -> i64 {
        matrix
            .iter()
            .map(|row| {
                let r = row[num_vars];
                if r.den == 0 { 0 } else { r.num.abs() / r.den }
            })
            .max()
            .unwrap_or(0)
            + 1
    }

    fn search_free_vars(
        matrix: &[Vec<Rational>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        num_vars: usize,
        depth: usize,
        free_vals: &mut Vec<i64>,
        bound: i64,
        best: &mut Option<i64>,
    ) {
        if depth == free_cols.len() {
            if let Some(total) =
                compute_solution(matrix, pivot_cols, free_cols, free_vals, num_vars)
            {
                if best.is_none_or(|b| total < b) {
                    *best = Some(total);
                }
            }
            return;
        }

        let current_sum: i64 = free_vals[..depth].iter().sum();
        if best.is_some_and(|b| current_sum >= b) {
            return;
        }

        for val in 0..=bound {
            free_vals[depth] = val;
            search_free_vars(
                matrix,
                pivot_cols,
                free_cols,
                num_vars,
                depth + 1,
                free_vals,
                bound,
                best,
            );
        }
    }

    fn compute_solution(
        matrix: &[Vec<Rational>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        free_vals: &[i64],
        num_vars: usize,
    ) -> Option<i64> {
        let mut solution = vec![Rational::ZERO; num_vars];

        for (i, &col) in free_cols.iter().enumerate() {
            solution[col] = Rational::from(free_vals[i]);
        }

        for (row, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut val = matrix[row][num_vars];

            for col in (pivot_col + 1)..num_vars {
                if !matrix[row][col].is_zero() {
                    val = val - matrix[row][col] * solution[col];
                }
            }

            solution[pivot_col] = val;
        }

        let mut total = 0i64;
        for val in solution {
            if !val.is_non_negative() {
                return None;
            }
            total += val.to_i64()?;
        }

        Some(total)
    }
}

impl Machine {
    fn min_joltage_presses(&self) -> Option<i64> {
        let num_buttons = self.buttons.len();

        if num_buttons == 0 {
            return self.joltages.iter().all(|&j| j == 0).then_some(0);
        }

        let mut matrix = self.build_rational_matrix();
        let pivot_cols = rational_la::gaussian_eliminate(&mut matrix, num_buttons);

        if !rational_la::is_consistent(&matrix, pivot_cols.len(), num_buttons) {
            return None;
        }

        let free_cols = free_columns(num_buttons, &pivot_cols);
        rational_la::find_min_solution(&matrix, &pivot_cols, &free_cols, num_buttons)
    }

    fn build_rational_matrix(&self) -> Vec<Vec<Rational>> {
        let num_counters = self.joltages.len();
        let num_buttons = self.buttons.len();

        let mut matrix = vec![vec![Rational::ZERO; num_buttons + 1]; num_counters];

        for (btn, button) in self.buttons.iter().enumerate() {
            for &idx in button {
                if (idx as usize) < num_counters {
                    matrix[idx as usize][btn] = Rational::from(1);
                }
            }
        }

        for (i, &joltage) in self.joltages.iter().enumerate() {
            matrix[i][num_buttons] = Rational::from(joltage as i64);
        }

        matrix
    }
}

fn free_columns(num_vars: usize, pivot_cols: &[usize]) -> Vec<usize> {
    (0..num_vars).filter(|c| !pivot_cols.contains(c)).collect()
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let factory = Factory::new(&contents).expect("Failed to parse input");

    match part {
        Part1 => factory.total_min_presses().unwrap_or(0),
        Part2 => factory.total_min_joltage_presses().unwrap_or(0),
    }
}

fn main() {
    println!("Part 1 value: {}", get_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        assert_eq!(get_value("./test.txt", Part1), 7);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        assert_eq!(get_value("./input.txt", Part1), 524);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        assert_eq!(get_value("./test.txt", Part2), 33);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        assert_eq!(get_value("./input.txt", Part2), 21696);
    }
}
