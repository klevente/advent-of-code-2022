use advent_of_code_2022::{
    parse_2d_char_grid, parse_2d_grid_as, print_2d_array, read_file_to_string,
};
use array2d::Array2D;
use std::str::FromStr;

type Coords = (usize, usize);

#[derive(Clone)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Blizzard {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err(format!("Invalid format: {value}")),
        }
    }
}

impl std::fmt::Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Left => '<',
            Self::Right => '>',
            Self::Up => '^',
            Self::Down => 'v',
        };

        write!(f, "{c}")
    }
}

#[derive(Clone)]
enum Tile {
    Wall,
    Ground(Vec<Blizzard>),
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Ground(Vec::new())),
            v => Blizzard::try_from(v).map(|b| Self::Ground(vec![b])),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Wall => "#".to_string(),
            Self::Ground(blizzards) => match blizzards.len() {
                0 => ".".to_string(),
                1 => return blizzards[0].fmt(f),
                n => n.to_string(),
            },
        };
        write!(f, "{s}")
    }
}

#[derive(Clone)]
struct Valley {
    tiles: Array2D<Tile>,
    tiles_without_blizzards: Array2D<Tile>,
    player: Coords,
    elapsed_time: u32,
}

impl FromStr for Valley {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = parse_2d_grid_as(s, |c| Tile::try_from(c).unwrap());
        let mut tiles_without_blizzards = tiles.clone();
        for (y, x) in tiles_without_blizzards.indices_row_major() {
            if let Tile::Ground(blizzards) = tiles_without_blizzards.get_mut(y, x).unwrap() {
                blizzards.clear();
            }
        }

        Ok(Self {
            tiles,
            tiles_without_blizzards,
            player: (1, 0),
            elapsed_time: 0,
        })
    }
}

impl Valley {
    fn calculate(&self) -> u32 {
        let mut start = self.clone();
        let mut best_time_so_far = u32::MAX;

        start.get_quickest_time_to_exit(&mut best_time_so_far);
        best_time_so_far
    }

    fn get_quickest_time_to_exit(&mut self, best_time_so_far: &mut u32) {
        // println!("({}, {})", self.player.0, self.player.1);
        if self.player.0 == self.tiles.row_len() - 2 && self.player.1 == self.tiles.column_len() - 1
        {
            println!("SUCCESS, {}", self.elapsed_time);
            *best_time_so_far = (*best_time_so_far).min(self.elapsed_time);
            // return self.elapsed_time;
        }

        self.elapsed_time += 1;

        /* if self.elapsed_time > *best_time_so_far {
            return u32::MAX;
        }*/

        if self.elapsed_time + self.calculate_manhattan_distance_to_exit() > *best_time_so_far {
            return;
        }

        self.move_blizzards();

        let neighbours = self.get_neighbouring_tiles_for_player();

        // let mut best = u32::MAX;

        if neighbours.is_empty() {
            let mut cloned = self.clone();
            cloned.get_quickest_time_to_exit(best_time_so_far);
            // best = best.min(cloned.get_quickest_time_to_exit(best_time_so_far));
        } else {
            for new_player_pos in neighbours {
                let mut cloned = self.clone();
                cloned.player = new_player_pos;
                cloned.get_quickest_time_to_exit(best_time_so_far);
                // best = best.min(cloned.get_quickest_time_to_exit(best_time_so_far));
            }
        }

        // best
    }

    fn calculate_manhattan_distance_to_exit(&self) -> u32 {
        let exit = (
            (self.tiles.row_len() - 2) as i32,
            (self.tiles.column_len() - 1) as i32,
        );
        let player = (self.player.0 as i32, self.player.1 as i32);
        ((player.0 - exit.0).abs() + (player.1 - exit.1).abs()) as u32
    }

    fn get_neighbouring_tiles_for_player(&self) -> Vec<Coords> {
        let (x, y) = self.player;

        if x == 1 && y == 0 {
            if let Tile::Ground(b) = self.tiles.get(1, 1).unwrap() {
                if b.is_empty() {
                    return vec![(1, 1)];
                }
            }
            return Vec::new();
        }

        let neighbours = [(x + 1, y), (x, y + 1), (x, y - 1), (x - 1, y)];
        neighbours
            .into_iter()
            .filter(|(n_x, n_y)| match self.tiles.get(*n_y, *n_x).unwrap() {
                Tile::Ground(b) if b.is_empty() => true,
                _ => false,
            })
            .filter(|(n_x, n_y)| {
                if x == 1 && y == 1 && *n_x == 1 && *n_y == 0 {
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    fn move_blizzards(&mut self) {
        let mut new_tiles = self.tiles_without_blizzards.clone();

        for (y, x) in self.tiles.indices_row_major() {
            if let Tile::Ground(blizzards) = self.tiles.get(y, x).unwrap() {
                for b in blizzards {
                    let (n_x, n_y) = self.get_neighbouring_tile_for_blizzard(x, y, b);
                    if let Tile::Ground(new_blizzards) = new_tiles.get_mut(n_y, n_x).unwrap() {
                        new_blizzards.push(b.clone());
                    }
                }
            }
        }

        self.tiles = new_tiles;
    }

    fn get_neighbouring_tile_for_blizzard(
        &self,
        x: usize,
        y: usize,
        blizzard: &Blizzard,
    ) -> Coords {
        match blizzard {
            Blizzard::Left => {
                if x > 1 {
                    (x - 1, y)
                } else {
                    (self.tiles.row_len() - 2, y)
                }
            }
            Blizzard::Right => {
                if x < self.tiles.row_len() - 2 {
                    (x + 1, y)
                } else {
                    (1, y)
                }
            }
            Blizzard::Up => {
                if y > 1 {
                    (x, y - 1)
                } else {
                    (x, self.tiles.column_len() - 2)
                }
            }
            Blizzard::Down => {
                if y < self.tiles.column_len() - 2 {
                    (x, y + 1)
                } else {
                    (x, 1)
                }
            }
        }
    }

    fn print(&self) {
        print_2d_array(&self.tiles);
        println!("=========");
    }
}

fn main() {
    let input = read_file_to_string("input/day24.txt");
    let mut valley = Valley::from_str(&input).unwrap();
    valley.print();
    let result = valley.calculate();
    dbg!(result);
}
