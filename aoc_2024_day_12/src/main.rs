use std::{collections::HashMap, ops::AddAssign};

fn parse_input(input_text: &str) -> Vec<Vec<char>> {
    input_text
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_region_map(garden_map: &[Vec<char>]) -> Vec<Vec<u32>> {
    let num_rows = garden_map.len();
    let num_columns = garden_map[0].len();

    let mut next_region_id: u32 = 1;
    let mut region_map: Vec<Vec<u32>> = vec![vec![0; num_columns]; num_rows];
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            if region_map[row_index][column_index] != 0 {
                continue;
            }

            let plant = garden_map[row_index][column_index];
            let mut region_stack: Vec<(usize, usize)> = vec![(row_index, column_index)];
            while let Some((region_row_index, region_column_index)) = region_stack.pop() {
                region_map[region_row_index][region_column_index] = next_region_id;

                if (region_column_index > 0)
                    && (plant == garden_map[region_row_index][region_column_index - 1])
                    && (region_map[region_row_index][region_column_index - 1] == 0)
                {
                    region_stack.push((region_row_index, region_column_index - 1));
                }

                if (region_row_index > 0)
                    && (plant == garden_map[region_row_index - 1][region_column_index])
                    && (region_map[region_row_index - 1][region_column_index] == 0)
                {
                    region_stack.push((region_row_index - 1, region_column_index));
                }

                if ((region_column_index + 1) < num_columns)
                    && (plant == garden_map[region_row_index][region_column_index + 1])
                    && (region_map[region_row_index][region_column_index + 1] == 0)
                {
                    region_stack.push((region_row_index, region_column_index + 1));
                }

                if ((region_row_index + 1) < num_rows)
                    && (plant == garden_map[region_row_index + 1][region_column_index])
                    && (region_map[region_row_index + 1][region_column_index] == 0)
                {
                    region_stack.push((region_row_index + 1, region_column_index));
                }
            }
            next_region_id += 1;
        }
    }

    region_map
}

fn process_part1(garden_map: &[Vec<char>]) -> u32 {
    let num_rows = garden_map.len();
    let num_columns = garden_map[0].len();
    let region_map = get_region_map(garden_map);

    let mut area_map: HashMap<u32, u32> = HashMap::new();
    let mut perimeter_map: HashMap<u32, u32> = HashMap::new();
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            let region = region_map[row_index][column_index];
            area_map.entry(region).or_insert(0).add_assign(1);

            if (column_index == 0) || (region != region_map[row_index][column_index - 1]) {
                perimeter_map.entry(region).or_insert(0).add_assign(1);
            }

            if (row_index == 0) || (region != region_map[row_index - 1][column_index]) {
                perimeter_map.entry(region).or_insert(0).add_assign(1);
            }

            if ((column_index + 1) == num_columns)
                || (region != region_map[row_index][column_index + 1])
            {
                perimeter_map.entry(region).or_insert(0).add_assign(1);
            }

            if ((row_index + 1) == num_rows) || (region != region_map[row_index + 1][column_index])
            {
                perimeter_map.entry(region).or_insert(0).add_assign(1);
            }
        }
    }

    area_map
        .iter()
        .map(|(region, area)| area * perimeter_map.get(region).unwrap())
        .sum()
}

fn process_part2(garden_map: &[Vec<char>]) -> u32 {
    let num_rows = garden_map.len();
    let num_columns = garden_map[0].len();

    let mut garden_map_with_padding: Vec<Vec<char>> =
        vec![vec!['.'; num_columns + 2]; num_rows + 2];
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            garden_map_with_padding[row_index + 1][column_index + 1] =
                garden_map[row_index][column_index];
        }
    }

    let region_map_with_padding = get_region_map(&garden_map_with_padding);

    let mut area_map: HashMap<u32, u32> = HashMap::new();
    let mut side_map: HashMap<u32, u32> = HashMap::new();
    for row_index in 1..=num_rows {
        for column_index in 1..=num_columns {
            let region = region_map_with_padding[row_index][column_index];
            area_map.entry(region).or_insert(0).add_assign(1);

            // convex corners ┌
            if (region != region_map_with_padding[row_index][column_index - 1])
                && (region != region_map_with_padding[row_index - 1][column_index])
            {
                side_map.entry(region).or_insert(0).add_assign(2);
            }

            // convex corners ┐
            if (region != region_map_with_padding[row_index][column_index + 1])
                && (region != region_map_with_padding[row_index - 1][column_index])
            {
                side_map.entry(region).or_insert(0).add_assign(1);
            }

            // convex corners └
            if (region != region_map_with_padding[row_index][column_index - 1])
                && (region != region_map_with_padding[row_index + 1][column_index])
            {
                side_map.entry(region).or_insert(0).add_assign(1);
            }

            // concav corners ┌
            if (region != region_map_with_padding[row_index][column_index - 1])
                && (region != region_map_with_padding[row_index - 1][column_index])
                && (region_map_with_padding[row_index][column_index - 1]
                    == region_map_with_padding[row_index - 1][column_index])
                && (region_map_with_padding[row_index][column_index - 1]
                    == region_map_with_padding[row_index - 1][column_index - 1])
            {
                side_map
                    .entry(region_map_with_padding[row_index][column_index - 1])
                    .or_insert(0)
                    .add_assign(2);
            }

            // concav corners ┐
            if (region != region_map_with_padding[row_index][column_index + 1])
                && (region != region_map_with_padding[row_index - 1][column_index])
                && (region_map_with_padding[row_index][column_index + 1]
                    == region_map_with_padding[row_index - 1][column_index])
                && (region_map_with_padding[row_index][column_index + 1]
                    == region_map_with_padding[row_index - 1][column_index + 1])
            {
                side_map
                    .entry(region_map_with_padding[row_index][column_index + 1])
                    .or_insert(0)
                    .add_assign(1);
            }

            // concav corners └
            if (region != region_map_with_padding[row_index][column_index - 1])
                && (region != region_map_with_padding[row_index + 1][column_index])
                && (region_map_with_padding[row_index][column_index - 1]
                    == region_map_with_padding[row_index + 1][column_index])
                && (region_map_with_padding[row_index][column_index - 1]
                    == region_map_with_padding[row_index + 1][column_index - 1])
            {
                side_map
                    .entry(region_map_with_padding[row_index][column_index - 1])
                    .or_insert(0)
                    .add_assign(1);
            }
        }
    }

    // remove padding sides
    side_map.remove(&region_map_with_padding[0][0]);

    area_map
        .iter()
        .map(|(region, area)| area * side_map.get(region).unwrap())
        .sum()
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
        assert_eq!(result_part1, 1930);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 1206);
    }
}
