use std::collections::{HashMap, HashSet};

fn parse_input(input_text: &str) -> Vec<Vec<char>> {
    input_text
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn process_part1(antenna_map: &[Vec<char>]) -> usize {
    let num_rows = antenna_map.len() as isize;
    let num_columns = antenna_map[0].len() as isize;

    let mut antenna_locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for row_index in 0..antenna_map.len() {
        for column_index in 0..antenna_map[row_index].len() {
            if antenna_map[row_index][column_index].is_alphanumeric() {
                antenna_locations
                    .entry(antenna_map[row_index][column_index])
                    .or_insert(Vec::with_capacity(4))
                    .push((row_index as isize, column_index as isize));
            }
        }
    }

    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();
    for locations in antenna_locations.values() {
        for location_index_1 in 0..(locations.len() - 1) {
            for location_index_2 in (location_index_1 + 1)..locations.len() {
                let row_delta = locations[location_index_2].0 - locations[location_index_1].0;
                let column_delta = locations[location_index_2].1 - locations[location_index_1].1;

                let antinode_location_1 = (
                    locations[location_index_1].0 - row_delta,
                    locations[location_index_1].1 - column_delta,
                );
                let antinode_location_2 = (
                    locations[location_index_2].0 + row_delta,
                    locations[location_index_2].1 + column_delta,
                );

                if (antinode_location_1.0 >= 0)
                    && (antinode_location_1.0 < num_rows)
                    && (antinode_location_1.1 >= 0)
                    && (antinode_location_1.1 < num_columns)
                {
                    antinode_locations.insert(antinode_location_1);
                }
                if (antinode_location_2.0 >= 0)
                    && (antinode_location_2.0 < num_rows)
                    && (antinode_location_2.1 >= 0)
                    && (antinode_location_2.1 < num_columns)
                {
                    antinode_locations.insert(antinode_location_2);
                }
            }
        }
    }

    antinode_locations.len()
}

fn process_part2(antenna_map: &[Vec<char>]) -> usize {
    let num_rows = antenna_map.len() as isize;
    let num_columns = antenna_map[0].len() as isize;

    let mut antenna_locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for row_index in 0..antenna_map.len() {
        for column_index in 0..antenna_map[row_index].len() {
            if antenna_map[row_index][column_index].is_alphanumeric() {
                antenna_locations
                    .entry(antenna_map[row_index][column_index])
                    .or_insert(Vec::with_capacity(4))
                    .push((row_index as isize, column_index as isize));
            }
        }
    }

    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();
    for locations in antenna_locations.values() {
        for location_index_1 in 0..(locations.len() - 1) {
            for location_index_2 in (location_index_1 + 1)..locations.len() {
                let row_delta = locations[location_index_2].0 - locations[location_index_1].0;
                let column_delta = locations[location_index_2].1 - locations[location_index_1].1;

                let mut antinode_location_1 = locations[location_index_1];
                while (antinode_location_1.0 >= 0)
                    && (antinode_location_1.0 < num_rows)
                    && (antinode_location_1.1 >= 0)
                    && (antinode_location_1.1 < num_columns)
                {
                    antinode_locations.insert(antinode_location_1);

                    antinode_location_1.0 -= row_delta;
                    antinode_location_1.1 -= column_delta;
                }

                let mut antinode_location_2 = locations[location_index_2];
                while (antinode_location_2.0 >= 0)
                    && (antinode_location_2.0 < num_rows)
                    && (antinode_location_2.1 >= 0)
                    && (antinode_location_2.1 < num_columns)
                {
                    antinode_locations.insert(antinode_location_2);

                    antinode_location_2.0 += row_delta;
                    antinode_location_2.1 += column_delta;
                }
            }
        }
    }

    antinode_locations.len()
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
        assert_eq!(result_part1, 14);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 34);
    }
}
