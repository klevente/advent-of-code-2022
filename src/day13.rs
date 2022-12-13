use advent_of_code_2022::{
    read_file_to_string, unwrap_enum_variant_inner, EMPTY_LINE_PATTERN, LINE_SEPARATOR,
};
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

fn traverse_syntax_tree<'a>(root: &'a mut Vec<Packet>, indices: &[usize]) -> &'a mut Vec<Packet> {
    let mut elem = root;
    for &i in indices {
        elem = unwrap_enum_variant_inner!(&mut elem[i], Packet::List);
    }
    elem
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = Self::List(Vec::new());
        let root_vec = unwrap_enum_variant_inner!(&mut root, Self::List);
        let mut depth_stack = Vec::new();
        let mut iter = s.chars().peekable();
        iter.next();
        while let Some(c) = iter.next() {
            let current_vec = traverse_syntax_tree(root_vec, &depth_stack);
            if c.is_digit(10) {
                if c == '1' {
                    if let Some(&next) = iter.peek() {
                        if next == '0' {
                            current_vec.push(Self::Integer(10));
                            iter.next();
                        } else {
                            current_vec.push(Self::Integer(1));
                        }
                    } else {
                        current_vec.push(Self::Integer(1));
                    }
                } else {
                    let digit = c.to_digit(10).unwrap() as u8;
                    current_vec.push(Self::Integer(digit));
                }
            } else if c == '[' {
                current_vec.push(Self::List(Vec::new()));
                depth_stack.push(current_vec.len() - 1);
            } else if c == ']' {
                depth_stack.pop();
            } else if c == ',' {
                // do nothing
            } else {
                return Err("Invalid format".to_string());
            }
        }

        Ok(root)
    }
}

#[derive(Debug, PartialEq)]
enum CompareResult {
    RightOrder,
    WrongOrder,
    Continue,
}

fn compare_packets(lhs: &Packet, rhs: &Packet) -> CompareResult {
    match (lhs, rhs) {
        (Packet::Integer(lhs), Packet::Integer(rhs)) => {
            if lhs < rhs {
                CompareResult::RightOrder
            } else if lhs > rhs {
                CompareResult::WrongOrder
            } else {
                CompareResult::Continue
            }
        }
        (Packet::List(lhs), Packet::List(rhs)) => {
            let shorter_len = lhs.len().min(rhs.len());
            for i in 0..shorter_len {
                let result = compare_packets(&lhs[i], &rhs[i]);
                if result != CompareResult::Continue {
                    return result;
                }
            }
            if lhs.len() < rhs.len() {
                CompareResult::RightOrder
            } else if lhs.len() > rhs.len() {
                CompareResult::WrongOrder
            } else {
                CompareResult::Continue
            }
        }
        (lhs, rhs) => {
            if let Packet::Integer(x) = lhs {
                let lhs = Packet::List(vec![Packet::Integer(*x)]);
                compare_packets(&lhs, rhs)
            } else if let Packet::Integer(x) = rhs {
                let rhs = Packet::List(vec![Packet::Integer(*x)]);
                compare_packets(lhs, &rhs)
            } else {
                unreachable!("The first 2 cases should cover everything")
            }
        }
    }
}

fn parse_packet_pairs(s: &str) -> Vec<(Packet, Packet)> {
    let split = s.split(EMPTY_LINE_PATTERN);
    split
        .map(|s| {
            let (lhs, rhs) = s.split_once(LINE_SEPARATOR).unwrap();
            (
                Packet::from_str(lhs).unwrap(),
                Packet::from_str(rhs).unwrap(),
            )
        })
        .collect()
}

fn calculate_sum_of_indices_of_correct_pairs(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (lhs, rhs))| {
            if compare_packets(lhs, rhs) == CompareResult::RightOrder {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn parse_all_packets(s: &str) -> Vec<Packet> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from_str(l).unwrap())
        .collect()
}

fn sort_packets_and_calculate_decoder_key(mut packets: Vec<Packet>) -> usize {
    packets.sort_by(|a, b| {
        let result = compare_packets(a, b);
        match result {
            CompareResult::RightOrder => Ordering::Less,
            CompareResult::WrongOrder => Ordering::Greater,
            CompareResult::Continue => Ordering::Equal,
        }
    });
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let Packet::List(outer_list) = p else {
                return None;
            };
            if outer_list.len() != 1 {
                return None;
            }

            let Packet::List(inner_list) = &outer_list[0] else {
                return None;
            };
            if inner_list.len() != 1 {
                return None;
            }

            let Packet::Integer(n) = inner_list[0] else {
                return None;
            };
            if n == 2 || n == 6 {
                return Some(i + 1);
            }
            None
        })
        .product()
}

fn main() {
    let input = read_file_to_string("input/day13.txt");
    let packet_pairs = parse_packet_pairs(&input);
    let sum_of_indices_of_correct_pairs = calculate_sum_of_indices_of_correct_pairs(&packet_pairs);
    println!("The sum of the indices of correct packet pairs is {sum_of_indices_of_correct_pairs}");

    let mut all_packets = parse_all_packets(&input);
    all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]));
    all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]));
    let decoder_key = sort_packets_and_calculate_decoder_key(all_packets);
    println!("The decoder key is {decoder_key}");
}
