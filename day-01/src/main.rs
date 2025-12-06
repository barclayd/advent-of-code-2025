use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn rotate_dial(starting_position: i32, direction: Direction) -> i32 {
    match direction {
        Direction::Left(x) => (starting_position - x).rem_euclid(100),
        Direction::Right(x) => (starting_position + x).rem_euclid(100),
    }
}

fn rotate_dial_with_wrappings(starting_position: i32, direction: Direction) -> (i32, i32) {
    let offset = match direction {
        Direction::Left(x) => starting_position - x,
        Direction::Right(x) => starting_position + x,
    };

    (offset.rem_euclid(100), offset.div_euclid(100).abs())
}

fn parse_instructions(file_path: &str) -> Vec<Direction> {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(|line| {
            let num = line[1..].parse().unwrap();
            match line.as_bytes()[0] {
                b'L' => Direction::Left(num),
                b'R' => Direction::Right(num),
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}

fn count_with_strategy<F>(file_path: &str, mut counter: F) -> i32
where
    F: FnMut(i32, Direction) -> (i32, i32),
{
    let instructions = parse_instructions(file_path);
    let mut position = 50;
    let mut count = 0;

    for instruction in instructions {
        let (new_position, increment) = counter(position, instruction);
        count += increment;
        position = new_position;
    }

    count
}

fn get_count_of_rotations_completed_at_0(file_path: &str) -> i32 {
    count_with_strategy(file_path, |position, instruction| {
        let new_position = rotate_dial(position, instruction);
        let increment = if new_position == 0 { 1 } else { 0 };
        (new_position, increment)
    })
}

fn get_count_of_rotations_past_0(file_path: &str) -> i32 {
    count_with_strategy(file_path, |position, instruction| {
        let (new_position, mut wrappings) = rotate_dial_with_wrappings(position, instruction);

        if position == 0 && matches!(instruction, Direction::Left(_)) {
            wrappings = wrappings.saturating_sub(1);
        }

        if let Direction::Left(x) = instruction
            && new_position == 0
            && x > 0
        {
            wrappings += 1;
        }

        (new_position, wrappings)
    })
}

fn main() {
    println!(
        "Part 1 - Rotations completed at 0: {}",
        get_count_of_rotations_completed_at_0("./input.txt")
    );

    println!(
        "Part 2 - Rotations past 0: {}",
        get_count_of_rotations_past_0("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::{get_count_of_rotations_completed_at_0, get_count_of_rotations_past_0};

    #[test]
    fn returns_expected_count_of_rotations_completed_at_0_for_test_data() {
        let count = get_count_of_rotations_completed_at_0("./test.txt");
        assert_eq!(count, 3);
    }

    #[test]
    fn returns_expected_count_of_rotations_completed_at_0_for_input_data() {
        let count = get_count_of_rotations_completed_at_0("./input.txt");
        assert_eq!(count, 1147);
    }

    #[test]
    fn returns_expected_count_of_wrappings_past_0_for_test_data() {
        let count = get_count_of_rotations_past_0("./test.txt");
        assert_eq!(count, 6);
    }

    #[test]
    fn returns_expected_count_of_wrappings_past_0_for_input_data() {
        let count = get_count_of_rotations_past_0("./input.txt");
        assert_eq!(count, 6789);
    }
}
