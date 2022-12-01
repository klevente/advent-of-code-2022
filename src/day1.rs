use advent_of_code_2022::read_file_lines;
use itertools::Itertools;

fn calculate_calories_for_elf_carrying_the_most(calories: &Vec<String>) -> u32 {
    let (max_value, _) =
        calories
            .iter()
            .fold((0u32, 0u32), |(absolute_max, local_max), calorie_line| {
                if calorie_line.is_empty() {
                    (absolute_max.max(local_max), 0)
                } else {
                    (
                        absolute_max,
                        local_max + calorie_line.parse::<u32>().unwrap(),
                    )
                }
            });
    max_value
}

fn calculate_calories_for_top_three_elves(calories: &Vec<String>) -> u32 {
    let calories_grouped_by_elves = calories
        .iter()
        .fold(vec![0u32], |mut groups, calorie_line| {
            if calorie_line.is_empty() {
                groups.push(0);
                groups
            } else {
                if let Some(last) = groups.last_mut() {
                    *last += calorie_line.parse::<u32>().unwrap();
                }
                groups
            }
        });

    calories_grouped_by_elves
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}

fn main() {
    let lines = read_file_lines("input/day1.txt");
    let calories_for_elf_carrying_the_most = calculate_calories_for_elf_carrying_the_most(&lines);
    println!("The elf with the most calories carries {calories_for_elf_carrying_the_most}");
    let calories_for_top_three_elves = calculate_calories_for_top_three_elves(&lines);
    println!(
        "The amount of calories the top three elves are taking is {calories_for_top_three_elves}"
    );
}
