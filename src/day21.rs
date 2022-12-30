use advent_of_code_2022::read_file_lines;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug)]
enum Monkey {
    Op {
        operation: Operation,
        lhs: String,
        rhs: String,
    },
    Const(i64),
    Human,
}

impl Monkey {
    fn parse_for_first_part(s: &str) -> (String, Self) {
        let (name, expression) = s.split_once(": ").unwrap();
        let (operation, lhs, rhs) = if let Some((lhs, rhs)) = expression.split_once(" + ") {
            (Operation::Add, lhs.to_string(), rhs.to_string())
        } else if let Some((lhs, rhs)) = expression.split_once(" - ") {
            (Operation::Sub, lhs.to_string(), rhs.to_string())
        } else if let Some((lhs, rhs)) = expression.split_once(" * ") {
            (Operation::Mul, lhs.to_string(), rhs.to_string())
        } else if let Some((lhs, rhs)) = expression.split_once(" / ") {
            (Operation::Div, lhs.to_string(), rhs.to_string())
        } else {
            let n = expression.parse().unwrap();
            return (name.to_string(), Self::Const(n));
        };
        (
            name.to_string(),
            Self::Op {
                operation,
                lhs,
                rhs,
            },
        )
    }

    fn calculate_value_first_part(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Op {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = monkeys
                    .get(lhs)
                    .unwrap()
                    .calculate_value_first_part(monkeys);
                let rhs = monkeys
                    .get(rhs)
                    .unwrap()
                    .calculate_value_first_part(monkeys);
                match operation {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                    _ => unreachable!(),
                }
            }
            Monkey::Const(n) => *n,
            _ => unreachable!(),
        }
    }

    fn parse_for_second_part(s: &str) -> (String, Self) {
        let (name, expression) = s.split_once(": ").unwrap();
        if name == "humn" {
            return (name.to_string(), Monkey::Human);
        }
        let (operation, lhs, rhs) = if let Some((lhs, rhs)) = expression.split_once(" + ") {
            if name == "root" {
                (Operation::Eq, lhs.to_string(), rhs.to_string())
            } else {
                (Operation::Add, lhs.to_string(), rhs.to_string())
            }
        } else if let Some((lhs, rhs)) = expression.split_once(" - ") {
            (Operation::Sub, lhs.to_string(), rhs.to_string())
        } else if let Some((lhs, rhs)) = expression.split_once(" * ") {
            (Operation::Mul, lhs.to_string(), rhs.to_string())
        } else if let Some((lhs, rhs)) = expression.split_once(" / ") {
            (Operation::Div, lhs.to_string(), rhs.to_string())
        } else {
            let n = expression.parse().unwrap();
            return (name.to_string(), Self::Const(n));
        };
        (
            name.to_string(),
            Self::Op {
                operation,
                lhs,
                rhs,
            },
        )
    }

    fn calculate_value_second_part(&self, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        match self {
            Monkey::Op {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = monkeys
                    .get(lhs)
                    .unwrap()
                    .calculate_value_second_part(monkeys);
                let rhs = monkeys
                    .get(rhs)
                    .unwrap()
                    .calculate_value_second_part(monkeys);
                match (lhs, rhs) {
                    (Some(lhs), Some(rhs)) => match operation {
                        Operation::Add => Some(lhs + rhs),
                        Operation::Sub => Some(lhs - rhs),
                        Operation::Mul => Some(lhs * rhs),
                        Operation::Div => Some(lhs / rhs),
                        Operation::Eq => Some(if lhs == rhs { 1 } else { 0 }),
                    },
                    (_, _) => None,
                }
            }
            Monkey::Const(n) => Some(*n),
            Monkey::Human => None,
        }
    }

    fn calculate_to_be_equal_to(&self, n: i64, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        match self {
            Monkey::Op {
                operation,
                lhs,
                rhs,
            } => {
                let lhs_val = monkeys
                    .get(lhs)
                    .unwrap()
                    .calculate_value_second_part(monkeys);
                let rhs_val = monkeys
                    .get(rhs)
                    .unwrap()
                    .calculate_value_second_part(monkeys);

                match (lhs_val, rhs_val) {
                    (Some(_), Some(_)) => None,
                    (Some(lhs_val), None) => {
                        let required_rhs = match operation {
                            Operation::Add => n - lhs_val,
                            Operation::Sub => lhs_val - n,
                            Operation::Mul => n / lhs_val,
                            Operation::Div => lhs_val / n,
                            Operation::Eq => {
                                if n == 1 {
                                    lhs_val
                                } else {
                                    return None;
                                }
                            }
                        };
                        monkeys
                            .get(rhs)
                            .unwrap()
                            .calculate_to_be_equal_to(required_rhs, monkeys)
                    }
                    (None, Some(rhs_val)) => {
                        let required_lhs = match operation {
                            Operation::Add => n - rhs_val,
                            Operation::Sub => n + rhs_val,
                            Operation::Mul => n / rhs_val,
                            Operation::Div => n * rhs_val,
                            Operation::Eq => {
                                if n == 1 {
                                    rhs_val
                                } else {
                                    return None;
                                }
                            }
                        };
                        monkeys
                            .get(lhs)
                            .unwrap()
                            .calculate_to_be_equal_to(required_lhs, monkeys)
                    }
                    (None, None) => None,
                }
            }
            Monkey::Const(_) => None,
            Monkey::Human => Some(n),
        }
    }
}

fn main() {
    let input = read_file_lines("input/day21.txt");
    let monkeys_first_part = input
        .iter()
        .map(|l| Monkey::parse_for_first_part(l))
        .collect::<HashMap<_, _>>();

    let root = monkeys_first_part.get("root").unwrap();
    let number_that_root_yells = root.calculate_value_first_part(&monkeys_first_part);
    println!("The number that root will yell is {number_that_root_yells}");

    let monkeys_second_part = input
        .iter()
        .map(|l| Monkey::parse_for_second_part(l))
        .collect::<HashMap<_, _>>();

    let root = monkeys_second_part.get("root").unwrap();
    let number_human_needs_to_yell = root
        .calculate_to_be_equal_to(1, &monkeys_second_part)
        .unwrap();
    println!("The number the human needs to yell is {number_human_needs_to_yell}");
}
