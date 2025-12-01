use std::fs;

enum Direction {
    Left(i32),
    Right(i32),
}

fn rotate_dial(starting_position: i32, direction: &Direction) -> i32 {
    match direction {
        Direction::Left(x) => (starting_position - x).rem_euclid(100),
        Direction::Right(x) => (starting_position + x).rem_euclid(100),
    }
}

fn get_count_of_rotations_completed_at_0(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let instructions: Vec<Direction> = file_contents
        .lines()
        .map(|line| {
            let num = line[1..].parse().unwrap();
            match line.as_bytes()[0] {
                b'L' => Direction::Left(num),
                b'R' => Direction::Right(num),
                _ => panic!("Invalid input"),
            }
        })
        .collect();

    let mut position = 50;
    let mut count = 0;

    instructions.iter().for_each(|instruction| {
        position = rotate_dial(position, instruction);

        if position == 0 {
            count += 1;
        }
    });

    return count;
}

fn main() {
    println!(
        "Similarity score between lists: {}",
        get_count_of_rotations_completed_at_0("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::get_count_of_rotations_completed_at_0;

    #[test]
    fn returns_expected_similarity_score_for_test_data() {
        let similarity_score = get_count_of_rotations_completed_at_0("./test.txt");
        assert_eq!(similarity_score, 3);
    }

    #[test]
    fn returns_expected_similarity_score_for_input_data() {
        let similarity_score = get_count_of_rotations_completed_at_0("./input.txt");
        assert_eq!(similarity_score, 1147);
    }
}
