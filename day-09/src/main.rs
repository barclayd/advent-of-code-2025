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

        let mut front = Vec::new();
        let mut best_y = i64::MIN;

        for p in points {
            let effective_y = p.y * y_dir;
            if effective_y > best_y {
                front.push(p);
                best_y = effective_y;
            }
        }

        front
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

    fn build_segments(&self) -> (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>) {
        let mut v_segs = Vec::new();
        let mut h_segs = Vec::new();

        for i in 0..self.red_tiles.len() {
            let p1 = &self.red_tiles[i];
            let p2 = &self.red_tiles[(i + 1) % self.red_tiles.len()];

            if p1.x == p2.x {
                v_segs.push((p1.x, p1.y.min(p2.y), p1.y.max(p2.y)));
            } else {
                h_segs.push((p1.y, p1.x.min(p2.x), p1.x.max(p2.x)));
            }
        }

        (v_segs, h_segs)
    }

    fn is_point_inside_or_on(
        &self,
        x: i64,
        y: i64,
        v_segs: &[(i64, i64, i64)],
        h_segs: &[(i64, i64, i64)],
    ) -> bool {
        for &(sx, y_min, y_max) in v_segs {
            if x == sx && y >= y_min && y <= y_max {
                return true;
            }
        }
        for &(sy, x_min, x_max) in h_segs {
            if y == sy && x >= x_min && x <= x_max {
                return true;
            }
        }
        let crossings = v_segs
            .iter()
            .filter(|&&(sx, y_min, y_max)| sx > x && y > y_min && y < y_max)
            .count();
        crossings % 2 == 1
    }

    fn is_rectangle_valid(
        &self,
        p1: &Point,
        p2: &Point,
        v_segs: &[(i64, i64, i64)],
        h_segs: &[(i64, i64, i64)],
    ) -> bool {
        let x1 = p1.x.min(p2.x);
        let x2 = p1.x.max(p2.x);
        let y1 = p1.y.min(p2.y);
        let y2 = p1.y.max(p2.y);

        let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
        let given = [(p1.x, p1.y), (p2.x, p2.y)];

        for &(cx, cy) in &corners {
            if !given.contains(&(cx, cy)) {
                if !self.is_point_inside_or_on(cx, cy, v_segs, h_segs) {
                    return false;
                }
            }
        }

        for &(sx, sy_min, sy_max) in v_segs {
            if sx > x1 && sx < x2 && sy_min < y2 && sy_max > y1 {
                return false;
            }
        }

        for &(sy, sx_min, sx_max) in h_segs {
            if sy > y1 && sy < y2 && sx_min < x2 && sx_max > x1 {
                return false;
            }
        }

        true
    }

    fn largest_valid_rectangle_area(&self) -> i64 {
        let (v_segs, h_segs) = self.build_segments();

        let mut max_area = 0i64;

        for i in 0..self.red_tiles.len() {
            for j in (i + 1)..self.red_tiles.len() {
                let p1 = &self.red_tiles[i];
                let p2 = &self.red_tiles[j];

                if self.is_rectangle_valid(p1, p2, &v_segs, &h_segs) {
                    let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                    max_area = max_area.max(area);
                }
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
        theatre.largest_valid_rectangle_area()
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
        assert_eq!(value, 24);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 1501292304);
    }
}
