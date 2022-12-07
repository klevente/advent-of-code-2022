use advent_of_code_2022::read_file_lines_as;
use std::{collections::HashMap, str::FromStr};

const TOTAL_DISK_SPACE: usize = 70000000;
const REQUIRED_FREE_SPACE: usize = 30000000;

#[derive(Debug)]
enum Command {
    Cd { path: String },
    Ls,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cmd) = s.split_once(' ').ok_or("Invalid format".to_string())?;
        if cmd == "ls" {
            Ok(Self::Ls)
        } else {
            let (_, path) = cmd.split_once(' ').ok_or("Invalid format".to_string())?;
            Ok(Self::Cd {
                path: path.to_string(),
            })
        }
    }
}

#[derive(Debug)]
enum Output {
    Dir { name: String },
    File { size: usize },
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_or_dir, name) = s.split_once(' ').ok_or("Invalid format".to_string())?;
        let name = name.to_string();
        if size_or_dir == "dir" {
            Ok(Self::Dir { name })
        } else {
            let size = size_or_dir
                .parse()
                .map_err(|_| "Invalid format".to_string())?;
            Ok(Self::File { size })
        }
    }
}

#[derive(Debug)]
enum TerminalLine {
    Command(Command),
    Output(Output),
}

impl FromStr for TerminalLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            let cmd = Command::from_str(s)?;
            Ok(Self::Command(cmd))
        } else {
            let output = Output::from_str(s)?;
            Ok(Self::Output(output))
        }
    }
}

fn calculate_directory_sizes(lines: &Vec<TerminalLine>) -> HashMap<String, usize> {
    let mut directories_with_size = HashMap::<String, usize>::new();
    let mut current_path = Vec::<String>::new();
    for line in lines {
        match line {
            TerminalLine::Command(cmd) => match cmd {
                Command::Cd { path } => {
                    if path == "/" {
                        if current_path.is_empty() {
                            directories_with_size.insert("/".to_string(), 0);
                            current_path.push("/".to_string());
                        } else {
                            current_path.drain(1..);
                        }
                    } else if path == ".." {
                        current_path.pop();
                    } else {
                        current_path.push(path.to_string() + "/");
                    }
                }
                Command::Ls => {}
            },
            TerminalLine::Output(output) => match output {
                Output::Dir { name } => {
                    let absolute_path = current_path.join("") + name + "/";
                    directories_with_size.insert(absolute_path, 0);
                }
                Output::File { size } => {
                    let mut absolute_path = String::new();
                    for dir in &current_path {
                        absolute_path += &dir;
                        if let Some(old_size) = directories_with_size.get_mut(&absolute_path) {
                            *old_size += size;
                        }
                    }
                }
            },
        }
    }
    directories_with_size
}

fn get_sum_of_directory_sizes_smaller_than(
    n: usize,
    directories: &HashMap<String, usize>,
) -> usize {
    directories.values().filter(|&s| s <= &n).sum()
}

fn find_size_of_directory_to_delete(directories: &HashMap<String, usize>) -> usize {
    let total_used_space = directories.get("/").unwrap();
    let unused_space = TOTAL_DISK_SPACE - total_used_space;
    let additional_required_space = REQUIRED_FREE_SPACE - unused_space;
    *directories
        .values()
        .filter(|&s| s >= &additional_required_space)
        .min()
        .unwrap()
}

fn main() {
    let input = read_file_lines_as("input/day7.txt", |l| TerminalLine::from_str(l).unwrap());
    let directories_with_size = calculate_directory_sizes(&input);

    let sum_of_directory_sizes_smaller_than_100000 =
        get_sum_of_directory_sizes_smaller_than(100000, &directories_with_size);
    println!("The sum of directoriy sizes smaller than 100000 is {sum_of_directory_sizes_smaller_than_100000}");

    let size_to_delete = find_size_of_directory_to_delete(&directories_with_size);
    println!("The directory that should be deleted has a size of {size_to_delete}");
}
