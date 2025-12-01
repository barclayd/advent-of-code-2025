use std::collections::HashMap;
use std::fs;

fn get_total_distance_between_lists(file_path: &str) -> i32 {
    5
}

fn get_similarity_score_between_lists(file_path: &str) -> i32 {
    5
}

fn main() {
    println!(
        "Total distance between lists: {}",
        get_total_distance_between_lists("./input.txt")
    );
    println!(
        "Similarity score between lists: {}",
        get_similarity_score_between_lists("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::{get_similarity_score_between_lists, get_total_distance_between_lists};

    #[test]
    fn returns_expected_total_distance_between_lists_for_test_data() {
        let total_distance_between_lists = get_total_distance_between_lists("./test.txt");
        assert_eq!(total_distance_between_lists, 5);
    }

    #[test]
    fn returns_expected_total_distance_between_lists_for_input_data() {
        let total_distance_between_lists = get_total_distance_between_lists("./input.txt");
        assert_eq!(total_distance_between_lists, 5);
    }

    #[test]
    fn returns_expected_similarity_score_for_test_data() {
        let similarity_score = get_similarity_score_between_lists("./test.txt");
        assert_eq!(similarity_score, 5);
    }

    #[test]
    fn returns_expected_similarity_score_for_input_data() {
        let similarity_score = get_similarity_score_between_lists("./input.txt");
        assert_eq!(similarity_score, 5);
    }
}
