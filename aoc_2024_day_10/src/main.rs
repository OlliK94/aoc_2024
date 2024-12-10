use std::collections::{HashMap, HashSet};

fn parse_input(input_text: &str) -> Vec<Vec<i8>> {
    input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Trail {
    start: (usize, usize),
    finish: (usize, usize),
}

impl Trail {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            start: (row, column),
            finish: (row, column),
        }
    }
}

fn process_part1(topographic_map: &[Vec<i8>]) -> usize {
    let num_rows = topographic_map.len();
    let num_columns = topographic_map[0].len();

    let mut trail_stack: Vec<Trail> = Vec::new();
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            if topographic_map[row_index][column_index] == 0 {
                trail_stack.push(Trail::new(row_index, column_index));
            }
        }
    }

    let mut start_finish_map: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    while let Some(trail) = trail_stack.pop() {
        let height = topographic_map[trail.finish.0][trail.finish.1];
        if height == 9 {
            start_finish_map
                .entry(trail.start)
                .or_insert(HashSet::new())
                .insert(trail.finish);
        } else {
            if (trail.finish.0 > 0)
                && ((topographic_map[trail.finish.0 - 1][trail.finish.1] - height) == 1)
            {
                let mut continue_up = trail.clone();
                continue_up.finish.0 -= 1;
                trail_stack.push(continue_up);
            }

            if (trail.finish.1 > 0)
                && ((topographic_map[trail.finish.0][trail.finish.1 - 1] - height) == 1)
            {
                let mut continue_left = trail.clone();
                continue_left.finish.1 -= 1;
                trail_stack.push(continue_left);
            }

            if ((trail.finish.0 + 1) < num_rows)
                && ((topographic_map[trail.finish.0 + 1][trail.finish.1] - height) == 1)
            {
                let mut continue_down = trail.clone();
                continue_down.finish.0 += 1;
                trail_stack.push(continue_down);
            }

            if ((trail.finish.1 + 1) < num_columns)
                && ((topographic_map[trail.finish.0][trail.finish.1 + 1] - height) == 1)
            {
                let mut continue_right = trail.clone();
                continue_right.finish.1 += 1;
                trail_stack.push(continue_right);
            }
        }
    }

    start_finish_map
        .values()
        .map(|trail_finishes| trail_finishes.len())
        .sum()
}

fn process_part2(topographic_map: &[Vec<i8>]) -> usize {
    let num_rows = topographic_map.len();
    let num_columns = topographic_map[0].len();

    let mut trail_stack: Vec<Trail> = Vec::new();
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            if topographic_map[row_index][column_index] == 0 {
                trail_stack.push(Trail::new(row_index, column_index));
            }
        }
    }

    let mut rating: usize = 0;
    while let Some(trail) = trail_stack.pop() {
        let height = topographic_map[trail.finish.0][trail.finish.1];
        if height == 9 {
            rating += 1;
        } else {
            if (trail.finish.0 > 0)
                && ((topographic_map[trail.finish.0 - 1][trail.finish.1] - height) == 1)
            {
                let mut continue_up = trail.clone();
                continue_up.finish.0 -= 1;
                trail_stack.push(continue_up);
            }

            if (trail.finish.1 > 0)
                && ((topographic_map[trail.finish.0][trail.finish.1 - 1] - height) == 1)
            {
                let mut continue_left = trail.clone();
                continue_left.finish.1 -= 1;
                trail_stack.push(continue_left);
            }

            if ((trail.finish.0 + 1) < num_rows)
                && ((topographic_map[trail.finish.0 + 1][trail.finish.1] - height) == 1)
            {
                let mut continue_down = trail.clone();
                continue_down.finish.0 += 1;
                trail_stack.push(continue_down);
            }

            if ((trail.finish.1 + 1) < num_columns)
                && ((topographic_map[trail.finish.0][trail.finish.1 + 1] - height) == 1)
            {
                let mut continue_right = trail.clone();
                continue_right.finish.1 += 1;
                trail_stack.push(continue_right);
            }
        }
    }

    rating
}

fn main() {
    let input_file_path = "input.txt";
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
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part1 = process_part1(&input);
        assert_eq!(result_part1, 36);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 81);
    }
}
