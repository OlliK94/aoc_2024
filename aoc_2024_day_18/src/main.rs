use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::format,
};

const SAFE: char = '.';
const CORRUPTED: char = '#';

fn parse_input(input_text: &str) -> Vec<(usize, usize)> {
    input_text
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let column = parts[0].parse::<usize>().unwrap() + 1;
            let row = parts[1].parse::<usize>().unwrap() + 1;
            (row, column)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node<const N: usize> {
    location: (usize, usize),
    steps: u32,
}

impl<const N: usize> Node<N> {
    pub fn new(location: (usize, usize)) -> Self {
        Self { location, steps: 0 }
    }

    pub fn distance(&self) -> usize {
        (N - self.location.0) + (N - self.location.1)
    }
}

impl<const N: usize> Ord for Node<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_min_req_steps = other.steps as usize + other.distance();
        let self_min_req_steps = self.steps as usize + self.distance();

        other_min_req_steps.cmp(&self_min_req_steps)
    }
}

impl<const N: usize> PartialOrd for Node<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn process_part1<const N: usize>(bytes: &[(usize, usize)], bytes_to_take: usize) -> Option<u32> {
    let start = (1, 1);
    let end = (N - 2, N - 2);
    let mut memory_space: [[char; N]; N] = [[SAFE; N]; N];
    // add padding
    for column_index in 0..N {
        memory_space[0][column_index] = CORRUPTED;
        memory_space[N - 1][column_index] = CORRUPTED;
    }
    for row_index in 0..N {
        memory_space[row_index][0] = CORRUPTED;
        memory_space[row_index][N - 1] = CORRUPTED;
    }

    bytes
        .iter()
        .take(bytes_to_take)
        .for_each(|(row, column)| memory_space[*row][*column] = CORRUPTED);

    let mut priority_queue: BinaryHeap<Node<N>> = BinaryHeap::from(vec![Node::<N>::new(start)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some(node) = priority_queue.pop() {
        if node.location == end {
            return Some(node.steps);
        }
        visited.insert(node.location);

        let neighbor_locations = [
            (node.location.0 - 1, node.location.1),
            (node.location.0 + 1, node.location.1),
            (node.location.0, node.location.1 - 1),
            (node.location.0, node.location.1 + 1),
        ];
        for neighbor_location in neighbor_locations {
            if visited.contains(&neighbor_location)
                || (memory_space[neighbor_location.0][neighbor_location.1] == CORRUPTED)
            {
                continue;
            }

            let neighbor = Node::<N> {
                location: neighbor_location,
                steps: node.steps + 1,
            };
            priority_queue.push(neighbor);
        }
    }

    None
}

fn process_part2<const N: usize>(bytes: &[(usize, usize)]) -> String {
    let mut left: usize = 0;
    let mut right: usize = bytes.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        let mid_result = process_part1::<N>(bytes, mid);

        if mid_result == None {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    let result_byte = bytes[left];
    format!("{},{}", result_byte.1 - 1, result_byte.0 - 1)
}

fn main() {
    let input_file_path = "input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);

    let result_part1 = process_part1::<73>(&input, 1024);
    if let Some(result) = result_part1 {
        println!("result part1: {result}");
    } else {
        println!("result part1: none");
    }

    let result_part2 = process_part2::<73>(&input);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part1 = process_part1::<9>(&input, 12);
        assert_eq!(result_part1, Some(22));
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2::<9>(&input);
        assert_eq!(result_part2, "6,1");
    }
}
