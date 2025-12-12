use crate::Part::{Part1, Part2};
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    cells: Vec<(i32, i32)>,
}

impl Shape {
    fn from_grid(grid: Vec<Vec<bool>>) -> Self {
        let mut cells: Vec<(i32, i32)> = Vec::new();

        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell {
                    cells.push((r as i32, c as i32));
                }
            }
        }

        Self::normalize(cells)
    }

    fn normalize(mut cells: Vec<(i32, i32)>) -> Self {
        if cells.is_empty() {
            return Self { cells };
        }

        let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
        let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
        for (r, c) in &mut cells {
            *r -= min_r;
            *c -= min_c;
        }
        cells.sort();
        Self { cells }
    }

    fn rotate_90(&self) -> Self {
        let cells: Vec<(i32, i32)> = self.cells.iter().map(|&(r, c)| (c, -r)).collect();
        Self::normalize(cells)
    }

    fn flip_horizontal(&self) -> Self {
        let cells: Vec<(i32, i32)> = self.cells.iter().map(|&(r, c)| (r, -c)).collect();
        Self::normalize(cells)
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut seen: HashSet<Shape> = HashSet::new();
        let mut orientations: Vec<Shape> = Vec::new();

        let mut current = self.clone();
        for _ in 0..4 {
            if seen.insert(current.clone()) {
                orientations.push(current.clone());
            }
            let flipped = current.flip_horizontal();
            if seen.insert(flipped.clone()) {
                orientations.push(flipped);
            }
            current = current.rotate_90();
        }

        orientations
    }

    fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

#[derive(Debug, Clone)]
struct Present {
    shape_index: usize,
    quantity: u64,
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<Present>,
}

#[derive(Debug)]
struct ChristmasTree {
    shapes: Vec<Shape>,
    shape_orientations: Vec<Vec<Shape>>,
    regions: Vec<Region>,
}

impl ChristmasTree {
    fn new(input: String) -> Self {
        let sections: Vec<&str> = input.split("\n\n").collect();

        let mut shapes: Vec<Shape> = Vec::new();

        let mut region_start_section = 0;
        for (i, section) in sections.iter().enumerate() {
            let first_line = section.lines().next().unwrap_or("");
            if first_line.contains('x')
                && first_line
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_digit())
            {
                region_start_section = i;
                break;
            }

            let lines: Vec<&str> = section.lines().collect();
            if lines.is_empty() {
                continue;
            }

            let shape_lines: Vec<&str> = lines.iter().skip(1).copied().collect();

            let grid: Vec<Vec<bool>> = shape_lines
                .iter()
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();

            if !grid.is_empty() {
                shapes.push(Shape::from_grid(grid));
            }
        }

        let shape_orientations: Vec<Vec<Shape>> =
            shapes.iter().map(|s| s.all_orientations()).collect();

        let regions: Vec<Region> = sections[region_start_section..]
            .join("\n")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (dimensions, quantities_str) =
                    line.split_once(": ").expect("invalid region format");
                let (width_str, height_str) =
                    dimensions.split_once('x').expect("invalid dimensions");

                let width: usize = width_str.parse().expect("invalid width");
                let height: usize = height_str.parse().expect("invalid height");

                let presents: Vec<Present> = quantities_str
                    .split_whitespace()
                    .enumerate()
                    .filter_map(|(shape_index, qty_str)| {
                        let quantity: u64 = qty_str.parse().expect("invalid quantity");
                        if quantity > 0 {
                            Some(Present {
                                shape_index,
                                quantity,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                Region {
                    width,
                    height,
                    presents,
                }
            })
            .collect();

        Self {
            shapes,
            shape_orientations,
            regions,
        }
    }

    fn can_place(
        &self,
        grid: &[Vec<bool>],
        shape: &Shape,
        base_r: i32,
        base_c: i32,
        height: usize,
        width: usize,
    ) -> bool {
        for &(dr, dc) in &shape.cells {
            let r = base_r + dr;
            let c = base_c + dc;
            if r < 0 || c < 0 || r >= height as i32 || c >= width as i32 {
                return false;
            }
            if grid[r as usize][c as usize] {
                return false;
            }
        }
        true
    }

    fn place(&self, grid: &mut [Vec<bool>], shape: &Shape, base_r: i32, base_c: i32) {
        for &(dr, dc) in &shape.cells {
            grid[(base_r + dr) as usize][(base_c + dc) as usize] = true;
        }
    }

    fn unplace(&self, grid: &mut [Vec<bool>], shape: &Shape, base_r: i32, base_c: i32) {
        for &(dr, dc) in &shape.cells {
            grid[(base_r + dr) as usize][(base_c + dc) as usize] = false;
        }
    }

    fn count_empty(&self, grid: &[Vec<bool>]) -> usize {
        grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| !cell)
            .count()
    }

    fn solve(
        &self,
        grid: &mut Vec<Vec<bool>>,
        presents: &mut [(usize, u64)],
        present_idx: usize,
        height: usize,
        width: usize,
    ) -> bool {
        let mut idx = present_idx;
        while idx < presents.len() && presents[idx].1 == 0 {
            idx += 1;
        }

        if idx >= presents.len() {
            return true;
        }

        let empty_cells = self.count_empty(grid);
        let needed_cells: usize = presents[idx..]
            .iter()
            .map(|(shape_idx, count)| self.shapes[*shape_idx].cell_count() * (*count as usize))
            .sum();

        if needed_cells > empty_cells {
            return false;
        }

        let shape_idx = presents[idx].0;

        for orientation in &self.shape_orientations[shape_idx] {
            for r in 0..height {
                for c in 0..width {
                    if self.can_place(grid, orientation, r as i32, c as i32, height, width) {
                        self.place(grid, orientation, r as i32, c as i32);
                        presents[idx].1 -= 1;

                        if self.solve(grid, presents, idx, height, width) {
                            return true;
                        }

                        presents[idx].1 += 1;
                        self.unplace(grid, orientation, r as i32, c as i32);
                    }
                }
            }
        }

        false
    }

    fn can_fit_region(&self, region: &Region) -> bool {
        let mut grid = vec![vec![false; region.width]; region.height];
        let mut presents: Vec<(usize, u64)> = region
            .presents
            .iter()
            .map(|p| (p.shape_index, p.quantity))
            .collect();

        presents.sort_by(|a, b| {
            let size_a = self.shapes[a.0].cell_count() * a.1 as usize;
            let size_b = self.shapes[b.0].cell_count() * b.1 as usize;
            size_b.cmp(&size_a)
        });

        self.solve(&mut grid, &mut presents, 0, region.height, region.width)
    }

    fn count_fitting_regions(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| self.can_fit_region(region))
            .count()
    }
}
fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tree = ChristmasTree::new(file_contents);

    return match part {
        Part1 => tree.count_fitting_regions() as i64,
        Part2 => 1,
    };
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
        assert_eq!(value, 2);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 463);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 1);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 1);
    }
}
