use std::collections::HashSet;

struct OnsenBranding {
    towels: HashSet<String>,
    designs: Vec<String>,
}

fn parse_input(input_text: &str) -> OnsenBranding {
    let towels: HashSet<String> = input_text
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(", ").map(|towel| towel.to_string()))
        .flatten()
        .collect();

    let designs: Vec<String> = input_text
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect();

    OnsenBranding { towels, designs }
}

fn process_part1(onsen_branding: &OnsenBranding) -> u32 {
    let mut count: u32 = 0;

    'design_loop: for design in &onsen_branding.designs {
        let mut is_start = vec![false; design.len() + 1];
        is_start[0] = true;

        for start in 0..design.len() {
            if !is_start[start] {
                continue;
            }

            for end in start..design.len() {
                if onsen_branding.towels.contains(&design[start..=end]) {
                    is_start[end + 1] = true;

                    if (end + 1) == design.len() {
                        count += 1;
                        continue 'design_loop;
                    }
                }
            }
        }
    }

    count
}

fn process_part2(onsen_branding: &OnsenBranding) -> u64 {
    let mut count: u64 = 0;

    for design in &onsen_branding.designs {
        let mut num_ways = vec![0u64; design.len() + 1];
        num_ways[0] = 1;

        for start in 0..design.len() {
            if num_ways[start] == 0 {
                continue;
            }

            for end in start..design.len() {
                if onsen_branding.towels.contains(&design[start..=end]) {
                    num_ways[end + 1] += num_ways[start];
                }
            }
        }

        count += num_ways.last().unwrap();
    }

    count
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
        assert_eq!(result_part1, 6);
    }

    #[test]
    fn test_process_part2() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);

        let result_part2 = process_part2(&input);
        assert_eq!(result_part2, 16);
    }
}
