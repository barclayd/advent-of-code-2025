use crate::Part::{Part1, Part2};
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

const ADJACENT_POSITIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    RollOfPaper,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Tile::RollOfPaper,
            _ => Tile::Empty,
        }
    }
}

struct PrintingDepartment {
    grid: Vec<Vec<Tile>>,
}

impl PrintingDepartment {
    fn new(input: &String) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Self { grid }
    }

    fn count_adjacent_rolls(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for &(dx, dy) in ADJACENT_POSITIONS.iter() {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);

            if let Some(&Tile::RollOfPaper) = self.grid.get(ny).and_then(|row| row.get(nx)) {
                count += 1;
            }
        }
        count
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut total = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == Tile::RollOfPaper {
                    let adjacent = self.count_adjacent_rolls(x, y);
                    if adjacent < 4 {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    fn count_total_removable_rolls(&mut self) -> usize {
        let mut count = 0;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut in_queue: HashSet<(usize, usize)> = HashSet::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == Tile::RollOfPaper && self.count_adjacent_rolls(x, y) < 4 {
                    queue.push_back((x, y));
                    in_queue.insert((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            in_queue.remove(&(x, y));

            if self.grid[y][x] != Tile::RollOfPaper {
                continue;
            }

            self.grid[y][x] = Tile::Empty;
            count += 1;

            for &(dx, dy) in ADJACENT_POSITIONS.iter() {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);

                if let Some(&Tile::RollOfPaper) = self.grid.get(ny).and_then(|row| row.get(nx)) {
                    if self.count_adjacent_rolls(nx, ny) < 4 && !in_queue.contains(&(nx, ny)) {
                        queue.push_back((nx, ny));
                        in_queue.insert((nx, ny));
                    }
                }
            }
        }

        count
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut printing_department = PrintingDepartment::new(&file_contents);

    if part == Part1 {
        printing_department.count_accessible_rolls()
    } else {
        printing_department.count_total_removable_rolls()
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
        assert_eq!(value, 13);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 1551);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 43);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 9784);
    }
}
