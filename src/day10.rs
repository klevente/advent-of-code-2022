use advent_of_code_2022::{print_2d_array, read_file_lines_as};
use array2d::Array2D;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if let Some((cmd, x)) = s.split_once(' ') {
            if cmd == "addx" {
                let x = i32::from_str(x).map_err(|_| "Invalid format".to_string())?;
                Ok(Self::AddX(x))
            } else {
                Err("Invalid format".to_string())
            }
        } else {
            Err("Invalid format".to_string())
        }
    }
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

struct Crt {
    x: i32,
    elapsed_ticks: usize,
    display: Array2D<char>,
}

impl Crt {
    fn new() -> Self {
        Self {
            x: 1,
            elapsed_ticks: 0,
            display: Array2D::filled_with('.', CRT_HEIGHT, CRT_WIDTH),
        }
    }

    fn run_program(&mut self, instructions: &[Instruction]) -> i32 {
        let mut sum_of_interesting_signal_strengths = 0;

        let mut instruction_iter = instructions.iter();
        let Some(mut current_instruction) = instruction_iter.next() else {
            return sum_of_interesting_signal_strengths;
        };
        let mut remaining_ticks = current_instruction.cycles();

        loop {
            self.elapsed_ticks += 1;

            let row = (self.elapsed_ticks - 1) / CRT_WIDTH;
            let column = (self.elapsed_ticks - 1) % CRT_WIDTH;
            if self.x == column as i32 || self.x - 1 == column as i32 || self.x + 1 == column as i32
            {
                self.display.set(row, column, '#').unwrap();
            }

            if self.should_check_signal_strength() {
                let signal_strength = self.calculate_signal_strength_at_current_state();
                sum_of_interesting_signal_strengths += signal_strength;
            }

            if remaining_ticks == 0 {
                let Some(next_instruction) = instruction_iter.next() else {
                    return sum_of_interesting_signal_strengths;
                };
                current_instruction = next_instruction;
                remaining_ticks = current_instruction.cycles();
            }

            match current_instruction {
                Instruction::Noop => {}
                Instruction::AddX(x) => {
                    if remaining_ticks == 1 {
                        self.x += x;
                    }
                }
            }

            remaining_ticks -= 1;
        }
    }

    fn should_check_signal_strength(&self) -> bool {
        self.elapsed_ticks % 40 == 20
    }

    fn calculate_signal_strength_at_current_state(&self) -> i32 {
        self.x * self.elapsed_ticks as i32
    }
}

fn main() {
    let instructions = read_file_lines_as("input/day10.txt", |l| Instruction::from_str(l).unwrap());

    let mut crt = Crt::new();
    let sum_of_interesting_signal_strengths = crt.run_program(&instructions);

    println!(
        "The sum of the interesting signal strengths is {sum_of_interesting_signal_strengths}"
    );

    println!("The CRT rendered the following image:");
    print_2d_array(&crt.display);
}
