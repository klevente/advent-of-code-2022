use advent_of_code_2022::read_file_lines_as;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Monkey {
    Op {
        operation: Operation,
        lhs: String,
        rhs: String,
    },
    Const(i64),
}

impl Monkey {
    fn parse(s: &str) -> (String, Self) {
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

    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Op {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = monkeys.get(lhs).unwrap().calculate(monkeys);
                let rhs = monkeys.get(rhs).unwrap().calculate(monkeys);
                match operation {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                }
            }
            Monkey::Const(n) => *n,
        }
    }
}

fn main() {
    let input = read_file_lines_as("input/day21.txt", Monkey::parse)
        .into_iter()
        .collect::<HashMap<_, _>>();

    let root = input.get("root").unwrap();
    let result = root.calculate(&input);
    dbg!(result);
}
