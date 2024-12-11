use std::{collections::HashMap, ops::AddAssign};

fn parse_input(input_text: &str) -> Vec<u64> {
    input_text
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect()
}

fn blink(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else {
        let num_digits = stone.ilog10() + 1;
        if (num_digits % 2) == 0 {
            let stone_string = stone.to_string();
            let split_stones = stone_string.split_at(num_digits as usize / 2);
            (
                split_stones.0.parse::<u64>().unwrap(),
                Some(split_stones.1.parse::<u64>().unwrap()),
            )
        } else {
            (stone * 2024, None)
        }
    }
}

fn process(stones: &[u64], num_blinks: usize) -> u64 {
    let mut stone_counter: HashMap<u64, u64> = stones.iter().map(|stone| (*stone, 1u64)).collect();

    for _ in 0..num_blinks {
        let mut new_stones: HashMap<u64, u64> = HashMap::with_capacity(stone_counter.len() * 2);

        for (stone, amount) in stone_counter {
            let (new_stone_1, maybe_new_stone_2) = blink(stone);
            new_stones
                .entry(new_stone_1)
                .or_insert(0)
                .add_assign(amount);
            if let Some(new_stone_2) = maybe_new_stone_2 {
                new_stones
                    .entry(new_stone_2)
                    .or_insert(0)
                    .add_assign(amount);
            }
        }

        stone_counter = new_stones;
    }

    stone_counter.values().sum()
}

fn main() {
    let input_file_path = "input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);
    let result_part1 = process(&input, 25);
    println!("result part1: {result_part1}");
    let result_part2 = process(&input, 75);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result = process(&input, 25);
        assert_eq!(result, 55312);
    }
}
