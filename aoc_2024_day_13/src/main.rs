#[derive(Debug)]
struct Vertex {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Vertex,
    button_b: Vertex,
    prize: Vertex,
}

fn parse_input(input_text: &str) -> Vec<ClawMachine> {
    let claw_machines: Vec<&str> = input_text.split("\n\n").collect();

    claw_machines
        .iter()
        .map(|claw_machine| {
            let parts: Vec<&str> = claw_machine.lines().collect();

            let button_a_parts: Vec<&str> = parts[0].split(',').collect();
            let button_a = Vertex {
                x: button_a_parts[0]
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                y: button_a_parts[1]
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            };

            let button_b_parts: Vec<&str> = parts[1].split(',').collect();
            let button_b = Vertex {
                x: button_b_parts[0]
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                y: button_b_parts[1]
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            };

            let prize_parts: Vec<&str> = parts[2].split(',').collect();
            let prize = Vertex {
                x: prize_parts[0]
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                y: prize_parts[1]
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            };

            ClawMachine {
                button_a,
                button_b,
                prize,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

impl Equation {
    pub fn mul(&mut self, factor: i64) {
        self.a *= factor;
        self.b *= factor;
        self.c *= factor;
    }

    pub fn sub(&mut self, other: &Equation) {
        self.a -= other.a;
        self.b -= other.b;
        self.c -= other.c;
    }
}

fn get_total_cost(claw_machines: &[ClawMachine], offset: i64) -> i64 {
    let mut total_cost: i64 = 0;

    for claw_machine in claw_machines {
        let mut equation_x = Equation {
            a: claw_machine.button_a.x,
            b: claw_machine.button_b.x,
            c: claw_machine.prize.x + offset,
        };
        let mut equation_y = Equation {
            a: claw_machine.button_a.y,
            b: claw_machine.button_b.y,
            c: claw_machine.prize.y + offset,
        };

        let b_x = equation_x.b;
        let b_y = equation_y.b;
        equation_x.mul(b_y);
        equation_y.mul(b_x);
        equation_y.sub(&equation_x);

        if ((equation_y.c % equation_y.a) != 0) || (equation_y.c.signum() != equation_y.a.signum())
        {
            continue;
        }
        let a = equation_y.c / equation_y.a;

        equation_x.c -= equation_x.a * a;
        equation_x.a = 0;

        if ((equation_x.c % equation_x.b) != 0) || (equation_x.c.signum() != equation_x.b.signum())
        {
            continue;
        }
        let b = equation_x.c / equation_x.b;

        total_cost += a * 3;
        total_cost += b;
    }

    total_cost
}

fn main() {
    let input_file_path = "input.txt";
    let input_text = std::fs::read_to_string(input_file_path).unwrap();
    let input = parse_input(&input_text);
    let result_part1 = get_total_cost(&input, 0);
    println!("result part1: {result_part1}");
    let result_part2 = get_total_cost(&input, 10000000000000);
    println!("result part2: {result_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_cost() {
        let input_file_path = "test_input.txt";
        let input_text = std::fs::read_to_string(input_file_path).unwrap();
        let input = parse_input(&input_text);
        let result_part1 = get_total_cost(&input, 0);
        assert_eq!(result_part1, 480);
    }
}
