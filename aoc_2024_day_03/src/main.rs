fn process_part1(input: &str) -> u32 {
    let mut sum: u32 = 0;

    let parts: Vec<&str> = input.split("mul(").skip(1).collect();
    for part in parts {
        let maybe_arg: Option<&str> = part.split(')').next();
        if let Some(arg) = maybe_arg {
            let numbers: Vec<&str> = arg.split(',').collect();
            if numbers.len() == 2 {
                let maybe_number1 = numbers[0].parse::<u32>();
                let maybe_number2 = numbers[1].parse::<u32>();
                if let (Ok(number1), Ok(number2)) = (maybe_number1, maybe_number2) {
                    sum += number1 * number2;
                }
            }
        }
    }

    sum
}

fn process_part2(input: &str) -> u32 {
    let mut sum: u32 = 0;

    let do_parts: Vec<&str> = input.split("do()").collect();
    for do_part in do_parts {
        let dont_parts: Vec<&str> = do_part.split("don't()").collect();
        if !dont_parts.is_empty() {
            sum += process_part1(dont_parts[0]);
        }
    }

    sum
}

fn main() {
    let input_file_path = "./input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let result_part1 = process_part1(&input_text);
    println!("result part1: {result_part1}");
    let result_part2 = process_part2(&input_text);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input_text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result_part1 = process_part1(&input_text);
        assert_eq!(result_part1, 161);
    }

    #[test]
    fn test_process_part2() {
        let input_text =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result_part2 = process_part2(&input_text);
        assert_eq!(result_part2, 48);
    }
}
