use std::collections::HashSet;

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const FREE: char = '.';
const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';

#[derive(Debug, Clone)]
struct Warehouse {
    floor_map: Vec<Vec<char>>,
    robot_location: (usize, usize),
    robot_movement_sequence: Vec<char>,
}

fn parse_input_part1(input_text: &str) -> Warehouse {
    let parts: Vec<&str> = input_text.split("\n\n").collect();

    let floor_map: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut robot_location: (usize, usize) = (0, 0);
    'row_loop: for row_index in 0..floor_map.len() {
        for column_index in 0..floor_map[row_index].len() {
            if floor_map[row_index][column_index] == ROBOT {
                robot_location = (row_index, column_index);
                break 'row_loop;
            }
        }
    }

    let robot_movement_sequence: Vec<char> = parts[1]
        .chars()
        .filter(|ch| matches!(*ch, UP | DOWN | LEFT | RIGHT))
        .collect();

    Warehouse {
        floor_map,
        robot_location,
        robot_movement_sequence,
    }
}

fn parse_input_part2(input_text: &str) -> Warehouse {
    let parts: Vec<&str> = input_text.split("\n\n").collect();

    let floor_map: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    WALL => [WALL, WALL],
                    BOX => [BOX_LEFT, BOX_RIGHT],
                    FREE => [FREE, FREE],
                    ROBOT => [ROBOT, FREE],
                    _ => panic!("invalid symbol"),
                })
                .flatten()
                .collect()
        })
        .collect();

    let mut robot_location: (usize, usize) = (0, 0);
    'row_loop: for row_index in 0..floor_map.len() {
        for column_index in 0..floor_map[row_index].len() {
            if floor_map[row_index][column_index] == ROBOT {
                robot_location = (row_index, column_index);
                break 'row_loop;
            }
        }
    }

    let robot_movement_sequence: Vec<char> = parts[1]
        .chars()
        .filter(|ch| matches!(*ch, UP | DOWN | LEFT | RIGHT))
        .collect();

    Warehouse {
        floor_map,
        robot_location,
        robot_movement_sequence,
    }
}

fn process_part1(mut warehouse: Warehouse) -> usize {
    for direction in warehouse.robot_movement_sequence {
        let mut shift_location = warehouse.robot_location.clone();

        while (warehouse.floor_map[shift_location.0][shift_location.1] != FREE)
            && (warehouse.floor_map[shift_location.0][shift_location.1] != WALL)
        {
            match direction {
                UP => shift_location.0 -= 1,
                DOWN => shift_location.0 += 1,
                LEFT => shift_location.1 -= 1,
                RIGHT => shift_location.1 += 1,
                _ => panic!("invalid direction"),
            }
        }

        if warehouse.floor_map[shift_location.0][shift_location.1] == FREE {
            while shift_location != warehouse.robot_location {
                let swap_location = match direction {
                    UP => (shift_location.0 + 1, shift_location.1),
                    DOWN => (shift_location.0 - 1, shift_location.1),
                    LEFT => (shift_location.0, shift_location.1 + 1),
                    RIGHT => (shift_location.0, shift_location.1 - 1),
                    _ => panic!("invalid direction"),
                };

                let tmp = warehouse.floor_map[swap_location.0][swap_location.1];
                warehouse.floor_map[swap_location.0][swap_location.1] =
                    warehouse.floor_map[shift_location.0][shift_location.1];
                warehouse.floor_map[shift_location.0][shift_location.1] = tmp;

                shift_location = swap_location;
            }

            match direction {
                UP => warehouse.robot_location.0 -= 1,
                DOWN => warehouse.robot_location.0 += 1,
                LEFT => warehouse.robot_location.1 -= 1,
                RIGHT => warehouse.robot_location.1 += 1,
                _ => panic!("invalid direction"),
            }
        }
    }

    warehouse
        .floor_map
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, warehouse_location)| **warehouse_location == BOX)
                .map(|(column_index, _)| column_index + (row_index * 100))
                .sum::<usize>()
        })
        .sum()
}

fn process_part2(mut warehouse: Warehouse) -> usize {
    for direction in warehouse.robot_movement_sequence {
        if matches!(direction, LEFT | RIGHT) {
            let mut shift_location = warehouse.robot_location.clone();

            while (warehouse.floor_map[shift_location.0][shift_location.1] != FREE)
                && (warehouse.floor_map[shift_location.0][shift_location.1] != WALL)
            {
                match direction {
                    LEFT => shift_location.1 -= 1,
                    RIGHT => shift_location.1 += 1,
                    _ => panic!("invalid direction"),
                }
            }

            if warehouse.floor_map[shift_location.0][shift_location.1] == FREE {
                while shift_location != warehouse.robot_location {
                    let swap_location = match direction {
                        LEFT => (shift_location.0, shift_location.1 + 1),
                        RIGHT => (shift_location.0, shift_location.1 - 1),
                        _ => panic!("invalid direction"),
                    };

                    let tmp = warehouse.floor_map[swap_location.0][swap_location.1];
                    warehouse.floor_map[swap_location.0][swap_location.1] =
                        warehouse.floor_map[shift_location.0][shift_location.1];
                    warehouse.floor_map[shift_location.0][shift_location.1] = tmp;

                    shift_location = swap_location;
                }

                match direction {
                    LEFT => warehouse.robot_location.1 -= 1,
                    RIGHT => warehouse.robot_location.1 += 1,
                    _ => panic!("invalid direction"),
                }
            }
        } else {
            let mut shift_row = warehouse.robot_location.0;
            let mut is_blocked = false;
            let mut column_stack = vec![HashSet::from([warehouse.robot_location.1])];
            loop {
                let src_row = shift_row;
                match direction {
                    UP => shift_row -= 1,
                    DOWN => shift_row += 1,
                    _ => panic!("invalid direction"),
                }

                let mut shift_columns = HashSet::new();
                for src_column in column_stack.last().unwrap() {
                    if matches!(
                        warehouse.floor_map[src_row][*src_column],
                        ROBOT | BOX_LEFT | BOX_RIGHT
                    ) {
                        if warehouse.floor_map[shift_row][*src_column] == BOX_LEFT {
                            shift_columns.insert(*src_column);
                            shift_columns.insert(src_column + 1);
                        } else if warehouse.floor_map[shift_row][*src_column] == BOX_RIGHT {
                            shift_columns.insert(*src_column);
                            shift_columns.insert(src_column - 1);
                        } else if warehouse.floor_map[shift_row][*src_column] == WALL {
                            is_blocked = true;
                        }
                    }
                }

                let mut is_free = true;
                for shift_column in &shift_columns {
                    if matches!(
                        warehouse.floor_map[shift_row][*shift_column],
                        BOX_LEFT | BOX_RIGHT | WALL
                    ) {
                        is_free = false;
                        break;
                    }
                }
                if is_free {
                    break;
                }

                column_stack.push(shift_columns);
            }

            if !is_blocked {
                while let Some(shift_columns) = column_stack.pop() {
                    let swap_row = match direction {
                        UP => shift_row + 1,
                        DOWN => shift_row - 1,
                        _ => panic!("invalid direction"),
                    };

                    for shift_column in shift_columns {
                        let tmp = warehouse.floor_map[swap_row][shift_column];
                        warehouse.floor_map[swap_row][shift_column] =
                            warehouse.floor_map[shift_row][shift_column];
                        warehouse.floor_map[shift_row][shift_column] = tmp;
                    }

                    shift_row = swap_row;
                }

                match direction {
                    UP => warehouse.robot_location.0 -= 1,
                    DOWN => warehouse.robot_location.0 += 1,
                    _ => panic!("invalid direction"),
                }
            }
        }
    }

    warehouse
        .floor_map
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, warehouse_location)| **warehouse_location == BOX_LEFT)
                .map(|(column_index, _)| column_index + (row_index * 100))
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input_file_path = "input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();

    let input_part1 = parse_input_part1(&input_text);
    let result_part1 = process_part1(input_part1);
    println!("result part1: {result_part1}");

    let input_part2 = parse_input_part2(&input_text);
    let result_part2 = process_part2(input_part2);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("test_input_small.txt" => 2028)]
    #[test_case("test_input_big.txt" => 10092)]
    fn test_process_part1(input_file_path: &str) -> usize {
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input_part1(&input_text);
        process_part1(input)
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input_big.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input_part2(&input_text);
        let result_part2 = process_part2(input);
        assert_eq!(result_part2, 9021);
    }
}
