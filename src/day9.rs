use advent_of_code_2022::read_file_lines_as;
use sscanf::scanf;
use std::{collections::HashSet, str::FromStr};

type Coords = (i32, i32);

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            'U' => Self::U,
            'D' => Self::D,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Movement {
    dir: Direction,
    n: u32,
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n): (char, u32) =
            scanf!(s, "{char} {u32}").map_err(|_| "Invalid format".to_string())?;
        let dir = Direction::from_char(d);
        Ok(Self { dir, n })
    }
}

struct Rope<const N: usize> {
    knot_positions: [Coords; N],
    tail_visited_positions: HashSet<(i32, i32)>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knot_positions: [(0, 0); N],
            tail_visited_positions: HashSet::new(),
        }
    }

    fn simulate_movement(&mut self, movement: &Movement) {
        for _ in 0..movement.n {
            self.move_once(&movement.dir);
        }
    }

    fn move_once(&mut self, dir: &Direction) {
        let head_movement = Self::get_head_movement_vector(dir);
        self.knot_positions[0].0 += head_movement.0;
        self.knot_positions[0].1 += head_movement.1;

        for i in 1..N {
            let tail_movement = Self::get_tail_movement_vector(
                &self.knot_positions[i - 1],
                &self.knot_positions[i],
            );
            self.knot_positions[i].0 += tail_movement.0;
            self.knot_positions[i].1 += tail_movement.1;
        }

        self.tail_visited_positions
            .insert(self.knot_positions[N - 1]);
    }

    fn get_head_movement_vector(dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
            Direction::U => (0, 1),
            Direction::D => (0, -1),
        }
    }

    /*
     * Tail movement pattern is the following (square is the head):
     * ↘ ↘ ↓ ↙ ↙
     * ↘ · · · ↙
     * → · ■ · ←
     * ↗ · · · ↖
     * ↗ ↗ ↑ ↖ ↖
     */
    fn get_tail_movement_vector(head: &Coords, tail: &Coords) -> (i32, i32) {
        if tail.0 == head.0 - 2 {
            if tail.1 < head.1 {
                (1, 1)
            } else if tail.1 > head.1 {
                (1, -1)
            } else {
                (1, 0)
            }
        } else if tail.0 == head.0 + 2 {
            if tail.1 < head.1 {
                (-1, 1)
            } else if tail.1 > head.1 {
                (-1, -1)
            } else {
                (-1, 0)
            }
        } else if tail.1 == head.1 - 2 {
            if tail.0 < head.0 {
                (1, 1)
            } else if tail.0 > head.0 {
                (-1, 1)
            } else {
                (0, 1)
            }
        } else if tail.1 == head.1 + 2 {
            if tail.0 < head.0 {
                (1, -1)
            } else if tail.0 > head.0 {
                (-1, -1)
            } else {
                (0, -1)
            }
        } else {
            (0, 0)
        }
    }

    fn get_number_of_visited_positions_by_tail(&self) -> u32 {
        self.tail_visited_positions.len() as u32
    }
}

fn main() {
    let movements = read_file_lines_as("input/day9.txt", |l| Movement::from_str(l).unwrap());
    let mut short_rope = Rope::<2>::new();

    for m in &movements {
        short_rope.simulate_movement(m);
    }
    let number_of_positions_tail_visited_short_rope =
        short_rope.get_number_of_visited_positions_by_tail();
    println!("The number of positions the tail of the short rope visited at least once is {number_of_positions_tail_visited_short_rope}");

    let mut long_rope = Rope::<10>::new();
    for m in &movements {
        long_rope.simulate_movement(m);
    }
    let number_of_positions_tail_visited_long_rope =
        long_rope.get_number_of_visited_positions_by_tail();
    println!("The number of positions the tail of the long rope visited at least once is {number_of_positions_tail_visited_long_rope}");
}
