fn parse_input(input_text: &str) -> Vec<Vec<char>> {
    input_text
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn process_part1(input: &Vec<Vec<char>>) -> usize {
    let num_rows = input.len();
    let num_columns = input.first().unwrap().len();
    let xmas = "XMAS";
    let samx = "SAMX";

    let mut count: usize = 0;

    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            // rows
            if (column_index + 3) < num_columns {
                let mut word = String::with_capacity(4);
                word.push(input[row_index][column_index]);
                word.push(input[row_index][column_index + 1]);
                word.push(input[row_index][column_index + 2]);
                word.push(input[row_index][column_index + 3]);

                if (word == xmas) || (word == samx) {
                    count += 1;
                }
            }

            // columns
            if (row_index + 3) < num_rows {
                let mut word = String::with_capacity(4);
                word.push(input[row_index][column_index]);
                word.push(input[row_index + 1][column_index]);
                word.push(input[row_index + 2][column_index]);
                word.push(input[row_index + 3][column_index]);

                if (word == xmas) || (word == samx) {
                    count += 1;
                }
            }

            // diagonals \
            if ((row_index + 3) < num_rows) && ((column_index + 3) < num_columns) {
                let mut word = String::with_capacity(4);
                word.push(input[row_index][column_index]);
                word.push(input[row_index + 1][column_index + 1]);
                word.push(input[row_index + 2][column_index + 2]);
                word.push(input[row_index + 3][column_index + 3]);

                if (word == xmas) || (word == samx) {
                    count += 1;
                }
            }

            // diagonals /
            if (row_index >= 3) && ((column_index + 3) < num_columns) {
                let mut word = String::with_capacity(4);
                word.push(input[row_index][column_index]);
                word.push(input[row_index - 1][column_index + 1]);
                word.push(input[row_index - 2][column_index + 2]);
                word.push(input[row_index - 3][column_index + 3]);

                if (word == xmas) || (word == samx) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn process_part2(input: &Vec<Vec<char>>) -> usize {
    let num_rows = input.len();
    let num_columns = input.first().unwrap().len();

    let mut count: usize = 0;

    for row_index in 1..(num_rows - 1) {
        for column_index in 1..(num_columns - 1) {
            if input[row_index][column_index] != 'A' {
                continue;
            }

            let top_left = input[row_index - 1][column_index - 1];
            let top_right = input[row_index - 1][column_index + 1];
            let bottom_left = input[row_index + 1][column_index - 1];
            let bottom_right = input[row_index + 1][column_index + 1];

            if (top_left == 'M')
                && (bottom_right == 'S')
                && (top_right == 'M')
                && (bottom_left == 'S')
            {
                count += 1;
            } else if (top_left == 'M')
                && (bottom_right == 'S')
                && (top_right == 'S')
                && (bottom_left == 'M')
            {
                count += 1;
            } else if (top_left == 'S')
                && (bottom_right == 'M')
                && (top_right == 'M')
                && (bottom_left == 'S')
            {
                count += 1;
            } else if (top_left == 'S')
                && (bottom_right == 'M')
                && (top_right == 'S')
                && (bottom_left == 'M')
            {
                count += 1;
            }
        }
    }

    count
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
        assert_eq!(result_part1, 18);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 9);
    }
}
