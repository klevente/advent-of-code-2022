use advent_of_code_2022::read_file_lines;
use std::collections::HashSet;

trait ToCharSet {
    fn to_set(&self) -> HashSet<char>;
}

impl ToCharSet for &str {
    fn to_set(&self) -> HashSet<char> {
        self.chars().collect()
    }
}

fn calculate_item_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        let difference = c as u8 - 'a' as u8;
        (1 + difference) as u32
    } else {
        let difference = c as u8 - 'A' as u8;
        (27 + difference) as u32
    }
}

fn find_item_present_in_both_compartments(rucksack: &str) -> Option<char> {
    let mid = rucksack.len() / 2;
    let (first_compartment, second_compartment) = rucksack.split_at(mid);

    let second_compartment_set = second_compartment.to_set();

    first_compartment
        .chars()
        .find(|c| second_compartment_set.contains(c))
}

fn calculate_sum_of_priorities_for_rucksacks(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|r| find_item_present_in_both_compartments(r).unwrap())
        .map(calculate_item_priority)
        .sum()
}

fn get_badge_type_for_group(rucksacks: &[String]) -> Option<char> {
    let mut sets = rucksacks
        .iter()
        .map(|r| (&r[..]).to_set())
        .collect::<Vec<_>>();
    let (intersection, others) = sets.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }
    intersection.iter().next().cloned()
}

fn calculate_sum_of_priorities_for_groups(rucksacks: &Vec<String>) -> u32 {
    let groups = rucksacks.chunks_exact(3);
    groups
        .map(|g| get_badge_type_for_group(g).unwrap())
        .map(calculate_item_priority)
        .sum()
}

fn main() {
    let lines = read_file_lines("input/day3.txt");
    let sum_of_priorities_for_rucksacks = calculate_sum_of_priorities_for_rucksacks(&lines);
    println!("The sum of priorities for the rucksacks is {sum_of_priorities_for_rucksacks}");
    let sum_of_priorities_for_groups = calculate_sum_of_priorities_for_groups(&lines);
    println!("The sum of priorities for the groups is {sum_of_priorities_for_groups}");
}
