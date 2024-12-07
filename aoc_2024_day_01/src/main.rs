fn parse_input(input_text: &str) -> (Vec<u32>, Vec<u32>) {
    let list1: Vec<u32> = input_text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    let list2: Vec<u32> = input_text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    (list1, list2)
}

fn process_part1(mut list1: Vec<u32>, mut list2: Vec<u32>) -> u32 {
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(number1, number2)| {
            if number1 > number2 {
                number1 - number2
            } else {
                number2 - number1
            }
        })
        .sum()
}

fn process_part2(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    list1
        .iter()
        .map(|number1| number1 * list2.iter().filter(|number2| *number2 == number1).count() as u32)
        .sum()
}

fn main() {
    let input_file_path = "./input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let (list1, list2) = parse_input(&input_text);
    let result_part1 = process_part1(list1.clone(), list2.clone());
    println!("result part1: {result_part1}");
    let result_part2 = process_part2(list1, list2);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let (list1, list2) = parse_input(&input_text);
        let result_part1 = process_part1(list1, list2);
        assert_eq!(result_part1, 11);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let (list1, list2) = parse_input(&input_text);
        let result_part2 = process_part2(list1, list2);
        assert_eq!(result_part2, 31);
    }
}
