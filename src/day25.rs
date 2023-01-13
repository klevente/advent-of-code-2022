use advent_of_code_2022::read_file_lines;
use itertools::Itertools;
use std::collections::VecDeque;

fn snafu_digit_to_decimal(d: char) -> i64 {
    match d {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn snafu_to_decimal(n: &str) -> i64 {
    let (result, _) = n.chars().rev().fold((0, 0), |(acc, pow), digit| {
        let decimal_digit = snafu_digit_to_decimal(digit);
        (acc + (decimal_digit * 5_i64.pow(pow)), pow + 1)
    });
    result
}

fn decimal_digit_to_snafu(d: i64) -> char {
    match d {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unreachable!(),
    }
}

fn decimal_to_snafu(n: i64) -> String {
    let mut result = VecDeque::new();
    let mut n = n;
    let mut carry = 0;
    while n != 0 {
        let mut digit = (n + carry) % 5;
        carry = if digit == 3 || digit == 4 { 1 } else { 0 };
        let digit = decimal_digit_to_snafu(digit);
        result.push_front(digit);

        n /= 5;
    }
    if carry != 0 {
        result.push_front(decimal_digit_to_snafu(carry));
    }

    result.iter().join("")
}

fn main() {
    let input = read_file_lines("input/day25.txt");
    let sum_decimal = input.iter().map(|l| snafu_to_decimal(l)).sum::<i64>();
    let snafu_number_needed_for_bobs_console = decimal_to_snafu(sum_decimal);
    println!(
        "The SNAFU number required for Bob's console is {snafu_number_needed_for_bobs_console}"
    );
}
