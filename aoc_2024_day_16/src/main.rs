use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::u32;

struct Maze {
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
}

fn parse_input(input_text: &str) -> Maze {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    input_text.lines().enumerate().for_each(|(row_index, row)| {
        row.chars()
            .enumerate()
            .for_each(|(column_index, cell)| match cell {
                'S' => start = (row_index, column_index),
                'E' => end = (row_index, column_index),
                '#' => {
                    walls.insert((row_index, column_index));
                }
                '.' => (),
                _ => unreachable!("unknown symbol"),
            })
    });

    Maze { start, end, walls }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reindeer {
    location: (usize, usize),
    orientation: Orientation,
    score: u32,
    visited: HashSet<(usize, usize)>,
}

impl Reindeer {
    pub fn new(location: (usize, usize)) -> Self {
        Self {
            location,
            orientation: Orientation::East,
            score: 0,
            visited: HashSet::new(),
        }
    }

    pub fn turn(&mut self, target_orientation: Orientation) {
        let mut num_right_turns: u32 = 0;

        while target_orientation != self.orientation {
            self.orientation = match self.orientation {
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
                Orientation::North => Orientation::East,
            };

            num_right_turns += 1;
        }

        self.score += match num_right_turns {
            0 => 0,
            1 => 1000,
            2 => 2000,
            3 => 1000,
            _ => unreachable!("turning in circles"),
        };
    }

    pub fn move_forward(&mut self) {
        match self.orientation {
            Orientation::East => self.location.1 += 1,
            Orientation::North => self.location.0 -= 1,
            Orientation::South => self.location.0 += 1,
            Orientation::West => self.location.1 -= 1,
        }

        self.score += 1;
    }

    pub fn mark_visited(&mut self) {
        self.visited.insert(self.location);
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn process_part1(maze: &Maze) -> u32 {
    let start_reindeer = Reindeer::new(maze.start);
    let mut priority_queue: BinaryHeap<Reindeer> = BinaryHeap::from([start_reindeer.clone()]);
    let mut visited: HashMap<(usize, usize, Orientation), Reindeer> = HashMap::from([(
        (
            start_reindeer.location.0,
            start_reindeer.location.1,
            start_reindeer.orientation,
        ),
        start_reindeer,
    )]);

    while let Some(reindeer) = priority_queue.pop() {
        if maze.end == reindeer.location {
            return reindeer.score;
        }

        let eastern_location = (reindeer.location.0, reindeer.location.1 + 1);
        let western_location = (reindeer.location.0, reindeer.location.1 - 1);
        let southern_location = (reindeer.location.0 + 1, reindeer.location.1);
        let northern_location = (reindeer.location.0 - 1, reindeer.location.1);

        let mut next_moves: Vec<Reindeer> = Vec::with_capacity(4);
        if !maze.walls.contains(&eastern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::East);
            next_move.move_forward();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&western_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::West);
            next_move.move_forward();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&southern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::South);
            next_move.move_forward();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&northern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::North);
            next_move.move_forward();
            next_moves.push(next_move);
        }

        for next_move in next_moves {
            if let Some(past_move) = visited.get(&(
                next_move.location.0,
                next_move.location.1,
                next_move.orientation,
            )) {
                if next_move.score < past_move.score {
                    priority_queue.push(next_move.clone());
                    visited.insert(
                        (
                            next_move.location.0,
                            next_move.location.1,
                            next_move.orientation,
                        ),
                        next_move,
                    );
                }
            } else {
                priority_queue.push(next_move.clone());
                visited.insert(
                    (
                        next_move.location.0,
                        next_move.location.1,
                        next_move.orientation,
                    ),
                    next_move,
                );
            }
        }
    }

    0
}

fn process_part2(maze: &Maze) -> usize {
    let mut start_reindeer = Reindeer::new(maze.start);
    start_reindeer.mark_visited();
    let mut priority_queue: BinaryHeap<Reindeer> = BinaryHeap::from([start_reindeer.clone()]);
    let mut visited: HashMap<(usize, usize, Orientation), Reindeer> = HashMap::from([(
        (
            start_reindeer.location.0,
            start_reindeer.location.1,
            start_reindeer.orientation,
        ),
        start_reindeer,
    )]);

    let mut min_score = u32::MAX;
    let mut min_score_reindeers: Vec<Reindeer> = Vec::new();
    while let Some(reindeer) = priority_queue.pop() {
        if reindeer.score > min_score {
            break;
        }

        if maze.end == reindeer.location {
            if reindeer.score <= min_score {
                if reindeer.score < min_score {
                    min_score = reindeer.score;
                    min_score_reindeers.clear();
                }
                min_score_reindeers.push(reindeer.clone());
            }
        }

        let eastern_location = (reindeer.location.0, reindeer.location.1 + 1);
        let western_location = (reindeer.location.0, reindeer.location.1 - 1);
        let southern_location = (reindeer.location.0 + 1, reindeer.location.1);
        let northern_location = (reindeer.location.0 - 1, reindeer.location.1);

        let mut next_moves: Vec<Reindeer> = Vec::with_capacity(4);
        if !maze.walls.contains(&eastern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::East);
            next_move.move_forward();
            next_move.mark_visited();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&western_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::West);
            next_move.move_forward();
            next_move.mark_visited();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&southern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::South);
            next_move.move_forward();
            next_move.mark_visited();
            next_moves.push(next_move);
        }
        if !maze.walls.contains(&northern_location) {
            let mut next_move = reindeer.clone();
            next_move.turn(Orientation::North);
            next_move.move_forward();
            next_move.mark_visited();
            next_moves.push(next_move);
        }

        for next_move in next_moves {
            if let Some(past_move) = visited.get(&(
                next_move.location.0,
                next_move.location.1,
                next_move.orientation,
            )) {
                if next_move.score <= past_move.score {
                    priority_queue.push(next_move.clone());
                    visited.insert(
                        (
                            next_move.location.0,
                            next_move.location.1,
                            next_move.orientation,
                        ),
                        next_move,
                    );
                }
            } else {
                priority_queue.push(next_move.clone());
                visited.insert(
                    (
                        next_move.location.0,
                        next_move.location.1,
                        next_move.orientation,
                    ),
                    next_move,
                );
            }
        }
    }

    let mut best_path_tiles: HashSet<(usize, usize)> = HashSet::new();
    for reindeer in min_score_reindeers {
        best_path_tiles.extend(reindeer.visited.iter());
    }

    best_path_tiles.len()
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
    use test_case::test_case;

    #[test_case("test_input_1.txt" => 7036)]
    #[test_case("test_input_2.txt" => 11048)]
    fn test_process_part1(input_file_path: &str) -> u32 {
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        process_part1(&input)
    }

    #[test_case("test_input_1.txt" => 45)]
    #[test_case("test_input_2.txt" => 64)]
    fn test_process_part2(input_file_path: &str) -> usize {
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        process_part2(&input)
    }
}
