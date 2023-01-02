use advent_of_code_2022::{parse_2d_char_grid, print_2d_array, read_file_to_string};
use array2d::Array2D;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Coords = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Self::N => Self::S,
            Self::S => Self::W,
            Self::W => Self::E,
            Self::E => Self::N,
        }
    }
}

struct Crater {
    elves: HashSet<Coords>,
    starting_direction: Direction,
}

impl FromStr for Crater {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = parse_2d_char_grid(&s);

        let mut elves = HashSet::new();

        for (y, x) in positions.indices_row_major() {
            let tile = positions.get(y, x).unwrap();
            if tile == &'#' {
                elves.insert((x as i32, y as i32));
            }
        }

        Ok(Self {
            elves,
            starting_direction: Direction::N,
        })
    }
}

impl Crater {
    fn simulate_until_finished(&mut self) -> usize {
        let mut num_of_rounds = 0;
        loop {
            num_of_rounds += 1;
            let finished = self.tick();
            if finished {
                return num_of_rounds;
            }
        }
    }

    fn simulate_steps(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.tick();
        }

        self.calculate_number_of_empty_tiles()
    }

    fn tick(&mut self) -> bool {
        let mut proposed_positions = HashMap::new();

        for elf in &self.elves {
            if self.has_no_neighbour(elf) {
                continue;
            }

            let mut direction = self.starting_direction;
            for _ in 0..4 {
                if !self.has_neighbours_in_direction(elf, &direction) {
                    let neighbour = self.get_neighbour_in_direction(elf, &direction);
                    proposed_positions
                        .entry(neighbour)
                        .or_insert(Vec::new())
                        .push(*elf);

                    break;
                }
                direction = direction.next();
            }
        }

        if proposed_positions.is_empty() {
            return true;
        }

        let proposed_positions = proposed_positions
            .into_iter()
            .filter_map(|(p, e)| if e.len() == 1 { Some((p, e[0])) } else { None })
            .collect::<HashMap<_, _>>();

        for (new, current) in proposed_positions {
            self.elves.remove(&current);
            self.elves.insert(new);
        }

        self.starting_direction = self.starting_direction.next();

        false
    }

    fn get_neighbour_in_direction(&self, elf: &Coords, direction: &Direction) -> Coords {
        match direction {
            Direction::N => (elf.0, elf.1 - 1),
            Direction::S => (elf.0, elf.1 + 1),
            Direction::W => (elf.0 - 1, elf.1),
            Direction::E => (elf.0 + 1, elf.1),
        }
    }

    fn get_neighbouring_positions(&self, elf: &Coords, direction: &Direction) -> [Coords; 3] {
        match direction {
            Direction::N => [
                (elf.0 - 1, elf.1 - 1),
                (elf.0, elf.1 - 1),
                (elf.0 + 1, elf.1 - 1),
            ],
            Direction::S => [
                (elf.0 - 1, elf.1 + 1),
                (elf.0, elf.1 + 1),
                (elf.0 + 1, elf.1 + 1),
            ],
            Direction::W => [
                (elf.0 - 1, elf.1 - 1),
                (elf.0 - 1, elf.1),
                (elf.0 - 1, elf.1 + 1),
            ],
            Direction::E => [
                (elf.0 + 1, elf.1 - 1),
                (elf.0 + 1, elf.1),
                (elf.0 + 1, elf.1 + 1),
            ],
        }
    }

    fn has_neighbours_in_direction(&self, elf: &Coords, direction: &Direction) -> bool {
        let neighbouring_positions = self.get_neighbouring_positions(elf, direction);

        neighbouring_positions
            .iter()
            .any(|pos| self.elves.contains(pos))
    }

    fn has_no_neighbour(&self, elf: &Coords) -> bool {
        !self.has_neighbours_in_direction(elf, &Direction::N)
            && !self.has_neighbours_in_direction(elf, &Direction::S)
            && !self.has_neighbours_in_direction(elf, &Direction::W)
            && !self.has_neighbours_in_direction(elf, &Direction::E)
    }

    fn get_bounding_rect(&self) -> (Coords, Coords) {
        let mut min = (i32::MAX, i32::MAX);
        let mut max = (i32::MIN, i32::MIN);
        for (x, y) in &self.elves {
            let x = *x;
            let y = *y;

            if x < min.0 {
                min.0 = x;
            }
            if x > max.0 {
                max.0 = x;
            }

            if y < min.1 {
                min.1 = y;
            }
            if y > max.1 {
                max.1 = y;
            }
        }

        (min, max)
    }

    fn get_width_height(&self) -> (usize, usize) {
        let (min, max) = self.get_bounding_rect();
        let width = ((max.0 - min.0).abs() + 1) as usize;
        let height = ((max.1 - min.1).abs() + 1) as usize;

        (width, height)
    }

    fn calculate_number_of_empty_tiles(&self) -> usize {
        let (width, height) = self.get_width_height();

        let area = (width * height) as usize;
        let num_of_elves = self.elves.len();

        area - num_of_elves
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (min, _) = self.get_bounding_rect();
        let (width, height) = self.get_width_height();
        let mut arr = Array2D::filled_with('.', height, width);

        for (x, y) in &self.elves {
            let x = (x - min.0) as usize;
            let y = (y - min.1) as usize;
            arr.set(y, x, '#').unwrap();
        }

        print_2d_array(&arr);
        println!("===========");
    }
}

fn main() {
    let input = read_file_to_string("input/day23.txt");
    let mut crater = Crater::from_str(&input).unwrap();

    let number_of_empty_tiles_after_10_rounds = crater.simulate_steps(10);
    println!(
        "The number of empty tiles after 10 rounds is {number_of_empty_tiles_after_10_rounds}"
    );

    let mut crater = Crater::from_str(&input).unwrap();
    let rounds_until_finished = crater.simulate_until_finished();
    println!("The number of rounds before the process is finished is {rounds_until_finished}");
}
