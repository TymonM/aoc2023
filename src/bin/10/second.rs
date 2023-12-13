use crate::pipe::{Direction, Pipe};

pub fn run(input: &str) {
    let (mut pipe_map, starting_coords) = Pipe::read_pipe_map(input);

    let (mut current_direction, _) = Pipe::extrapolate_start(&mut pipe_map, starting_coords);

    let mut in_loop = vec![vec![false; pipe_map[0].len()]; pipe_map.len()];

    let mut current_coords = starting_coords;
    loop {
        in_loop[current_coords.1][current_coords.0] = true;

        let (x, y) = Pipe::step(current_coords, current_direction);
        current_coords = (x as usize, y as usize);
        if current_coords.0 == starting_coords.0 && current_coords.1 == starting_coords.1 {
            break;
        }
        current_direction = pipe_map[current_coords.1][current_coords.0].clone().unwrap()
            .other(current_direction.opposite()).unwrap();
    }

    // debugging + cool art
    // Pipe::print_pipe_map(&pipe_map, &in_loop);

    let mut enclosed = 0;
    let mut y = 0;
    while y < pipe_map.len() {
        let mut x = 0;
        while x < pipe_map[y].len() {
            if !in_loop[y][x] {
                if check_enclosed(x, y, &pipe_map, &in_loop) {
                    // println!("{}, {} is enclosed", x, y);
                    enclosed += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    println!("{}", enclosed);
}

fn check_enclosed(x: usize, y: usize, pipe_map: &Vec<Vec<Option<Pipe>>>, in_loop: &Vec<Vec<bool>>) -> bool {
    let mut y2 = 0;
    let mut is_enclosed = false;
    while y2 < y {
        if in_loop[y2][x] &&
            (pipe_map[y2][x].unwrap().0 != Direction::Up || pipe_map[y2][x].unwrap().1 != Direction::Down) {
            match pipe_map[y2][x].unwrap().other(Direction::Right) {
                Some(_) => is_enclosed = ! is_enclosed,
                None => (),
            };
        }
        y2 += 1;
    }
    is_enclosed
}