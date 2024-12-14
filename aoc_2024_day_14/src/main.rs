#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    pub fn move_once(&mut self, num_rows: i32, num_columns: i32) {
        self.position.0 = (num_columns + self.position.0 + self.velocity.0) % num_columns;
        self.position.1 = (num_rows + self.position.1 + self.velocity.1) % num_rows;
    }
}

fn parse_input(input_text: &str) -> Vec<Robot> {
    input_text
        .lines()
        .map(|line| {
            let position_numbers: Vec<&str> = line
                .split_whitespace()
                .next()
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .split(',')
                .collect();
            let position_x = position_numbers[0].parse::<i32>().unwrap();
            let position_y = position_numbers[1].parse::<i32>().unwrap();

            let velocity_numbers: Vec<&str> = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .split(',')
                .collect();
            let velocity_x = velocity_numbers[0].parse::<i32>().unwrap();
            let velocity_y = velocity_numbers[1].parse::<i32>().unwrap();

            Robot {
                position: (position_x, position_y),
                velocity: (velocity_x, velocity_y),
            }
        })
        .collect()
}

fn process_part1(mut robots: Vec<Robot>, num_rows: i32, num_columns: i32) -> i32 {
    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.move_once(num_rows, num_columns));
    }

    let mut q1_count: i32 = 0;
    let mut q2_count: i32 = 0;
    let mut q3_count: i32 = 0;
    let mut q4_count: i32 = 0;
    let mid_x = num_columns / 2;
    let mid_y = num_rows / 2;
    for robot in robots {
        if (robot.position.0 < mid_x) && (robot.position.1 < mid_y) {
            q1_count += 1;
        } else if (robot.position.0 > mid_x) && (robot.position.1 < mid_y) {
            q2_count += 1;
        } else if (robot.position.0 > mid_x) && (robot.position.1 > mid_y) {
            q3_count += 1;
        } else if (robot.position.0 < mid_x) && (robot.position.1 > mid_y) {
            q4_count += 1;
        }
    }

    q1_count * q2_count * q3_count * q4_count
}

fn process_part2(mut robots: Vec<Robot>, num_rows: usize, num_columns: usize) -> i32 {
    let mut time: i32 = 0;

    let mut grid: Vec<Vec<u8>> = vec![vec![0; num_columns]; num_rows];
    'time_loop: loop {
        // clear grid
        robots
            .iter_mut()
            .for_each(|robot| grid[robot.position.1 as usize][robot.position.0 as usize] = 0);

        robots.iter_mut().for_each(|robot| {
            robot.move_once(num_rows as i32, num_columns as i32);
            grid[robot.position.1 as usize][robot.position.0 as usize] = 1;
        });
        time += 1;

        let mut counter: usize = 0;
        for row_index in 0..num_rows {
            for column_index in 0..num_columns {
                if grid[row_index][column_index] != 0 {
                    counter += 1;
                } else {
                    counter = 0;
                }

                if counter >= 8 {
                    break 'time_loop;
                }
            }
        }
    }

    time
}

fn main() {
    let input_file_path = "input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);
    let result_part1 = process_part1(input.clone(), 103, 101);
    println!("result part1: {result_part1}");
    let result_part2 = process_part2(input, 103, 101);
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
        let result_part1 = process_part1(input, 7, 11);
        assert_eq!(result_part1, 12);
    }
}
