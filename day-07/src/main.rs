use crate::Part::{Part1, Part2};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Start,
    Splitter,
    Empty,
    Beam,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            '^' => Tile::Splitter,
            '|' => Tile::Beam,
            _ => Tile::Empty,
        }
    }
}

struct TachyonManifold {
    grid: Vec<Vec<Tile>>,
}

impl TachyonManifold {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Self { grid }
    }

    fn extend_beam(&mut self, y: i32, x: i32) {
        if let (Some(row), Some(col)) = (self.grid.get_mut(y as usize), Some(x as usize)) {
            if let Some(cell) = row.get_mut(col) {
                if cell == &Tile::Empty {
                    *cell = Tile::Beam;
                }
            }
        }
    }

    fn find_beam_start(&self) -> Option<(i32, i32)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == Tile::Start {
                    return Some((y as i32, x as i32));
                }
            }
        }
        None
    }

    fn get_tile(&self, y: i32, x: i32) -> Tile {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(Tile::Empty)
    }

    fn split_count(&mut self) -> i64 {
        let (start_y, start_x) = self.find_beam_start().expect("Start of beam not found");

        let mut active_beams: HashSet<i32> = HashSet::new();
        active_beams.insert(start_x);

        let height = self.grid.len() as i32;

        let mut split_count = 0;

        for y in (start_y + 1)..height {
            let mut new_beams: HashSet<i32> = HashSet::new();

            for &x in &active_beams {
                match self.get_tile(y, x) {
                    Tile::Empty => {
                        self.extend_beam(y, x);
                        new_beams.insert(x);
                    }
                    Tile::Splitter => {
                        split_count += 1;

                        if self.get_tile(y, x - 1) == Tile::Empty {
                            self.extend_beam(y, x - 1);
                            new_beams.insert(x - 1);
                        }
                        if self.get_tile(y, x + 1) == Tile::Empty {
                            self.extend_beam(y, x + 1);
                            new_beams.insert(x + 1);
                        }
                    }
                    _ => {}
                }
            }

            active_beams = new_beams;
        }

        split_count
    }

    fn timeline_count(&self) -> i64 {
        let (start_y, start_x) = self.find_beam_start().expect("Start not found");

        let mut timelines: HashMap<i32, i64> = HashMap::new();
        timelines.insert(start_x, 1);

        let height = self.grid.len() as i32;
        let width = self.grid.first().map_or(0, |r| r.len()) as i32;

        for y in (start_y + 1)..height {
            let mut new_timelines: HashMap<i32, i64> = HashMap::new();

            for (&x, &count) in &timelines {
                if x < 0 || x >= width {
                    continue;
                }

                match self.get_tile(y, x) {
                    Tile::Empty => {
                        *new_timelines.entry(x).or_insert(0) += count;
                    }
                    Tile::Splitter => {
                        if x - 1 >= 0 && self.get_tile(y, x - 1) == Tile::Empty {
                            *new_timelines.entry(x - 1).or_insert(0) += count;
                        }
                        if x + 1 < width && self.get_tile(y, x + 1) == Tile::Empty {
                            *new_timelines.entry(x + 1).or_insert(0) += count;
                        }
                    }
                    _ => {}
                }
            }

            timelines = new_timelines;
        }

        timelines.values().sum()
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    match part {
        Part::Part1 => {
            let mut tachyon_manifold = TachyonManifold::new(&file_contents);

            tachyon_manifold.split_count()
        }
        Part::Part2 => {
            let tachyon_manifold = TachyonManifold::new(&file_contents);
            tachyon_manifold.timeline_count()
        }
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
        assert_eq!(value, 21);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 1516);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 40);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 1393669447690);
    }
}
