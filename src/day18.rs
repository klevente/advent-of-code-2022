use advent_of_code_2022::read_file_to_string;
use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;
use std::{collections::HashSet, str::FromStr};

type Coords = (i32, i32, i32);

fn calculate_bounding_coords(coords: &HashSet<Coords>) -> (i32, i32) {
    match coords
        .iter()
        .flat_map(|(x, y, z)| [*x, *y, *z].into_iter())
        .minmax()
    {
        MinMaxResult::NoElements => (-1, 1),
        MinMaxResult::OneElement(n) => (n - 1, n + 1),
        MinMaxResult::MinMax(min, max) => (min - 1, max + 1),
    }
}

fn get_neighbours_in_bounds(coords: &Coords, min: i32, max: i32) -> Vec<Coords> {
    let mut neighbours = Vec::new();
    let (x, y, z) = coords.clone();

    if x > min {
        neighbours.push((x - 1, y, z));
    }
    if x < max {
        neighbours.push((x + 1, y, z));
    }
    if y > min {
        neighbours.push((x, y - 1, z));
    }
    if y < max {
        neighbours.push((x, y + 1, z));
    }
    if z > min {
        neighbours.push((x, y, z - 1));
    }
    if z < max {
        neighbours.push((x, y, z + 1));
    }

    neighbours
}

struct Pond {
    droplets: HashSet<Coords>,
    min: i32,
    max: i32,
}

impl FromStr for Pond {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let droplets = s
            .lines()
            .map(|l| scanf!(l, "{i32},{i32},{i32}"))
            .collect::<Result<HashSet<_>, _>>()
            .map_err(|_| "Invalid format".to_string())?;

        let (min, max) = calculate_bounding_coords(&droplets);

        Ok(Self { droplets, min, max })
    }
}

impl Pond {
    fn calculate_area(&self) -> u32 {
        self.droplets
            .iter()
            .map(|c| {
                get_neighbours_in_bounds(c, self.min, self.max)
                    .iter()
                    .filter(|n| !self.droplets.contains(n))
                    .count()
            })
            .sum::<usize>() as u32
    }

    fn calculate_outside_area(&self) -> u32 {
        let mut visited_cubes = HashSet::new();
        let mut area = 0;
        // walk through the wireframe of the bounding cube and start flood filling
        for x in -1..=22 {
            area += self.flood_fill_from((x, self.min, self.min), &mut visited_cubes);
            area += self.flood_fill_from((x, self.max, self.min), &mut visited_cubes);
            area += self.flood_fill_from((x, self.min, self.max), &mut visited_cubes);
            area += self.flood_fill_from((x, self.max, self.max), &mut visited_cubes);
        }

        for y in -1..=22 {
            area += self.flood_fill_from((self.min, y, self.min), &mut visited_cubes);
            area += self.flood_fill_from((self.max, y, self.min), &mut visited_cubes);
            area += self.flood_fill_from((self.min, y, self.max), &mut visited_cubes);
            area += self.flood_fill_from((self.max, y, self.max), &mut visited_cubes);
        }

        for z in -1..=22 {
            area += self.flood_fill_from((self.min, self.min, z), &mut visited_cubes);
            area += self.flood_fill_from((self.max, self.min, z), &mut visited_cubes);
            area += self.flood_fill_from((self.min, self.max, z), &mut visited_cubes);
            area += self.flood_fill_from((self.max, self.max, z), &mut visited_cubes);
        }

        area
    }

    fn flood_fill_from(&self, current: Coords, visited_cubes: &mut HashSet<Coords>) -> u32 {
        if visited_cubes.contains(&current) {
            return 0;
        }
        visited_cubes.insert(current.clone());
        if self.droplets.contains(&current) {
            return 0;
        }

        let mut area = 0;
        let neighbours = get_neighbours_in_bounds(&current, self.min, self.max);
        for n in neighbours {
            if self.droplets.contains(&n) {
                area += 1;
            }
            area += self.flood_fill_from(n, visited_cubes);
        }
        area
    }
}

fn main() {
    let input = read_file_to_string("input/day18.txt");
    let pond = Pond::from_str(&input).unwrap();
    let area = pond.calculate_area();
    println!("The surface area of the scanned droplet is {area}");

    let area_without_air = pond.calculate_outside_area();
    println!("The exterior surface area of the scanned droplet is {area_without_air}");
}
