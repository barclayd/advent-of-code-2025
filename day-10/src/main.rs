use crate::Part::{Part1, Part2};
use std::fs;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational {
    num: i64,
    den: i64,
}

const fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl Rational {
    fn new(num: i64, den: i64) -> Self {
        if den == 0 {
            panic!("Division by zero");
        }
        if num == 0 {
            return Rational { num: 0, den: 1 };
        }
        let g = gcd(num.abs(), den.abs());
        let sign = if den < 0 { -1 } else { 1 };
        Rational {
            num: sign * num / g,
            den: (sign * den).abs() / g,
        }
    }

    fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }

    fn from_i64(n: i64) -> Self {
        Rational { num: n, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn to_i64(&self) -> Option<i64> {
        if self.den == 1 {
            Some(self.num)
        } else if self.num % self.den == 0 {
            Some(self.num / self.den)
        } else {
            None
        }
    }

    fn is_non_negative(&self) -> bool {
        self.num >= 0
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Rational::new(-self.num, self.den)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.num * other.den).cmp(&(other.num * self.den)))
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

        let mut matrix = self.build_gf2_matrix();
        let pivot_cols = gf2_gaussian_eliminate(&mut matrix, num_buttons);

        if !gf2_is_consistent(&matrix, &pivot_cols, num_buttons) {
            return None;
        }

        let free_cols = find_free_columns(num_buttons, &pivot_cols);

        Some(gf2_find_minimum_weight_solution(
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

    fn min_joltage_presses(&self) -> Option<i64> {
        let num_counters = self.joltages.len();
        let num_buttons = self.buttons.len();

        if num_buttons == 0 {
            return if self.joltages.iter().all(|&j| j == 0) {
                Some(0)
            } else {
                None
            };
        }

        let mut matrix = self.build_rational_matrix();
        let pivot_cols = rational_gaussian_eliminate(&mut matrix, num_buttons);

        if !rational_is_consistent(&matrix, &pivot_cols, num_buttons, num_counters) {
            return None;
        }

        let free_cols = find_free_columns(num_buttons, &pivot_cols);

        rational_find_minimum_solution(&matrix, &pivot_cols, &free_cols, num_buttons)
    }

    fn build_rational_matrix(&self) -> Vec<Vec<Rational>> {
        let num_counters = self.joltages.len();
        let num_buttons = self.buttons.len();

        let mut matrix = vec![vec![Rational::zero(); num_buttons + 1]; num_counters];

        for (btn_idx, button) in self.buttons.iter().enumerate() {
            for &counter_idx in button {
                let idx = counter_idx as usize;
                if idx < num_counters {
                    matrix[idx][btn_idx] = Rational::from_i64(1);
                }
            }
        }

        for (i, &joltage) in self.joltages.iter().enumerate() {
            matrix[i][num_buttons] = Rational::from_i64(joltage as i64);
        }

        matrix
    }
}

fn gf2_gaussian_eliminate(matrix: &mut [Vec<bool>], num_buttons: usize) -> Vec<usize> {
    let num_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..num_buttons {
        let pivot_found = (pivot_row..num_rows).find(|&row| matrix[row][col]);

        if let Some(found) = pivot_found {
            matrix.swap(pivot_row, found);

            for row in 0..num_rows {
                if row != pivot_row && matrix[row][col] {
                    gf2_xor_rows(matrix, row, pivot_row);
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    pivot_cols
}

fn gf2_xor_rows(matrix: &mut [Vec<bool>], target: usize, source: usize) {
    let num_cols = matrix[target].len();
    for col in 0..num_cols {
        matrix[target][col] ^= matrix[source][col];
    }
}

fn gf2_is_consistent(matrix: &[Vec<bool>], pivot_cols: &[usize], num_buttons: usize) -> bool {
    let num_pivots = pivot_cols.len();

    for row in matrix.iter().skip(num_pivots) {
        if row[num_buttons] {
            return false;
        }
    }

    true
}

fn gf2_find_minimum_weight_solution(
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
            gf2_compute_solution(matrix, pivot_cols, free_cols, free_assignment, num_buttons);

        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);

        if min_presses <= 1 {
            break;
        }
    }

    min_presses
}

fn gf2_compute_solution(
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

fn rational_gaussian_eliminate(matrix: &mut [Vec<Rational>], num_buttons: usize) -> Vec<usize> {
    let num_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..num_buttons {
        let pivot_found = (pivot_row..num_rows).find(|&row| !matrix[row][col].is_zero());

        if let Some(found) = pivot_found {
            matrix.swap(pivot_row, found);

            let pivot_val = matrix[pivot_row][col];
            let num_cols = matrix[pivot_row].len();
            for c in 0..num_cols {
                matrix[pivot_row][c] = matrix[pivot_row][c] / pivot_val;
            }

            for row in 0..num_rows {
                if row != pivot_row && !matrix[row][col].is_zero() {
                    let factor = matrix[row][col];
                    for c in 0..num_cols {
                        let sub = factor * matrix[pivot_row][c];
                        matrix[row][c] = matrix[row][c] - sub;
                    }
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    pivot_cols
}

fn rational_is_consistent(
    matrix: &[Vec<Rational>],
    pivot_cols: &[usize],
    num_buttons: usize,
    num_rows: usize,
) -> bool {
    let num_pivots = pivot_cols.len();

    for row in num_pivots..num_rows {
        let all_zero = (0..num_buttons).all(|c| matrix[row][c].is_zero());
        if all_zero && !matrix[row][num_buttons].is_zero() {
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

fn rational_find_minimum_solution(
    matrix: &[Vec<Rational>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    num_buttons: usize,
) -> Option<i64> {
    let num_free = free_cols.len();

    if num_free == 0 {
        let mut total = 0i64;
        for (row_idx, &_pivot_col) in pivot_cols.iter().enumerate() {
            let val = matrix[row_idx][num_buttons];
            if !val.is_non_negative() {
                return None;
            }
            match val.to_i64() {
                Some(n) => total += n,
                None => return None,
            }
        }
        return Some(total);
    }

    let max_target = matrix
        .iter()
        .map(|row| row[num_buttons].num.abs() / row[num_buttons].den.max(1))
        .max()
        .unwrap_or(0) as usize;

    let bound = max_target + 1;

    let mut min_presses: Option<i64> = None;

    enumerate_free_vars(
        matrix,
        pivot_cols,
        free_cols,
        num_buttons,
        0,
        &mut vec![0i64; num_free],
        bound,
        &mut min_presses,
    );

    min_presses
}

fn enumerate_free_vars(
    matrix: &[Vec<Rational>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    num_buttons: usize,
    free_idx: usize,
    free_vals: &mut Vec<i64>,
    bound: usize,
    min_presses: &mut Option<i64>,
) {
    let num_free = free_cols.len();

    if free_idx == num_free {
        if let Some(total) =
            compute_rational_solution(matrix, pivot_cols, free_cols, free_vals, num_buttons)
        {
            match min_presses {
                Some(current_min) if total < *current_min => *min_presses = Some(total),
                None => *min_presses = Some(total),
                _ => {}
            }
        }
        return;
    }

    let current_free_sum: i64 = free_vals[..free_idx].iter().sum();
    if let Some(current_min) = min_presses {
        if current_free_sum >= *current_min {
            return;
        }
    }

    for val in 0..=bound {
        free_vals[free_idx] = val as i64;
        enumerate_free_vars(
            matrix,
            pivot_cols,
            free_cols,
            num_buttons,
            free_idx + 1,
            free_vals,
            bound,
            min_presses,
        );
    }
}

fn compute_rational_solution(
    matrix: &[Vec<Rational>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    free_vals: &[i64],
    num_buttons: usize,
) -> Option<i64> {
    let mut solution = vec![Rational::zero(); num_buttons];

    for (i, &col) in free_cols.iter().enumerate() {
        solution[col] = Rational::from_i64(free_vals[i]);
    }

    for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
        let mut val = matrix[row_idx][num_buttons];

        for col in (pivot_col + 1)..num_buttons {
            if !matrix[row_idx][col].is_zero() {
                val = val - matrix[row_idx][col] * solution[col];
            }
        }

        solution[pivot_col] = val;
    }

    let mut total = 0i64;
    for val in &solution {
        if !val.is_non_negative() {
            return None;
        }
        match val.to_i64() {
            Some(n) => total += n,
            None => return None,
        }
    }

    Some(total)
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

    fn total_min_joltage_presses(&self) -> Option<i64> {
        self.machines.iter().map(|m| m.min_joltage_presses()).sum()
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let factory = Factory::new(&file_contents).expect("Failed to parse input");

    if part == Part1 {
        factory.total_min_presses().unwrap_or(0)
    } else {
        factory.total_min_joltage_presses().unwrap_or(0)
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
        assert_eq!(value, 33);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 21696);
    }
}
