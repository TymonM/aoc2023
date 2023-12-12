use crate::pipe::Direction::*;

const NEIGHBOURS: [((i32, i32), Direction); 4] = [
    ((0, 1), Direction::Up),
    ((-1, 0), Direction::Right),
    ((0, -1), Direction::Down),
    ((1, 0), Direction::Left),
];

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

#[derive(Copy, Clone)]
pub struct Pipe(pub Direction, pub Direction);

impl Pipe {
    pub fn from(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe(Up, Down)),
            '-' => Some(Pipe(Left, Right)),
            'L' => Some(Pipe(Up, Right)),
            'J' => Some(Pipe(Up, Left)),
            '7' => Some(Pipe(Down, Left)),
            'F' => Some(Pipe(Down, Right)),
            '.' | 'S' => None,
            _ => panic!("Invalid symbol!"),
        }
    }

    pub fn other(&self, direction: Direction) -> Option<Direction> {
        if self.0 == direction {
            Some(self.1)
        } else if self.1 == direction {
            Some(self.0)
        } else {
            None
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
}

impl Pipe {

    pub(crate) fn read_pipe_map(input: &str) -> (Vec<Vec<Option<Pipe>>>, (usize, usize)) {
        let mut pipe_map = vec![];
        let mut starting_coords = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    starting_coords = (x, y);
                }
                row.push(Pipe::from(c));
            }
            pipe_map.push(row);
        }
        (pipe_map, starting_coords)
    }

    pub(crate) fn extrapolate_start(pipe_map: &mut Vec<Vec<Option<Pipe>>>, starting_coords: (usize, usize)) -> (Direction, Direction) {
        let mut directions: [Direction; 2] = [Direction::Up, Direction::Up];
        let mut i = 0;
        for neighbour in NEIGHBOURS {
            let x = starting_coords.0 as i32 + neighbour.0.0;
            let y = starting_coords.1 as i32 + neighbour.0.1;
            if x < 0 || y < 0 || x >= pipe_map.len() as i32 || y >= pipe_map.len() as i32 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            match &pipe_map[y][x] {
                Some(pipe) => {
                    match pipe.other(neighbour.1) {
                        Some(_) => {
                            directions[i] = neighbour.1.opposite();
                            i += 1;
                        },
                        None => continue
                    }
                },
                None => continue,
            };
        }

        pipe_map[starting_coords.1][starting_coords.0] = Some(Pipe(directions[0], directions[1]));
        (directions[0], directions[1])
    }

    pub(crate) fn step(coords: (usize, usize), direction: Direction) -> (i32, i32) {
        for ((x, y), d) in NEIGHBOURS {
            if d == direction {
                return (coords.0 as i32 - x, coords.1 as i32 - y);
            }
        }
        panic!()
    }
}

// Printing is for debug only
impl Pipe {


    pub fn print(&self) -> char {
        match self {
            Pipe(Up, Down) => '║',
            Pipe(Left, Right) => '═',
            Pipe(Up, Right) => '╚',
            Pipe(Up, Left) => '╝',
            Pipe(Down, Left) => '╗',
            Pipe(Down, Right) => '╔',
            _ => '?'
        }
    }

    pub fn print_pipe_map(pipe_map: &Vec<Vec<Option<Pipe>>>, in_loop: &Vec<Vec<bool>>) {
        let mut y = 0;
        while y < pipe_map.len() {
            let mut x = 0;
            while x < pipe_map[y].len() {
                if in_loop[y][x] {
                    print!("{}", pipe_map[y][x].unwrap().print());
                } else {
                    print!(".");
                }
                x += 1;
            }
            print!("\n");
            y += 1;
        }
    }
}