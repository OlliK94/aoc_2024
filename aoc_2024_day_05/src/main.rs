use std::{cmp::Ordering, collections::HashSet};

struct SafetyProtocol {
    ordering_rules: HashSet<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

fn parse_input(input_text: &str) -> SafetyProtocol {
    let parts: Vec<&str> = input_text.split("\n\n").collect();

    let ordering_rules: HashSet<(u32, u32)> = parts[0]
        .lines()
        .map(|line| {
            let numbers: Vec<&str> = line.split('|').collect();
            (
                numbers[0].parse::<u32>().unwrap(),
                numbers[1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    let pages: Vec<Vec<u32>> = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    SafetyProtocol {
        ordering_rules,
        pages,
    }
}

fn is_ordered(pages: &[u32], ordering_rules: &HashSet<(u32, u32)>) -> bool {
    let mut ordered = true;
    'outer_page_loop: for page_index_1 in 1..pages.len() {
        for page_index_0 in 0..page_index_1 {
            if ordering_rules.contains(&(pages[page_index_1], pages[page_index_0])) {
                ordered = false;
                break 'outer_page_loop;
            }
        }
    }

    ordered
}

fn process_part1(safety_protocol: &SafetyProtocol) -> u32 {
    safety_protocol
        .pages
        .iter()
        .filter(|pages| is_ordered(pages, &safety_protocol.ordering_rules))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn process_part2(safety_protocol: &SafetyProtocol) -> u32 {
    let mut unordered_pages: Vec<Vec<u32>> = safety_protocol
        .pages
        .iter()
        .filter(|pages| !is_ordered(pages, &safety_protocol.ordering_rules))
        .map(|pages| pages.clone())
        .collect();

    unordered_pages.iter_mut().for_each(|pages| {
        pages.sort_by(|page0, page1| {
            if safety_protocol.ordering_rules.contains(&(*page0, *page1)) {
                Ordering::Less
            } else if safety_protocol.ordering_rules.contains(&(*page1, *page0)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    });

    unordered_pages
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn main() {
    let input_file_path = "./input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);
    let result_part1 = process_part1(&input);
    println!("result part1: {result_part1}");
    let result_part2 = process_part2(&input);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part1 = process_part1(&input);
        assert_eq!(result_part1, 143);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 123);
    }
}
