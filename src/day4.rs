use advent_of_code_2022::read_file_lines_as;
use sscanf::scanf;
use std::{ops::RangeInclusive, str::FromStr};

fn is_range_contained_by(container: &RangeInclusive<u32>, containee: &RangeInclusive<u32>) -> bool {
    container.start() <= containee.start() && container.end() >= containee.end()
}

struct SectionAssignment {
    elf_1: RangeInclusive<u32>,
    elf_2: RangeInclusive<u32>,
}

impl FromStr for SectionAssignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_1, to_1, from_2, to_2): (u32, u32, u32, u32) =
            scanf!(s, "{u32}-{u32},{u32}-{u32}").map_err(|_| "Invalid format".to_string())?;

        Ok(Self {
            elf_1: from_1..=to_1,
            elf_2: from_2..=to_2,
        })
    }
}

impl SectionAssignment {
    fn does_one_fully_contain_the_other(&self) -> bool {
        is_range_contained_by(&self.elf_1, &self.elf_2)
            || is_range_contained_by(&self.elf_2, &self.elf_1)
    }

    fn do_ranges_overlap(&self) -> bool {
        self.elf_1.start() <= self.elf_2.start() && self.elf_2.start() <= self.elf_1.end()
            || self.elf_2.start() <= self.elf_1.start() && self.elf_1.start() <= self.elf_2.end()
    }
}

fn count_filtered_assignments(
    assignments: &Vec<SectionAssignment>,
    f: fn(&SectionAssignment) -> bool,
) -> u32 {
    assignments.iter().filter(|a| f(a)).count() as u32
}

fn main() {
    let assignments = read_file_lines_as("input/day4.txt", |l| {
        SectionAssignment::from_str(l).unwrap()
    });
    let number_of_assignments_where_one_is_fully_contained =
        count_filtered_assignments(&assignments, |a| a.does_one_fully_contain_the_other());
    println!("The number of assignments where one section fully contains the other is {number_of_assignments_where_one_is_fully_contained}");
    let number_of_assignments_where_there_is_overlap =
        count_filtered_assignments(&assignments, |a| a.do_ranges_overlap());
    println!("The number of assignments where one section overlaps the other is {number_of_assignments_where_there_is_overlap}");
}
