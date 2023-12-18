use crate::beam::{Beam, Direction};

pub fn run(input: &str) {
    let tiles = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max = 0;
    for x in 0..tiles[0].len() {
        max = *[
            max,
            crate::first::calculate_energised_tiles(
                &tiles,
                &mut vec![Beam {
                    direction: Direction::Down,
                    x,
                    y: 0,
                }],
            ),
            crate::first::calculate_energised_tiles(
                &tiles,
                &mut vec![Beam {
                    direction: Direction::Up,
                    x,
                    y: tiles.len() - 1,
                }],
            ),
        ]
        .iter()
        .max()
        .unwrap();
    }
    for y in 0..tiles.len() {
        max = *[
            max,
            crate::first::calculate_energised_tiles(
                &tiles,
                &mut vec![Beam {
                    direction: Direction::Right,
                    x: 0,
                    y,
                }],
            ),
            crate::first::calculate_energised_tiles(
                &tiles,
                &mut vec![Beam {
                    direction: Direction::Left,
                    x: tiles[0].len() - 1,
                    y,
                }],
            ),
        ]
        .iter()
        .max()
        .unwrap();
    }

    println!("{}", max);
}
