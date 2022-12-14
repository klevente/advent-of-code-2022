use advent_of_code_2022::{print_2d_array, read_file_to_string};
use array2d::Array2D;
use itertools::{Itertools, MinMaxResult};

type Coords = (usize, usize);

#[derive(Clone)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Air => '.',
            Self::Rock => '#',
            Self::Sand => 'o',
        };
        write!(f, "{}", c)
    }
}

struct Cave {
    layout: Array2D<Tile>,
    top_left: Coords,
    bottom_right: Coords,
}

const SAND_STARTING_POSITION: Coords = (500, 0);

impl Cave {
    fn from_str_with_abyss(s: &str) -> Self {
        let parsed_line_strips = Self::parse_line_strips(s);

        let (x_min, x_max) = {
            match parsed_line_strips
                .iter()
                .flatten()
                .map(|(x, _)| *x)
                .minmax()
            {
                MinMaxResult::MinMax(min, max) => (min, max),
                MinMaxResult::OneElement(n) => (n, n),
                _ => unreachable!(),
            }
        };

        let y_max = parsed_line_strips
            .iter()
            .flatten()
            .map(|(_, y)| *y)
            .max()
            .unwrap();

        let num_rows = y_max + 2;
        let num_columns = x_max - x_min + 3;
        let top_left = (x_min - 1, 0);
        let bottom_right = (x_max + 1, y_max + 1);

        let layout = Self::build_layout_array_from_line_strips(
            &parsed_line_strips,
            num_rows,
            num_columns,
            &top_left,
        );

        Self {
            layout,
            top_left,
            bottom_right,
        }
    }

    fn from_str_with_floor(s: &str) -> Self {
        let parsed_line_strips = Self::parse_line_strips(s);

        let y_max = parsed_line_strips
            .iter()
            .flatten()
            .map(|(_, y)| *y)
            .max()
            .unwrap();

        let num_rows = y_max + 3;
        let num_columns = 2 * num_rows + 1;
        let top_left = (
            SAND_STARTING_POSITION.0 - num_rows,
            SAND_STARTING_POSITION.1,
        );
        let bottom_right = (SAND_STARTING_POSITION.0 + num_rows, num_rows - 1);

        let mut layout = Self::build_layout_array_from_line_strips(
            &parsed_line_strips,
            num_rows,
            num_columns,
            &top_left,
        );

        for x in 0..layout.num_columns() {
            let y = layout.num_rows() - 1;
            layout.set(y, x, Tile::Rock).unwrap();
        }

        Self {
            layout,
            top_left,
            bottom_right,
        }
    }

    fn parse_line_strips(s: &str) -> Vec<Vec<Coords>> {
        s.lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|coord| {
                        let (x, y) = coord.split_once(',').unwrap();
                        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                    })
                    .collect()
            })
            .collect()
    }

    fn build_layout_array_from_line_strips(
        line_strips: &Vec<Vec<Coords>>,
        num_rows: usize,
        num_columns: usize,
        top_left: &Coords,
    ) -> Array2D<Tile> {
        let mut layout = Array2D::filled_with(Tile::Air, num_rows, num_columns);

        for strip in line_strips {
            strip
                .iter()
                .tuple_windows::<(_, _)>()
                .for_each(|((x1, y1), (x2, y2))| {
                    let start_x = *x1.min(x2);
                    let end_x = *x1.max(x2);
                    let start_y = *y1.min(y2);
                    let end_y = *y1.max(y2);

                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            let y = y - top_left.1;
                            let x = x - top_left.0;
                            layout.set(y, x, Tile::Rock).unwrap();
                        }
                    }
                });
        }

        layout
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_position = SAND_STARTING_POSITION;
        loop {
            if self.is_at_bottom(sand_position) {
                return true;
            } else if self.is_air_at(sand_position.0, sand_position.1 + 1) {
                sand_position.1 += 1;
            } else if self.is_air_at(sand_position.0 - 1, sand_position.1 + 1) {
                sand_position.0 -= 1;
                sand_position.1 += 1;
            } else if self.is_air_at(sand_position.0 + 1, sand_position.1 + 1) {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                let x = sand_position.0 - self.top_left.0;
                let y = sand_position.1 - self.top_left.1;
                self.layout.set(y, x, Tile::Sand).unwrap();
                return false;
            }
        }
    }

    fn drop_sand_2(&mut self) -> bool {
        let mut sand_position = SAND_STARTING_POSITION;
        loop {
            if self.is_air_at(sand_position.0, sand_position.1 + 1) {
                sand_position.1 += 1;
            } else if self.is_air_at(sand_position.0 - 1, sand_position.1 + 1) {
                sand_position.0 -= 1;
                sand_position.1 += 1;
            } else if self.is_air_at(sand_position.0 + 1, sand_position.1 + 1) {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                let x = sand_position.0 - self.top_left.0;
                let y = sand_position.1 - self.top_left.1;
                self.layout.set(y, x, Tile::Sand).unwrap();

                return sand_position.0 == SAND_STARTING_POSITION.0
                    && sand_position.1 == SAND_STARTING_POSITION.1;
            }
        }
    }

    fn is_at_bottom(&self, pos: Coords) -> bool {
        pos.1 == self.bottom_right.1
    }

    fn is_air_at(&self, x: usize, y: usize) -> bool {
        let tile = self.get_tile_at(x, y);
        match tile {
            Tile::Air => true,
            _ => false,
        }
    }

    fn get_tile_at(&self, x: usize, y: usize) -> &Tile {
        let x = x - self.top_left.0;
        let y = y - self.top_left.1;
        self.layout.get(y, x).unwrap()
    }

    #[allow(dead_code)]
    fn print(&self) {
        print_2d_array(&self.layout);
    }
}

fn count_number_of_sand_until_it_starts_falling_into_abyss(input: &str) -> u32 {
    let mut cave = Cave::from_str_with_abyss(input);
    let mut cnt = 0;
    while !cave.drop_sand() {
        cnt += 1;
    }

    cnt
}

fn count_number_of_sand_until_it_stops_falling(input: &str) -> u32 {
    let mut cave = Cave::from_str_with_floor(input);
    let mut cnt = 1;
    while !cave.drop_sand_2() {
        cnt += 1;
    }

    cnt
}

fn main() {
    let input = read_file_to_string("input/day14.txt");

    let units_of_sand_at_rest_before_they_drop_into_abyss =
        count_number_of_sand_until_it_starts_falling_into_abyss(&input);
    println!("The number of units of sand that comes to rest before it starts falling into the abyss is {units_of_sand_at_rest_before_they_drop_into_abyss}");

    let units_of_sand_fallen_until_it_stops = count_number_of_sand_until_it_stops_falling(&input);
    println!("The number of units of sand that comes to rest before it stops falling is {units_of_sand_fallen_until_it_stops}");
}
