use advent_of_code_2022::read_file_lines_as;
use itertools::Itertools;
use std::collections::HashMap;

fn print_array(indices: &[(i64, usize)]) {
    let out = indices
        .iter()
        .sorted_by(|(_, i), (_, j)| i.cmp(j))
        .map(|(n, _)| *n)
        .join(", ");
    println!("[{out}]");
}

fn check_indices_unique_and_in_bounds(indices: &[(i64, usize)]) {
    let iter = indices.iter().map(|(_, i)| *i);
    let unique = iter.clone().all_unique();

    if !unique {
        let freq = indices.iter().counts();
        dbg!(freq);
        panic!("The indices were not unique");
    }

    let bounds = 0..indices.len();
    let elements_out_of_bounds = iter
        .clone()
        .filter(|i| !bounds.contains(&i))
        .collect::<Vec<_>>();

    if !elements_out_of_bounds.is_empty() {
        dbg!(elements_out_of_bounds);
        panic!("The indices were not in bounds");
    }
}

const KEY: i64 = 811589153;

fn main() {
    let arr = read_file_lines_as("input/day20.txt", |l| l.parse::<i64>().unwrap());

    let mut indices = arr
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n * 1, i))
        .collect::<Vec<_>>();

    print_array(&indices);

    // dbg!(&indices);

    let n = indices.len() as i64;
    for i in 0..indices.len() {
        if i % 100 == 0 {
            dbg!(i);
        }
        let (value, old_idx) = indices[i];
        let new_idx = if value > 0 {
            let temp = value + old_idx as i64;
            if temp >= n {
                (temp + 1).rem_euclid(n)
            } else {
                temp
            }
        } else if value < 0 {
            let temp = value + old_idx as i64;
            if temp < 1 {
                (temp - 1).rem_euclid(n)
            } else {
                temp
            }
        } else {
            continue;
        };
        let new_idx = new_idx as usize;
        indices[i].1 = new_idx;
        // dbg!(new_idx);
        let should_push_left = new_idx > old_idx;
        let range_to_update = (old_idx.min(new_idx))..=(old_idx.max(new_idx));

        for j in 0..indices.len() {
            if i != j && range_to_update.contains(&indices[j].1) {
                if should_push_left {
                    indices[j].1 -= 1;
                } else {
                    indices[j].1 += 1;
                }
            }
        }

        check_indices_unique_and_in_bounds(&indices);
        print_array(&indices);
        // dbg!(&indices);
    }

    print_array(&indices);
    let position_of_0 = indices.iter().find(|(n, _)| n == &0).unwrap().1;
    dbg!(position_of_0);
    let position_of_x = (position_of_0 + 1000) % n as usize;
    let position_of_y = (position_of_0 + 2000) % n as usize;
    let position_of_z = (position_of_0 + 3000) % n as usize;

    let x = indices.iter().find(|(_, i)| i == &position_of_x).unwrap().0;
    let y = indices.iter().find(|(_, i)| i == &position_of_y).unwrap().0;
    let z = indices.iter().find(|(_, i)| i == &position_of_z).unwrap().0;

    dbg!(x, y, z);
    dbg!(x + y + z);
}
