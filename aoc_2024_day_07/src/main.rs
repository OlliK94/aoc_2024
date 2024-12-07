struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl CalibrationEquation {
    pub fn is_true_part1(&self) -> bool {
        let mut stack: Vec<(u64, &[u64])> = Vec::with_capacity(self.numbers.len() * 2);
        stack.push((self.test_value, &self.numbers));

        while let Some((test_value, numbers)) = stack.pop() {
            if let Some(last_number) = numbers.last() {
                if numbers.len() > 1 {
                    if *last_number <= test_value {
                        stack.push((test_value - last_number, &numbers[0..(numbers.len() - 1)]));
                    }

                    if (test_value % last_number) == 0 {
                        stack.push((test_value / last_number, &numbers[0..(numbers.len() - 1)]));
                    }
                } else if test_value == *last_number {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_true_part2(&self) -> bool {
        let mut stack: Vec<(u64, &[u64])> = Vec::with_capacity(self.numbers.len() * 3);
        stack.push((self.test_value, &self.numbers));

        while let Some((test_value, numbers)) = stack.pop() {
            if let Some(last_number) = numbers.last() {
                if numbers.len() > 1 {
                    if *last_number <= test_value {
                        stack.push((test_value - last_number, &numbers[0..(numbers.len() - 1)]));
                    }

                    if (test_value % last_number) == 0 {
                        stack.push((test_value / last_number, &numbers[0..(numbers.len() - 1)]));
                    }

                    let mut test_value_string = test_value.to_string();
                    let last_number_string = last_number.to_string();
                    if (test_value_string.len() > last_number_string.len())
                        && test_value_string.ends_with(&last_number_string)
                    {
                        for _ in 0..last_number_string.len() {
                            test_value_string.pop();
                        }
                        stack.push((
                            test_value_string.parse::<u64>().unwrap(),
                            &numbers[0..(numbers.len() - 1)],
                        ));
                    }
                } else if test_value == *last_number {
                    return true;
                }
            }
        }

        false
    }
}

fn parse_input(input_text: &str) -> Vec<CalibrationEquation> {
    input_text
        .lines()
        .map(|line| {
            let test_value = line.split(':').next().unwrap().parse::<u64>().unwrap();

            let numbers: Vec<u64> = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect();

            CalibrationEquation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn process_part1(equations: &[CalibrationEquation]) -> u64 {
    equations
        .iter()
        .filter(|equation| equation.is_true_part1())
        .map(|equation| equation.test_value)
        .sum()
}

fn process_part2(equations: &[CalibrationEquation]) -> u64 {
    equations
        .iter()
        .filter(|equation| equation.is_true_part2())
        .map(|equation| equation.test_value)
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
        assert_eq!(result_part1, 3749);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 11387);
    }
}
