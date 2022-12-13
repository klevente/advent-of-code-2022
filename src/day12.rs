use advent_of_code_2022::{parse_2d_char_grid, read_file_to_string};
use array2d::Array2D;
use std::{collections::HashSet, str::FromStr};

type Coords = (usize, usize);

enum Direction {
    Forwards,
    Backwards,
}

struct Valley {
    height_map: Array2D<u8>,
    start: Coords,
    finish: Coords,
}

impl FromStr for Valley {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height_map_original = parse_2d_char_grid(s);
        let start = height_map_original
            .indices_row_major()
            .find(|(y, x)| {
                let elem = height_map_original.get(*y, *x).unwrap();
                elem == &'S'
            })
            .ok_or("Invalid format".to_string())?;

        let finish = height_map_original
            .indices_row_major()
            .find(|(y, x)| {
                let elem = height_map_original.get(*y, *x).unwrap();
                elem == &'E'
            })
            .ok_or("Invalid format".to_string())?;

        let height_map_iter = height_map_original
            .elements_row_major_iter()
            .map(|c| match c {
                'S' => 0,
                'E' => 25,
                c => (*c as u8) - ('a' as u8),
            });

        let height_map = Array2D::from_iter_row_major(
            height_map_iter,
            height_map_original.num_rows(),
            height_map_original.num_columns(),
        )
        .map_err(|_| "Invalid format".to_string())?;

        Ok(Self {
            height_map,
            start,
            finish,
        })
    }
}

impl Valley {
    fn find_length_of_shortest_path_from_start_to_finish(&self) -> usize {
        let shortest_paths = self.find_shortest_paths_from(&self.start, &Direction::Forwards);
        *shortest_paths.get(self.finish.0, self.finish.1).unwrap()
    }

    fn find_length_of_shortest_path_from_lowest_points_to_finish(&self) -> usize {
        let shortest_paths_from_finish =
            self.find_shortest_paths_from(&self.finish, &Direction::Backwards);
        self.height_map
            .indices_row_major()
            .filter_map(|(r, c)| {
                let height = *self.height_map.get(r, c).unwrap();

                if height == 0 {
                    let path = *shortest_paths_from_finish.get(r, c).unwrap();
                    Some(path)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn find_shortest_paths_from(&self, start: &Coords, dir: &Direction) -> Array2D<usize> {
        let mut shortest_path_until = Array2D::filled_with(
            usize::MAX,
            self.height_map.num_rows(),
            self.height_map.num_columns(),
        );
        shortest_path_until.set(start.0, start.1, 0).unwrap();

        let mut coords_under_improvement = HashSet::new();
        coords_under_improvement.insert(*start);

        while !coords_under_improvement.is_empty() {
            coords_under_improvement =
                self.try_improve_paths(&mut shortest_path_until, coords_under_improvement, dir);
        }

        shortest_path_until
    }

    fn try_improve_paths(
        &self,
        shortest_path_until: &mut Array2D<usize>,
        coords_under_improvement: HashSet<Coords>,
        dir: &Direction,
    ) -> HashSet<Coords> {
        let mut changed = HashSet::new();
        for (row, column) in coords_under_improvement {
            let shortest_path_len = *shortest_path_until.get(row, column).unwrap();
            let neighbours = self.get_neighbours(&(row, column), dir);
            for n in &neighbours {
                let neighbour_shortest_len = *shortest_path_until.get(n.0, n.1).unwrap();

                let potential = shortest_path_len + 1;
                if potential < neighbour_shortest_len {
                    shortest_path_until.set(n.0, n.1, potential).unwrap();
                    changed.insert(*n);
                }
            }
        }

        changed
    }

    /*
     * checks whether a tile is reachable from another tile according to the rule:
     * a tile is reachable from the current if it's at most 1 higher
     * e.g. if `current = 5`, then neighbour can be `0..=6`
     */
    fn can_reach(current_height: u8, neighbour_height: u8) -> bool {
        current_height + 1 >= neighbour_height
    }

    fn get_neighbours(&self, pos: &Coords, dir: &Direction) -> Vec<Coords> {
        let row = pos.0 as isize;
        let column = pos.1 as isize;
        let neighbour_positions = [
            (row, column + 1),
            (row, column - 1),
            (row - 1, column),
            (row + 1, column),
        ];

        let height = self.get_height_at(pos.0, pos.1).unwrap();

        neighbour_positions
            .iter()
            .filter_map(|(r, c)| {
                if *r >= 0 && *c >= 0 {
                    let r = *r as usize;
                    let c = *c as usize;
                    self.get_height_at(r, c)
                        .filter(|&neighbour_height| match dir {
                            // when going forwards check what neighbouring tiles can the current tile reach
                            Direction::Forwards => Self::can_reach(height, neighbour_height),
                            // when going backwards, check the opposite: what neighbouring tiles can reach the current tile
                            Direction::Backwards => Self::can_reach(neighbour_height, height),
                        })
                        .and(Some((r, c)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_height_at(&self, row: usize, column: usize) -> Option<u8> {
        self.height_map.get(row, column).copied()
    }
}

fn main() {
    let input = read_file_to_string("input/day12.txt");
    let valley = Valley::from_str(&input).unwrap();
    let length_of_shortest_path_from_start_to_finish =
        valley.find_length_of_shortest_path_from_start_to_finish();
    println!("The length of the shortest path from start to finish is {length_of_shortest_path_from_start_to_finish}");

    let length_of_shortest_path_from_lowest_points_to_finish =
        valley.find_length_of_shortest_path_from_lowest_points_to_finish();
    println!("The length of the shortest path from all lowest points to finish is {length_of_shortest_path_from_lowest_points_to_finish}");
}
