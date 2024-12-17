use std::u64;

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
    instruction_pointer: usize,
}

#[derive(Debug, PartialEq)]
enum ComputerError {
    InvalidOperand,
    InvalidOpcode,
}

impl Computer {
    fn map_combo_operand(&self, operand: u8) -> Result<u64, ComputerError> {
        match operand {
            0..=3 => Ok(operand as u64),
            4 => Ok(self.register_a),
            5 => Ok(self.register_b),
            6 => Ok(self.register_c),
            _ => Err(ComputerError::InvalidOperand),
        }
    }

    pub fn run_program(&mut self) -> Result<String, ComputerError> {
        let mut output = String::new();

        while (self.instruction_pointer + 1) < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            match opcode {
                0 => {
                    self.register_a >>= self.map_combo_operand(operand)?;
                    self.instruction_pointer += 2;
                }
                1 => {
                    self.register_b ^= operand as u64;
                    self.instruction_pointer += 2;
                }
                2 => {
                    self.register_b = self.map_combo_operand(operand)? & 7;
                    self.instruction_pointer += 2;
                }
                3 => {
                    if self.register_a != 0 {
                        self.instruction_pointer = operand as usize
                    } else {
                        self.instruction_pointer += 2;
                    }
                }
                4 => {
                    self.register_b ^= self.register_c;
                    self.instruction_pointer += 2;
                }
                5 => {
                    let out = self.map_combo_operand(operand)? & 7;
                    output.push_str(&out.to_string());
                    output.push(',');
                    self.instruction_pointer += 2;
                }
                6 => {
                    self.register_b = self.register_a >> self.map_combo_operand(operand)?;
                    self.instruction_pointer += 2;
                }
                7 => {
                    self.register_c = self.register_a >> self.map_combo_operand(operand)?;
                    self.instruction_pointer += 2;
                }
                _ => return Err(ComputerError::InvalidOpcode),
            }
        }

        output.pop();
        Ok(output)
    }
}

fn process_part1(mut computer: Computer) -> Result<String, ComputerError> {
    computer.run_program()
}

fn process_part2(computer: Computer, output: &str) -> u64 {
    let mut stack: Vec<u64> = vec![0];
    let mut min_result = u64::MAX;
    while let Some(a_init) = stack.pop() {
        for a_value in 0..8 {
            let mut computer_clone = computer.clone();
            let next_a_init = (a_init << 3) + a_value;
            computer_clone.register_a = next_a_init;
            let out = computer_clone.run_program();
            if let Ok(out) = out {
                if (out == output) && (next_a_init < min_result) {
                    min_result = next_a_init;
                } else if output.ends_with(&out) {
                    stack.push(next_a_init);
                }
            }
        }
    }

    min_result
}

fn main() {
    /*
    b = a & 7
    b = b ^ 2
    c = a >> b
    b = b ^ c
    b = b ^ 3
    out(b & 7)
    a = a >> 3
    jnz 0
     */
    let computer = Computer {
        register_a: 35200350,
        register_b: 0,
        register_c: 0,
        program: vec![2, 4, 1, 2, 7, 5, 4, 7, 1, 3, 5, 5, 0, 3, 3, 0],
        instruction_pointer: 0,
    };

    let result_part1 = process_part1(computer.clone());
    if let Ok(result) = result_part1 {
        println!("result part1: {result}");
    } else {
        println!("result part1: error");
    }

    let output = "2,4,1,2,7,5,4,7,1,3,5,5,0,3,3,0";
    let result_part2 = process_part2(computer, output);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let computer = Computer {
            register_a: 729,
            register_b: 0,
            register_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            instruction_pointer: 0,
        };
        let result_part1 = process_part1(computer);
        assert_eq!(result_part1, Ok("4,6,3,5,6,3,5,2,1,0".to_string()));
    }
}
