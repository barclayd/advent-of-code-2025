use crate::Part::{Part1, Part2};
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone)]
struct Device {
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Server {
    devices: HashMap<String, Device>,
}

impl Server {
    fn new(input: String) -> Self {
        let devices = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (id, outputs_str) = line.split_once(": ").expect("invalid format");
                let outputs = outputs_str.split_whitespace().map(String::from).collect();
                let device = Device {
                    outputs,
                };
                (id.to_string(), device)
            })
            .collect();

        Self { devices }
    }

    fn count_paths(&self, from: &str, to: &str) -> u64 {
        let mut cache: HashMap<&str, u64> = HashMap::new();
        self.count_paths_memo(from, to, &mut cache)
    }

    fn count_paths_memo<'a>(
        &'a self,
        current: &'a str,
        target: &str,
        cache: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if current == target {
            return 1;
        }

        if let Some(&count) = cache.get(current) {
            return count;
        }

        let Some(device) = self.devices.get(current) else {
            return 0;
        };

        let count = device
            .outputs
            .iter()
            .map(|output| self.count_paths_memo(output, target, cache))
            .sum();

        cache.insert(current, count);

        count
    }

    fn count_paths_through_both(
        &self,
        start: &str,
        end: &str,
        checkpoint1: &str,
        checkpoint2: &str,
    ) -> u64 {
        let order1 = self.count_paths(start, checkpoint1)
            * self.count_paths(checkpoint1, checkpoint2)
            * self.count_paths(checkpoint2, end);

        let order2 = self.count_paths(start, checkpoint2)
            * self.count_paths(checkpoint2, checkpoint1)
            * self.count_paths(checkpoint1, end);

        order1 + order2
    }
}

fn get_value(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let server = Server::new(file_contents);

    if part == Part1 {
        server.count_paths("you", "out") as i64
    } else {
        server.count_paths_through_both("svr", "out", "dac", "fft") as i64
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
        assert_eq!(value, 5);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 566);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test2.txt", Part2);
        assert_eq!(value, 2);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 331837854931968);
    }
}
