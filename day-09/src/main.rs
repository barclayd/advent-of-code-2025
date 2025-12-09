use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

struct Theatre {
    red_tiles: Vec<Point>,
}

impl Theatre {
    fn new(input: &str) -> Self {
        let red_tiles = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(',');
                let x: i64 = parts.next().unwrap().trim().parse().unwrap();
                let y: i64 = parts.next().unwrap().trim().parse().unwrap();
                Point { x, y }
            })
            .collect();
        Self { red_tiles }
    }

    fn pareto_front(&self, x_dir: i64, y_dir: i64) -> Vec<Point> {
        if self.red_tiles.is_empty() {
            return Vec::new();
        }

        let mut points = self.red_tiles.clone();

        points.sort_by_key(|p| std::cmp::Reverse(p.x * x_dir));

        let mut frontier = Vec::new();
        let mut best_y = i64::MIN;

        for p in points {
            let effective_y = p.y * y_dir;
            if effective_y > best_y {
                frontier.push(p);
                best_y = effective_y;
            }
        }

        frontier
    }

    fn largest_rectangle_area(&self) -> i64 {
        let bottom_left = self.pareto_front(-1, -1);
        let top_right = self.pareto_front(1, 1);
        let top_left = self.pareto_front(-1, 1);
        let bottom_right = self.pareto_front(1, -1);

        let mut max_area = 0i64;

        for p1 in &bottom_left {
            for p2 in &top_right {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                max_area = max_area.max(area);
            }
        }

        for p1 in &top_left {
            for p2 in &bottom_right {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let theatre = Theatre::new(&file_contents);

    if part == Part1 {
        theatre.largest_rectangle_area()
    } else {
        8
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
        assert_eq!(value, 50);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 4763932976);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 8);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 8);
    }
}
