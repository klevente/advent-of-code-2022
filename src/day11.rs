use advent_of_code_2022::{read_file_to_string, EMPTY_LINE_PATTERN};
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Operation {
    Squared,
    Plus(u64),
    Times(u64),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operand, rhs) = s
            .trim()
            .split_once(' ')
            .ok_or("Invalid format".to_string())?;
        if rhs == "old" {
            Ok(Self::Squared)
        } else {
            let rhs = rhs
                .parse::<u64>()
                .map_err(|_| "Invalid format".to_string())?;
            match operand {
                "+" => Ok(Self::Plus(rhs)),
                "*" => Ok(Self::Times(rhs)),
                _ => Err("Invalid format".to_string()),
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    throw_to_if_true_id: usize,
    throw_to_if_false_id: usize,
    num_of_inspections: u64,
}

fn next_line_or_err<'a>(i: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, String> {
    i.next().ok_or("Invalid format".to_string())
}

fn parse_number_at_last_word_of<N: FromStr>(s: &str) -> Result<N, String> {
    s.rsplit_once(' ')
        .map(|(_, n)| n.parse::<N>().ok())
        .flatten()
        .ok_or("Invalid format".to_string())
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();

        let _ = next_line_or_err(&mut lines_iter)?;

        let second_line = next_line_or_err(&mut lines_iter)?;
        let (_, items_str) = second_line
            .split_once(": ")
            .ok_or("Invalid format".to_string())?;
        let items = items_str
            .split(", ")
            .map(|n| n.parse::<u64>())
            .collect::<Result<VecDeque<_>, _>>()
            .map_err(|_| "Invalid format".to_string())?;

        let third_line = next_line_or_err(&mut lines_iter)?;
        let (_, op_str) = third_line
            .split_once("old ")
            .ok_or("Invalid format".to_string())?;
        let operation = Operation::from_str(op_str)?;

        let fourth_line = next_line_or_err(&mut lines_iter)?;
        let divisor = parse_number_at_last_word_of(fourth_line)?;

        let fifth_line = next_line_or_err(&mut lines_iter)?;
        let true_id = parse_number_at_last_word_of(fifth_line)?;

        let sixth_line = next_line_or_err(&mut lines_iter)?;
        let false_id = parse_number_at_last_word_of(sixth_line)?;

        Ok(Self {
            items,
            operation,
            divisor,
            throw_to_if_true_id: true_id,
            throw_to_if_false_id: false_id,
            num_of_inspections: 0,
        })
    }
}

fn create_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split(EMPTY_LINE_PATTERN)
        .map(|s| Monkey::from_str(s).unwrap())
        .collect()
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> u64 {
    monkeys
        .iter()
        .map(|m| m.num_of_inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn simulate_round(monkeys: &mut [Monkey], use_remainders: bool) {
    let product_of_all_divisors = monkeys.iter().map(|m| m.divisor).product::<u64>();

    let num_of_monkeys = monkeys.len();
    for i in 0..num_of_monkeys {
        while let Some(old_worry_level) = monkeys[i].items.pop_front() {
            monkeys[i].num_of_inspections += 1;
            let new_worry_level = match monkeys[i].operation {
                Operation::Squared => old_worry_level * old_worry_level,
                Operation::Plus(rhs) => old_worry_level + rhs,
                Operation::Times(rhs) => old_worry_level * rhs,
            };
            let adjusted_worry_level = if use_remainders {
                // if using remainders, constrain the worry to be less than the product of
                // all remainders, as this retains all divisibility information inside the result
                // for all divisors that the monkeys have
                new_worry_level % product_of_all_divisors
            } else {
                // if not using remainders, divide by `3` to relief some worry
                new_worry_level / 3
            };

            let true_id = monkeys[i].throw_to_if_true_id;
            let false_id = monkeys[i].throw_to_if_false_id;

            let monkey_to_throw_to = if adjusted_worry_level % monkeys[i].divisor == 0 {
                monkeys.get_mut(true_id).unwrap()
            } else {
                monkeys.get_mut(false_id).unwrap()
            };
            monkey_to_throw_to.items.push_back(adjusted_worry_level);
        }
    }
}

fn simulate_game_for_20_rounds_with_worry_relief(input: &str) -> u64 {
    let mut monkeys = create_monkeys(input);

    for _ in 0..20 {
        simulate_round(&mut monkeys, false);
    }

    calculate_monkey_business(&monkeys)
}

fn simulate_game_for_10_000_rounds_without_worry_relief(input: &str) -> u64 {
    let mut monkeys = create_monkeys(input);

    for _ in 0..10_000 {
        simulate_round(&mut monkeys, true);
    }

    calculate_monkey_business(&monkeys)
}

fn main() {
    let input = read_file_to_string("input/day11.txt");

    let level_of_monkey_business_after_20_rounds =
        simulate_game_for_20_rounds_with_worry_relief(&input);
    println!("The level of monkey business after 20 rounds with worry relief is {level_of_monkey_business_after_20_rounds}");

    let level_of_monkey_business_after_10_000_rounds =
        simulate_game_for_10_000_rounds_without_worry_relief(&input);
    println!("The level of monkey business after 10,000 rounds without worry relief is {level_of_monkey_business_after_10_000_rounds}");
}
