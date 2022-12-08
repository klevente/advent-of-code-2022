use advent_of_code_2022::{parse_2d_number_grid, read_file_to_string};
use array2d::Array2D;
use take_until::TakeUntilExt;

struct Forest {
    height_map: Array2D<u8>,
}

impl Forest {
    fn new(s: &str) -> Self {
        let height_map = parse_2d_number_grid(s);
        Self { height_map }
    }

    fn is_tree_visible(&self, y: usize, x: usize) -> bool {
        let height = self.height_map.get(y, x).unwrap();
        let is_hidden_from_top = (0..y)
            .map(|i| self.height_map.get(i, x).unwrap())
            .any(|tree| tree >= height);
        if !is_hidden_from_top {
            return true;
        }
        let is_hidden_from_bottom = ((y + 1)..self.height_map.num_rows())
            .map(|i| self.height_map.get(i, x).unwrap())
            .any(|tree| tree >= height);
        if !is_hidden_from_bottom {
            return true;
        }
        let is_hidden_from_left = (0..x)
            .map(|i| self.height_map.get(y, i).unwrap())
            .any(|tree| tree >= height);
        if !is_hidden_from_left {
            return true;
        }

        let is_hidden_from_right = ((x + 1)..self.height_map.num_columns())
            .map(|i| self.height_map.get(y, i).unwrap())
            .any(|tree| tree >= height);
        if !is_hidden_from_right {
            return true;
        }

        false
    }

    fn calculate_number_of_visible_trees(&self) -> u32 {
        self.height_map
            .indices_row_major()
            .map(|(y, x)| self.is_tree_visible(y, x))
            .filter(|a| *a)
            .count() as u32
    }

    fn calculate_scenic_score_for(&self, y: usize, x: usize) -> u32 {
        let height = self.height_map.get(y, x).unwrap();
        let top_score = (0..y)
            .rev()
            .map(|i| self.height_map.get(i, x).unwrap())
            .take_until(|&tree| tree >= height)
            .count();

        let bottom_score = ((y + 1)..self.height_map.num_rows())
            .map(|i| self.height_map.get(i, x).unwrap())
            .take_until(|&tree| tree >= height)
            .count();

        let left_score = (0..x)
            .rev()
            .map(|i| self.height_map.get(y, i).unwrap())
            .take_until(|&tree| tree >= height)
            .count();

        let right_score = ((x + 1)..self.height_map.num_columns())
            .map(|i| self.height_map.get(y, i).unwrap())
            .take_until(|&tree| tree >= height)
            .count();

        (top_score * bottom_score * left_score * right_score) as u32
    }

    fn calculate_highest_scenic_score(&self) -> u32 {
        self.height_map
            .indices_row_major()
            .map(|(y, x)| self.calculate_scenic_score_for(y, x))
            .max()
            .unwrap()
    }
}

fn main() {
    let input = read_file_to_string("input/day8.txt");
    let forest = Forest::new(&input);
    let number_of_visible_trees = forest.calculate_number_of_visible_trees();
    println!("The number of visible trees in the forest is {number_of_visible_trees}");

    let highest_scenic_score = forest.calculate_highest_scenic_score();
    println!("The highest scenic score in the forest is {highest_scenic_score}");
}
