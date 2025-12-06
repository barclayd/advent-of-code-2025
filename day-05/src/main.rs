use crate::Part::{Part1, Part2};
use rangemap::RangeInclusiveSet;
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut parts = file_contents.split("\n\n");

    let fresh_ingredient_id_ranges: RangeInclusiveSet<i64> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('-').map(|n| n.parse::<i64>().unwrap());
            let start = nums.next().unwrap();
            let end = nums.next().unwrap();
            start..=end
        })
        .collect();

    let available_ingredient_ids: Vec<i64> = parts
        .next()
        .unwrap()
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    if part == Part1 {
        available_ingredient_ids
            .iter()
            .filter(|n| fresh_ingredient_id_ranges.contains(n))
            .count()
    } else {
        fresh_ingredient_id_ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<i64>() as usize
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
        assert_eq!(value, 3);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 679);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 14);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 358155203664116);
    }
}
