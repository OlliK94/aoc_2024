const OBSTRUCTION: char = '#';
const FREE: char = '.';
const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct GuardedArea {
    area: Vec<Vec<char>>,
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl GuardedArea {
    pub fn is_guard_leaving(&self) -> bool {
        if (self.guard_position.0 == 0) && (self.guard_direction == Direction::Up) {
            true
        } else if (self.guard_position.0 == (self.area.len() - 1))
            && (self.guard_direction == Direction::Down)
        {
            true
        } else if (self.guard_position.1 == 0) && (self.guard_direction == Direction::Left) {
            true
        } else if (self.guard_position.1 == (self.area[0].len() - 1))
            && (self.guard_direction == Direction::Right)
        {
            true
        } else {
            false
        }
    }

    fn turn_guard_right(&mut self) {
        match self.guard_direction {
            Direction::Up => self.guard_direction = Direction::Right,
            Direction::Down => self.guard_direction = Direction::Left,
            Direction::Left => self.guard_direction = Direction::Up,
            Direction::Right => self.guard_direction = Direction::Down,
        }
    }

    fn is_guard_path_blocked(&self) -> bool {
        if self.is_guard_leaving() {
            return false;
        }

        match self.guard_direction {
            Direction::Up => {
                self.area[self.guard_position.0 - 1][self.guard_position.1] == OBSTRUCTION
            }
            Direction::Down => {
                self.area[self.guard_position.0 + 1][self.guard_position.1] == OBSTRUCTION
            }
            Direction::Left => {
                self.area[self.guard_position.0][self.guard_position.1 - 1] == OBSTRUCTION
            }
            Direction::Right => {
                self.area[self.guard_position.0][self.guard_position.1 + 1] == OBSTRUCTION
            }
        }
    }

    pub fn move_guard(&mut self) {
        while self.is_guard_path_blocked() {
            self.turn_guard_right();
        }

        if self.is_guard_leaving() {
            return;
        }

        match self.guard_direction {
            Direction::Up => self.guard_position.0 -= 1,
            Direction::Down => self.guard_position.0 += 1,
            Direction::Left => self.guard_position.1 -= 1,
            Direction::Right => self.guard_position.1 += 1,
        }

        self.area[self.guard_position.0][self.guard_position.1] = match self.guard_direction {
            Direction::Up => UP,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
        };
    }

    pub fn has_loop(&self) -> bool {
        let mut guarded_area = self.clone();

        while !guarded_area.is_guard_leaving() {
            match guarded_area.guard_direction {
                Direction::Up => {
                    if guarded_area.area[guarded_area.guard_position.0 - 1]
                        [guarded_area.guard_position.1]
                        == UP
                    {
                        return true;
                    }
                }
                Direction::Down => {
                    if guarded_area.area[guarded_area.guard_position.0 + 1]
                        [guarded_area.guard_position.1]
                        == DOWN
                    {
                        return true;
                    }
                }
                Direction::Left => {
                    if guarded_area.area[guarded_area.guard_position.0]
                        [guarded_area.guard_position.1 - 1]
                        == LEFT
                    {
                        return true;
                    }
                }
                Direction::Right => {
                    if guarded_area.area[guarded_area.guard_position.0]
                        [guarded_area.guard_position.1 + 1]
                        == RIGHT
                    {
                        return true;
                    }
                }
            }

            guarded_area.move_guard();
        }

        false
    }
}

fn parse_input(input_text: &str) -> GuardedArea {
    let area: Vec<Vec<char>> = input_text
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut guard_position: (usize, usize) = (0, 0);
    'row_loop: for row_index in 0..area.len() {
        for column_index in 0..area[row_index].len() {
            if area[row_index][column_index] == UP {
                guard_position = (row_index, column_index);
                break 'row_loop;
            }
        }
    }

    GuardedArea {
        area,
        guard_position,
        guard_direction: Direction::Up,
    }
}

fn process_part1(mut guarded_area: GuardedArea) -> usize {
    while !guarded_area.is_guard_leaving() {
        guarded_area.move_guard();
    }

    guarded_area
        .area
        .iter()
        .map(|row| {
            row.iter()
                .filter(|place| {
                    (**place == UP) || (**place == DOWN) || (**place == LEFT) || (**place == RIGHT)
                })
                .count()
        })
        .sum()
}

fn process_part2(mut guarded_area: GuardedArea) -> usize {
    let mut count: usize = 0;

    for row_index in 0..guarded_area.area.len() {
        for column_index in 0..guarded_area.area[row_index].len() {
            if guarded_area.area[row_index][column_index] == FREE {
                guarded_area.area[row_index][column_index] = OBSTRUCTION;
                if guarded_area.has_loop() {
                    count += 1;
                }
                guarded_area.area[row_index][column_index] = FREE;
            }
        }
    }

    count
}

fn main() {
    let input_file_path = "./input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);
    let result_part1 = process_part1(input.clone());
    println!("result part1: {result_part1}");
    let result_part2 = process_part2(input);
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
        let result_part1 = process_part1(input);
        assert_eq!(result_part1, 41);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "./test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(input);
        assert_eq!(result_part2, 6);
    }
}
