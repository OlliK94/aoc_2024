#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskSpace {
    DataBlock(u64),
    FreeSpace,
}

fn parse_input(input_text: &str) -> Vec<DiskSpace> {
    let mut disk_map: Vec<DiskSpace> = Vec::with_capacity(input_text.chars().count());

    let mut next_file_id: u64 = 0;
    let mut is_data_block = true;
    for block in input_text.chars() {
        let block_size = block.to_digit(10).unwrap();
        if is_data_block {
            for _ in 0..block_size {
                disk_map.push(DiskSpace::DataBlock(next_file_id));
            }
            next_file_id += 1;
            is_data_block = false;
        } else {
            for _ in 0..block_size {
                disk_map.push(DiskSpace::FreeSpace);
            }
            is_data_block = true;
        }
    }

    disk_map
}

fn process_part1(mut disk_map: Vec<DiskSpace>) -> u64 {
    let mut left_index: usize = 0;
    let mut right_index = disk_map.len() - 1;
    while right_index > left_index {
        if disk_map[right_index] == DiskSpace::FreeSpace {
            right_index -= 1;
        } else if matches!(disk_map[left_index], DiskSpace::DataBlock(_)) {
            left_index += 1;
        } else {
            // left_index == FreeSpace, right_index == DataBlock
            disk_map.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }
    }

    let mut checksum: u64 = 0;
    for position in 0..disk_map.len() {
        if let DiskSpace::DataBlock(file_id) = disk_map[position] {
            checksum += position as u64 * file_id;
        } else {
            break;
        }
    }

    checksum
}

#[derive(Debug, Clone, Copy)]
struct File {
    file_id: u64,
    location: usize,
    size: usize,
}

fn find_free_disk_space(disk_map: &[DiskSpace], size: usize) -> Option<usize> {
    let mut current_size: usize = 0;
    for block_index in 0..disk_map.len() {
        if disk_map[block_index] == DiskSpace::FreeSpace {
            current_size += 1;
            if current_size >= size {
                return Some(block_index - (current_size - 1));
            }
        } else {
            current_size = 0;
        }
    }

    None
}

fn process_part2(mut disk_map: Vec<DiskSpace>) -> u64 {
    // add padding
    disk_map.push(DiskSpace::FreeSpace);

    let mut files: Vec<File> = Vec::new();
    let mut current_file: Option<File> = None;
    for block_index in 0..disk_map.len() {
        if let DiskSpace::DataBlock(file_id) = disk_map[block_index] {
            if let Some(file) = &mut current_file {
                if file_id == file.file_id {
                    file.size += 1;
                } else {
                    files.push(*file);
                    current_file = Some(File {
                        file_id,
                        location: block_index,
                        size: 1,
                    });
                }
            } else {
                current_file = Some(File {
                    file_id,
                    location: block_index,
                    size: 1,
                });
            }
        } else if let Some(file) = current_file {
            files.push(file);
            current_file = None;
        }
    }

    for file_index in (0..files.len()).rev() {
        let free_disk_location = find_free_disk_space(&disk_map, files[file_index].size);
        if let Some(location) = free_disk_location {
            if location < files[file_index].location {
                for block_index in 0..files[file_index].size {
                    disk_map.swap(
                        location + block_index,
                        files[file_index].location + block_index,
                    );
                }
            }
        }
    }

    let mut checksum: u64 = 0;
    for position in 0..disk_map.len() {
        if let DiskSpace::DataBlock(file_id) = disk_map[position] {
            checksum += position as u64 * file_id;
        }
    }

    checksum
}

fn main() {
    let input_file_path = "input.txt";
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
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part1 = process_part1(input);
        assert_eq!(result_part1, 1928);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part2 = process_part2(input);
        assert_eq!(result_part2, 2858);
    }
}
