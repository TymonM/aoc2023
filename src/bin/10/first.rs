use crate::pipe::Pipe;

pub fn run(input: &str) {
    let (mut pipe_map, starting_coords) = Pipe::read_pipe_map(input);

    let (mut forward_direction, mut back_direction) =
        Pipe::extrapolate_start(&mut pipe_map, starting_coords);

    let mut forward_coords = starting_coords;
    let mut back_coords = starting_coords;
    let mut count = 0;
    loop {
        count += 1;

        let (x, y) = Pipe::step(forward_coords, forward_direction);
        forward_coords = (x as usize, y as usize);
        if forward_coords.0 == back_coords.0 && forward_coords.1 == back_coords.1 {
            break;
        }
        forward_direction = pipe_map[forward_coords.1][forward_coords.0]
            .unwrap()
            .other(forward_direction.opposite())
            .unwrap();

        let (x, y) = Pipe::step(back_coords, back_direction);
        back_coords = (x as usize, y as usize);
        if forward_coords.0 == back_coords.0 && forward_coords.1 == back_coords.1 {
            break;
        }
        back_direction = pipe_map[back_coords.1][back_coords.0]
            .unwrap()
            .other(back_direction.opposite())
            .unwrap();
    }

    println!("{}", count);
}
