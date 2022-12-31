use advent_of_code_2022::{read_file_to_string, EMPTY_LINE_PATTERN};
use array2d::Array2D;
use std::ops::Range;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction {
    R,
    L,
    U,
    D,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::R),
            'L' => Ok(Self::L),
            'U' => Ok(Self::U),
            'D' => Ok(Self::D),
            _ => Err("Invalid format".to_string()),
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::R => Self::D,
            Self::L => Self::U,
            Self::U => Self::R,
            Self::D => Self::L,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::R => Self::U,
            Self::L => Self::D,
            Self::U => Self::L,
            Self::D => Self::R,
        }
    }

    fn get_value(&self) -> usize {
        match self {
            Self::R => 0,
            Self::L => 2,
            Self::U => 3,
            Self::D => 1,
        }
    }
}

#[derive(Debug)]
enum Step {
    Move(usize),
    Right,
    Left,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<usize>() {
            Ok(Self::Move(n))
        } else {
            let c = s.chars().next().ok_or("Invalid format".to_string())?;
            match c {
                'R' => Ok(Self::Right),
                'L' => Ok(Self::Left),
                _ => Err("Invalid format".to_string()),
            }
        }
    }
}

fn parse_steps(steps: &str) -> Vec<Step> {
    steps
        .split_inclusive(['R', 'L'])
        .flat_map(|s| {
            if s.contains(['R', 'L']) {
                let (a, b) = s.split_at(s.len() - 1);
                vec![a, b].into_iter()
            } else {
                vec![s].into_iter()
            }
        })
        .map(|s| Step::from_str(s).unwrap())
        .collect()
}

#[derive(Clone, Debug)]
enum Tile {
    Air,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Air),
            '#' => Ok(Self::Wall),
            _ => Err("Invalid format".to_string()),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Air => '.',
            Tile::Wall => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Player {
    global_x: isize,
    global_y: isize,
    local_x: isize,
    local_y: isize,
    direction: Direction,
    current_section: usize,
}

impl Player {
    fn new(global_x: isize, global_y: isize) -> Self {
        Self {
            global_x,
            global_y,
            local_x: 0,
            local_y: 0,
            direction: Direction::R,
            current_section: 0,
        }
    }

    fn calculate_final_password(&self) -> usize {
        let row = self.global_y as usize + 1;
        let column = self.global_x as usize + 1;
        let direction = self.direction.get_value();

        1000 * row + 4 * column + direction
    }
}

#[derive(Debug)]
struct Section {
    idx: usize,
    offset_x: isize,
    offset_y: isize,
    tiles: Array2D<Tile>,
}

impl Section {
    fn new(idx: usize, offset_x: isize, offset_y: isize, tiles: Array2D<Tile>) -> Self {
        Self {
            idx,
            offset_x,
            offset_y,
            tiles,
        }
    }

    fn get_tile(&self, x: isize, y: isize) -> &Tile {
        self.tiles.get(y as usize, x as usize).unwrap()
    }

    fn get_horizontal_area(&self) -> Range<isize> {
        self.offset_x..(self.offset_x + self.tiles.row_len() as isize)
    }

    fn print(&self) {
        for row in self.tiles.rows_iter() {
            let padding = " ".repeat(self.offset_x as usize);
            print!("{padding}");
            for column in row.into_iter() {
                print!("{column}");
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    sections: Vec<Section>,
    player: Player,
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = Vec::new();
        let mut acc: Vec<&str> = Vec::new();

        let mut offset_y = 0;
        let mut width = 0;
        let mut prev_len = 0;
        for (i, l) in s.lines().enumerate() {
            let total_len = l.len();
            let tiles_len = l.trim().len();

            width = width.max(total_len);

            if prev_len != tiles_len && i != 0 {
                let offset_x = acc[0].chars().filter(|&c| c == ' ').count();
                let tiles = acc
                    .iter()
                    .map(|&line| {
                        line.trim()
                            .chars()
                            .map(|c| Tile::try_from(c).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                let tiles = Array2D::from_rows(&tiles).unwrap();
                let section = Section::new(sections.len(), offset_x as isize, offset_y, tiles);
                sections.push(section);
                acc = vec![l];
                offset_y = i as isize;
            } else {
                acc.push(l);
            }
            prev_len = tiles_len;
        }

        let offset_x = acc[0].chars().filter(|&c| c == ' ').count();
        let tiles = acc
            .iter()
            .map(|&line| {
                line.trim()
                    .chars()
                    .map(|c| Tile::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let tiles = Array2D::from_rows(&tiles).unwrap();
        let section = Section::new(sections.len(), offset_x as isize, offset_y, tiles);
        sections.push(section);

        let height = s.lines().count();

        let player = Player::new(sections[0].offset_x as isize, 0);

        Ok(Self {
            width,
            height,
            sections,
            player,
        })
    }
}

impl Board {
    fn execute_steps(&mut self, steps: &[Step]) -> usize {
        for step in steps {
            match step {
                Step::Move(n) => self.move_player(*n),
                Step::Right => self.player.direction = self.player.direction.turn_right(),
                Step::Left => self.player.direction = self.player.direction.turn_left(),
            }
        }

        self.player.calculate_final_password()
    }

    fn find_downwards_furthest_section_in_range(&self) -> usize {
        let mut res = self.player.current_section;
        for i in (self.player.current_section + 1)..self.sections.len() {
            if self.sections[i]
                .get_horizontal_area()
                .contains(&self.player.global_x)
            {
                res = i;
            } else {
                break;
            }
        }
        res
    }

    fn find_upwards_furthest_section_in_range(&self) -> usize {
        let mut res = self.player.current_section;
        for i in (0..self.player.current_section).rev() {
            if self.sections[i]
                .get_horizontal_area()
                .contains(&self.player.global_x)
            {
                res = i;
            } else {
                break;
            }
        }
        res
    }

    fn move_player(&mut self, n: usize) {
        let dir = self.player.direction;
        for i in 0..n {
            match dir {
                Direction::R => {
                    let mut x = self.player.local_x + 1;
                    let y = self.player.local_y;
                    if x == self.sections[self.player.current_section].tiles.row_len() as isize {
                        x = 0;
                    }
                    let tile = self.sections[self.player.current_section].get_tile(x, y);
                    match tile {
                        Tile::Air => {
                            self.player.local_x = x;
                            self.player.global_x =
                                self.sections[self.player.current_section].offset_x + x;
                        }
                        Tile::Wall => return,
                    }
                }
                Direction::L => {
                    let mut x = self.player.local_x - 1;
                    let y = self.player.local_y;
                    if x == -1 {
                        x = (self.sections[self.player.current_section].tiles.row_len() - 1)
                            as isize;
                    }
                    let tile = self.sections[self.player.current_section].get_tile(x, y);
                    match tile {
                        Tile::Air => {
                            self.player.local_x = x;
                            self.player.global_x =
                                self.sections[self.player.current_section].offset_x + x;
                        }
                        Tile::Wall => return,
                    }
                }
                Direction::U => {
                    let mut x = self.player.local_x;
                    let mut y = self.player.local_y - 1;
                    let mut section_idx = self.player.current_section;
                    if y == -1 {
                        let section_to_wrap_around_to =
                            self.find_downwards_furthest_section_in_range();
                        if section_idx > 0
                            && self.sections[section_idx - 1]
                                .get_horizontal_area()
                                .contains(&self.player.global_x)
                        {
                            section_idx -= 1;
                        } else if section_to_wrap_around_to != section_idx {
                            section_idx = section_to_wrap_around_to;
                        }
                        x = self.player.global_x - self.sections[section_idx].offset_x;
                        y = (self.sections[section_idx].tiles.column_len() - 1) as isize;
                    }
                    let tile = self.sections[section_idx].get_tile(x, y);
                    match tile {
                        Tile::Air => {
                            self.player.local_x = x;

                            self.player.current_section = section_idx;
                            self.player.local_y = y;
                            self.player.global_y = self.sections[section_idx].offset_y + y;
                        }
                        Tile::Wall => return,
                    }
                }
                Direction::D => {
                    let mut x = self.player.local_x;
                    let mut y = self.player.local_y + 1;
                    let mut section_idx = self.player.current_section;
                    if y == self.sections[self.player.current_section]
                        .tiles
                        .column_len() as isize
                    {
                        let section_to_wrap_around_to =
                            self.find_upwards_furthest_section_in_range();
                        if section_idx < self.sections.len() - 1
                            && self.sections[section_idx + 1]
                                .get_horizontal_area()
                                .contains(&self.player.global_x)
                        {
                            section_idx += 1;
                        } else if section_to_wrap_around_to != section_idx {
                            section_idx = section_to_wrap_around_to;
                        }

                        x = self.player.global_x - self.sections[section_idx].offset_x;
                        y = 0;
                    }
                    let tile = self.sections[section_idx].get_tile(x, y);
                    match tile {
                        Tile::Air => {
                            self.player.local_x = x;

                            self.player.current_section = section_idx;
                            self.player.local_y = y;
                            self.player.global_y = self.sections[section_idx].offset_y + y;
                        }
                        Tile::Wall => return,
                    }
                }
            }
        }
    }

    fn print(&self) {
        for s in &self.sections {
            s.print();
        }
    }
}

fn main() {
    let input = read_file_to_string("input/day22.txt");
    let (board, moves) = input.split_once(EMPTY_LINE_PATTERN).unwrap();
    let mut board = Board::from_str(board).unwrap();
    board.print();

    let steps = parse_steps(moves);

    let result = board.execute_steps(&steps);
    dbg!(result);
}
