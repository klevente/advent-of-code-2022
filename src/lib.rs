use array2d::Array2D;
use std::{
    convert::{identity, TryInto},
    fmt::Display,
    fs::read_to_string,
    path::Path,
};

#[cfg(windows)]
pub const LINE_SEPARATOR: &'static str = "\r\n";
#[cfg(not(windows))]
pub const LINE_SEPARATOR: &'static str = "\n";

#[cfg(windows)]
pub const EMPTY_LINE_PATTERN: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub const EMPTY_LINE_PATTERN: &'static str = "\n\n";

pub fn read_file_to_string(path: impl AsRef<Path>) -> String {
    read_to_string(path).unwrap()
}

pub fn read_file_lines(path: impl AsRef<Path>) -> Vec<String> {
    read_file_lines_as(path, str::to_string)
}

pub fn read_file_lines_as<T>(path: impl AsRef<Path>, f: fn(&str) -> T) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().map(f).collect()
}

pub fn read_file_lines_filter_as<T>(path: impl AsRef<Path>, f: fn(&str) -> Option<T>) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().filter_map(f).collect()
}

pub fn read_file_lines_extract_first(path: impl AsRef<Path>) -> (String, Vec<String>) {
    let mut input = read_file_lines(path)
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.to_owned())
            }
        })
        .collect::<Vec<_>>();
    let first = input.remove(0);

    (first, input)
}

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn print_2d_array<T: Clone + Display>(array: &Array2D<T>) {
    for row in array.rows_iter() {
        for column in row.into_iter() {
            print!("{}", column);
        }
        println!();
    }
}

pub fn print_u8_2d_array_with_delim(array: &Array2D<u8>) {
    for row in array.rows_iter() {
        print!("|");
        for column in row.into_iter() {
            print!("{:2}|", column);
        }
        println!();
    }
}

pub fn print_usize_2d_array_with_delim(array: &Array2D<usize>) {
    for row in array.rows_iter() {
        print!("|");
        for column in row.into_iter() {
            if column == &usize::MAX {
                print!("{:5}|", "  MAX");
            } else {
                print!("{:5}|", column);
            }
        }
        println!();
    }
}

pub fn parse_2d_grid_as<T: Clone>(s: &str, f: fn(char) -> T) -> Array2D<T> {
    let elements = &*s
        .lines()
        .map(|l| l.chars().map(f).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2D::from_rows(elements).unwrap()
}

pub fn parse_2d_char_grid(s: &str) -> Array2D<char> {
    parse_2d_grid_as(s, identity)
}

pub fn parse_2d_number_grid(s: &str) -> Array2D<u8> {
    parse_2d_grid_as(s, |d| d.to_digit(10).unwrap() as u8)
}
