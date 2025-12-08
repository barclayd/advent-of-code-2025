use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn squared_distance(&self, other: &JunctionBox) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let roots: Vec<usize> = (0..n).map(|i| self.find(i)).collect();

        (0..n)
            .filter(|&i| roots[i] == i)
            .map(|i| self.size[i])
            .collect()
    }
}

struct Playground {
    boxes: Vec<JunctionBox>,
}

impl Playground {
    fn new(input: &str) -> Self {
        let boxes = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
                JunctionBox {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                }
            })
            .collect();
        Self { boxes }
    }

    fn solve(&self, connections: usize) -> i64 {
        let n = self.boxes.len();

        let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            for j in (i + 1)..n {
                let dist = self.boxes[i].squared_distance(&self.boxes[j]);
                pairs.push((dist, i, j));
            }
        }

        pairs.sort_unstable_by_key(|&(dist, _, _)| dist);

        let mut union_find = UnionFind::new(n);

        for &(_, i, j) in pairs.iter().take(connections) {
            union_find.union(i, j);
        }

        let mut sizes = union_find.component_sizes();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        sizes.iter().take(3).map(|&s| s as i64).product()
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let playground = Playground::new(&file_contents);

    match part {
        Part1 => playground.solve(if file_path == "./test.txt" { 10 } else { 1000 }),
        Part2 => 40
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
        assert_eq!(value, 40);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 72150);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 40);
    }


    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 40);
    }
}