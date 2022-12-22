use advent_of_code_2022::{print_2d_array, print_2d_array_flipped_vertically, read_file_to_string};
use array2d::Array2D;

#[derive(Clone)]
enum Tile {
    Air,
    Rock,
    FallingRock,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Air => '.',
            Self::Rock => '#',
            Self::FallingRock => '@',
        };
        write!(f, "{c}")
    }
}

type Coords = (usize, usize);

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Rock {
    Dash {
        left: Coords,
        left_center: Coords,
        right_center: Coords,
        right: Coords,
    },
    Plus {
        top: Coords,
        left: Coords,
        center: Coords,
        right: Coords,
        bottom: Coords,
    },
    ReverseL {
        top: Coords,
        center: Coords,
        bottom_left: Coords,
        bottom_center: Coords,
        bottom_right: Coords,
    },
    I {
        top: Coords,
        top_center: Coords,
        bottom_center: Coords,
        bottom: Coords,
    },
    Square {
        top_left: Coords,
        top_right: Coords,
        bottom_left: Coords,
        bottom_right: Coords,
    },
}

impl Rock {
    fn get_coords(&self) -> Vec<&Coords> {
        match self {
            Rock::Dash {
                left,
                left_center,
                right_center,
                right,
            } => vec![left, left_center, right_center, right],
            Rock::Plus {
                top,
                left,
                center,
                right,
                bottom,
            } => vec![top, left, center, right, bottom],
            Rock::ReverseL {
                top,
                center,
                bottom_left,
                bottom_center,
                bottom_right,
            } => vec![top, center, bottom_left, bottom_center, bottom_right],
            Rock::I {
                top,
                top_center,
                bottom_center,
                bottom,
            } => vec![top, top_center, bottom_center, bottom],
            Rock::Square {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => vec![top_left, top_right, bottom_left, bottom_right],
        }
    }

    fn get_coords_mut(&mut self) -> Vec<&mut Coords> {
        match self {
            Rock::Dash {
                left,
                left_center,
                right_center,
                right,
            } => vec![left, left_center, right_center, right],
            Rock::Plus {
                top,
                left,
                center,
                right,
                bottom,
            } => vec![top, left, center, right, bottom],
            Rock::ReverseL {
                top,
                center,
                bottom_left,
                bottom_center,
                bottom_right,
            } => vec![top, center, bottom_left, bottom_center, bottom_right],
            Rock::I {
                top,
                top_center,
                bottom_center,
                bottom,
            } => vec![top, top_center, bottom_center, bottom],
            Rock::Square {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => vec![top_left, top_right, bottom_left, bottom_right],
        }
    }

    fn get_coords_for_left_collision_check(&self) -> Vec<&Coords> {
        match self {
            Rock::Dash { left, .. } => vec![left],
            Rock::Plus {
                top, left, bottom, ..
            } => vec![top, left, bottom],
            Rock::ReverseL {
                top,
                center,
                bottom_left,
                ..
            } => vec![top, center, bottom_left],
            Rock::I {
                top,
                top_center,
                bottom_center,
                bottom,
            } => vec![top, top_center, bottom_center, bottom],
            Rock::Square {
                top_left,
                bottom_left,
                ..
            } => vec![top_left, bottom_left],
        }
    }

    fn get_coords_for_right_collision_check(&self) -> Vec<&Coords> {
        match self {
            Rock::Dash { right, .. } => vec![right],
            Rock::Plus {
                top, right, bottom, ..
            } => vec![top, right, bottom],
            Rock::ReverseL {
                top,
                center,
                bottom_right,
                ..
            } => vec![top, center, bottom_right],
            Rock::I {
                top,
                top_center,
                bottom_center,
                bottom,
            } => vec![top, top_center, bottom_center, bottom],
            Rock::Square {
                top_right,
                bottom_right,
                ..
            } => vec![top_right, bottom_right],
        }
    }

    fn get_coords_for_down_collision_check(&self) -> Vec<&Coords> {
        match self {
            Rock::Dash {
                left,
                left_center,
                right_center,
                right,
            } => vec![left, left_center, right_center, right],
            Rock::Plus {
                left,
                right,
                bottom,
                ..
            } => vec![left, right, bottom],
            Rock::ReverseL {
                bottom_left,
                bottom_center,
                bottom_right,
                ..
            } => vec![bottom_left, bottom_center, bottom_right],
            Rock::I { bottom, .. } => vec![bottom],
            Rock::Square {
                bottom_left,
                bottom_right,
                ..
            } => vec![bottom_left, bottom_right],
        }
    }

    fn move_down(&mut self) {
        for (_, y) in self.get_coords_mut() {
            *y -= 1;
        }
    }

    fn move_sideways(&mut self, direction: &Direction) {
        for (x, _) in self.get_coords_mut() {
            match direction {
                Direction::Left => *x -= 1,
                Direction::Right => *x += 1,
            };
        }
    }
}

struct Cavern {
    tiles: Array2D<Tile>,
    num_of_rocks: usize,
    highest_point: usize,
    push_rules: Vec<Direction>,
    next_push_rule: usize,
    next_rock_idx: usize,
    push_rule_cnt: usize,
}

impl Cavern {
    fn new(num_of_rocks: usize, push_rules: &str) -> Self {
        let push_rules = push_rules
            .chars()
            .map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => unreachable!(),
            })
            .collect();
        Self {
            tiles: Array2D::filled_with(Tile::Air, 3170 /*num_of_rocks * 4 + 10*/, 7),
            num_of_rocks,
            highest_point: 0,
            push_rules,
            next_push_rule: 914,
            next_rock_idx: 1,
            push_rule_cnt: 0,
        }
    }

    fn simulate(&mut self) -> usize {
        dbg!(self.push_rules.len());
        for i in 0..self.num_of_rocks {
            /*println!(
                "=i={}, rule_idx={}, rock_idx={}, cnt={}====",
                i, self.next_push_rule, self.next_rock_idx, self.push_rule_cnt
            );*/
            /*if i == 1861 {
                dbg!(self.next_rock_idx);
                dbg!(self.next_push_rule);
                dbg!(self.push_rule_cnt);
            }*/
            self.drop_rock(i);
            // print_2d_array_flipped_vertically(&self.tiles);
        }
        // print_2d_array_flipped_vertically(&self.tiles);
        println!("{}", self.push_rule_cnt);
        self.highest_point
    }

    fn drop_rock(&mut self, i: usize) {
        let mut rock = self.create_next_rock();
        // self.print_falling_rock(&rock);
        loop {
            let direction = self.push_rules[self.next_push_rule].clone();
            self.next_push_rule = (self.next_push_rule + 1) % self.push_rules.len();
            self.push_rule_cnt += 1;
            if !self.has_rock_hit_side(&rock, &direction) {
                rock.move_sideways(&direction);
            }
            // self.print_falling_rock(&rock);
            if self.has_rock_hit_bottom(&rock) {
                break;
            }
            rock.move_down();
            // self.print_falling_rock(&rock);
        }
        // self.print_falling_rock(&rock);
        self.place_rock_into_cavern(rock, i);
    }

    fn create_next_rock(&mut self) -> Rock {
        let bottom = self.highest_point + 3;
        let rock = match self.next_rock_idx {
            0 => Rock::Dash {
                left: (2, bottom),
                left_center: (3, bottom),
                right_center: (4, bottom),
                right: (5, bottom),
            },
            1 => Rock::Plus {
                top: (3, bottom + 2),
                left: (2, bottom + 1),
                center: (3, bottom + 1),
                right: (4, bottom + 1),
                bottom: (3, bottom),
            },
            2 => Rock::ReverseL {
                top: (4, bottom + 2),
                center: (4, bottom + 1),
                bottom_left: (2, bottom),
                bottom_center: (3, bottom),
                bottom_right: (4, bottom),
            },
            3 => Rock::I {
                top: (2, bottom + 3),
                top_center: (2, bottom + 2),
                bottom_center: (2, bottom + 1),
                bottom: (2, bottom),
            },
            4 => Rock::Square {
                top_left: (2, bottom + 1),
                top_right: (3, bottom + 1),
                bottom_left: (2, bottom),
                bottom_right: (3, bottom),
            },
            _ => unreachable!(),
        };

        self.next_rock_idx = (self.next_rock_idx + 1) % 5;
        rock
    }

    fn place_rock_into_cavern(&mut self, rock: Rock, i: usize) {
        let coords = rock.get_coords();
        for (x, y) in &coords {
            self.tiles.set(*y, *x, Tile::Rock).unwrap();
        }

        let potential_highest_point = coords.iter().map(|(_, y)| *y).max().unwrap();
        self.highest_point = self.highest_point.max(potential_highest_point + 1);
    }

    fn has_rock_hit_bottom(&self, rock: &Rock) -> bool {
        for (x, y) in rock.get_coords_for_down_collision_check() {
            if *y == 0 {
                return true;
            }
            let y = y - 1;
            if let Some(Tile::Rock) = self.tiles.get(y, *x) {
                return true;
            }
        }
        false
    }

    fn has_rock_hit_side(&self, rock: &Rock, direction: &Direction) -> bool {
        match direction {
            Direction::Left => {
                for (x, y) in rock.get_coords_for_left_collision_check() {
                    if *x == 0 {
                        return true;
                    }
                    let x = x - 1;
                    if let Some(Tile::Rock) = self.tiles.get(*y, x) {
                        return true;
                    }
                }
            }
            Direction::Right => {
                for (x, y) in rock.get_coords_for_right_collision_check() {
                    if *x == 6 {
                        return true;
                    }
                    let x = x + 1;
                    if let Some(Tile::Rock) = self.tiles.get(*y, x) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn print_falling_rock(&self, rock: &Rock) {
        let mut tiles = self.tiles.clone();
        for (x, y) in rock.get_coords() {
            tiles.set(*y, *x, Tile::FallingRock).unwrap();
        }
        print_2d_array_flipped_vertically(&tiles);
        println!("=======");
    }
}

/**
 * 3 sections:
 * 1.: bottom, where the floor is empty
 * 2.: cyclical, where it starts to repeat the same pattern because of the continuity
 *     of the push rules
 * 3.: end: the ending section is a partial of the cyclical section
 * - have to figure out: when does the cyclical section start, how long and tall it is,
 * - then just calculate how much full cycles we can do, and then build a partial cycle
 * - from the rest of the rocks we have after building the full cycles
**/

fn main() {
    let input = read_file_to_string("input/day17.txt");
    let mut cavern = Cavern::new(1053, &input);
    let result = cavern.simulate();
    dbg!(result);
}
