use advent_of_code_2022::{read_file_to_string, EMPTY_LINE_PATTERN};
use sscanf::scanf;
use std::str::FromStr;

#[derive(Debug)]
struct Step {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (quantity, from, to): (usize, usize, usize) =
            scanf!(s, "move {usize} from {usize} to {usize}")
                .map_err(|_| "Invalid format".to_string())?;

        Ok(Self { quantity, from, to })
    }
}

fn get_char_idx(stack_idx: usize) -> usize {
    1 + 4 * stack_idx
}

#[derive(Debug)]
struct CrateMover {
    stacks: Vec<Vec<char>>,
}

impl FromStr for CrateMover {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().rev();
        let number_line = iter.next().ok_or("Invalid format".to_string())?.trim_end();
        let num_of_stacks = number_line
            .chars()
            .last()
            .ok_or("Invalid format".to_string())?
            .to_digit(10)
            .ok_or("Invalid format".to_string())? as usize;

        let mut stacks = vec![Vec::new(); num_of_stacks];

        for line in iter {
            for i in 0..num_of_stacks {
                let idx = get_char_idx(i);
                if let Some(item) = line.chars().nth(idx) {
                    if item != ' ' {
                        stacks[i].push(item);
                    }
                }
            }
        }

        Ok(Self { stacks })
    }
}

impl CrateMover {
    fn execute_9000(&mut self, step: &Step) {
        for _ in 0..step.quantity {
            let Some(top) = self.stacks[step.from - 1].pop() else {
                break;
            };
            self.stacks[step.to - 1].push(top);
        }
    }

    fn execute_9001(&mut self, step: &Step) {
        let n = self.stacks[step.from - 1].len() - step.quantity;
        let items_to_move = self.stacks[step.from - 1].drain(n..).collect::<Vec<_>>();
        self.stacks[step.to - 1].extend(items_to_move);
    }

    fn get_top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last().copied())
            .collect()
    }
}

fn get_top_row_after_rearranging_crates_using_crate_mover_9000_rules(
    crates: &str,
    steps: &Vec<Step>,
) -> String {
    let mut crate_area = CrateMover::from_str(crates).unwrap();

    for step in steps {
        crate_area.execute_9000(step);
    }

    crate_area.get_top_crates()
}

fn get_top_row_after_rearranging_crates_using_crate_mover_9001_rules(
    crates: &str,
    steps: &Vec<Step>,
) -> String {
    let mut crate_mover = CrateMover::from_str(crates).unwrap();

    for step in steps {
        crate_mover.execute_9001(step);
    }

    crate_mover.get_top_crates()
}

fn main() {
    let input = read_file_to_string("input/day5.txt");
    let (crates, steps) = input.split_once(EMPTY_LINE_PATTERN).unwrap();

    let steps = steps
        .lines()
        .map(|l| Step::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let top_row_crate_mover_9000 =
        get_top_row_after_rearranging_crates_using_crate_mover_9000_rules(crates, &steps);
    println!(
        "The top row after rearranging with CrateMover 9000 rules is: {top_row_crate_mover_9000}"
    );

    let top_row_crate_mover_9001 =
        get_top_row_after_rearranging_crates_using_crate_mover_9001_rules(crates, &steps);
    println!(
        "The top row after rearranging with CrateMover 9001 rules is: {top_row_crate_mover_9001}"
    );
}
