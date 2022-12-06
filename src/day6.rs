use advent_of_code_2022::read_file_to_string;
use itertools::Itertools;

fn find_position_of_first_n_unique_chars(s: &str, n: usize) -> Option<usize> {
    let v = s.chars().collect::<Vec<_>>();
    let result = v
        .windows(n)
        .find_position(|x| x.iter().unique().count() == n);
    result.map(|(idx, _)| idx + n)
}

fn main() {
    let input = read_file_to_string("input/day6.txt");

    let position_of_first_start_of_packet_marker =
        find_position_of_first_n_unique_chars(&input, 4).unwrap();
    println!("The position of the first start-of-packet marker is {position_of_first_start_of_packet_marker}");

    let position_of_first_start_of_message_marker =
        find_position_of_first_n_unique_chars(&input, 14).unwrap();
    println!("The position of the first start-of-message marker is {position_of_first_start_of_message_marker}");
}
