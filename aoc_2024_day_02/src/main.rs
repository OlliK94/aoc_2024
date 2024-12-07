fn parse_input(input_text: &str) -> Vec<Vec<u32>> {
    input_text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[u32]) -> bool {
    let is_increasing = report[0] < report[1];
    if is_increasing {
        report
            .windows(2)
            .all(|window| (window[0] < window[1]) && ((window[1] - window[0]) <= 3))
    } else {
        report
            .windows(2)
            .all(|window| (window[1] < window[0]) && ((window[0] - window[1]) <= 3))
    }
}

fn process_part1(data: &[Vec<u32>]) -> u32 {
    data.iter().filter(|report| is_safe(report)).count() as u32
}

fn is_safe_with_tolerance(report: &[u32]) -> bool {
    for remove_index in 0..report.len() {
        let mut report_dampened: Vec<u32> = report.into();
        report_dampened.remove(remove_index);
        if is_safe(&report_dampened) {
            return true;
        }
    }

    false
}

fn process_part2(data: &[Vec<u32>]) -> u32 {
    data.iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count() as u32
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
        assert_eq!(result_part1, 2);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 4);
    }
}
