use crate::beam::{Beam, Direction};
use std::collections::HashSet;

pub fn run(input: &str) {
    let tiles = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut beams = vec![Beam {
        direction: Direction::Right,
        x: 0,
        y: 0,
    }];

    let energised_tiles = calculate_energised_tiles(&tiles, &mut beams);

    println!("{}", energised_tiles);
}

pub(crate) fn calculate_energised_tiles(tiles: &[Vec<char>], beams: &mut Vec<Beam>) -> usize {
    let mut seen_beams = HashSet::new();
    while !beams.is_empty() {
        *beams = step_beams(beams, tiles, &mut seen_beams);
    }

    let mut energised_tiles = HashSet::new();
    for beam in seen_beams {
        if !energised_tiles.contains(&(beam.x, beam.y)) {
            energised_tiles.insert((beam.x, beam.y));
        }
    }
    energised_tiles.len()
}

fn step_beams(beams: &[Beam], tiles: &[Vec<char>], seen_beams: &mut HashSet<Beam>) -> Vec<Beam> {
    let mut new_beams = vec![];
    for beam in beams {
        if seen_beams.contains(beam) {
            continue;
        } else {
            seen_beams.insert(*beam);
        }
        let tile = tiles[beam.y][beam.x];
        match beam.direction {
            Direction::Up | Direction::Down if tile == '-' => {
                if let Some(left_beam) = walk_beam(
                    Beam {
                        direction: Direction::Left,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(left_beam);
                }
                if let Some(right_beam) = walk_beam(
                    Beam {
                        direction: Direction::Right,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(right_beam);
                }
            }
            Direction::Left | Direction::Right if tile == '|' => {
                if let Some(up_beam) = walk_beam(
                    Beam {
                        direction: Direction::Up,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(up_beam);
                }
                if let Some(down_beam) = walk_beam(
                    Beam {
                        direction: Direction::Down,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(down_beam);
                }
            }
            Direction::Up if tile == '/' => {
                if let Some(right_beam) = walk_beam(
                    Beam {
                        direction: Direction::Right,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(right_beam);
                }
            }
            Direction::Up if tile == '\\' => {
                if let Some(left_beam) = walk_beam(
                    Beam {
                        direction: Direction::Left,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(left_beam);
                }
            }
            Direction::Down if tile == '/' => {
                if let Some(left_beam) = walk_beam(
                    Beam {
                        direction: Direction::Left,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(left_beam);
                }
            }
            Direction::Down if tile == '\\' => {
                if let Some(right_beam) = walk_beam(
                    Beam {
                        direction: Direction::Right,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(right_beam);
                }
            }
            Direction::Left if tile == '/' => {
                if let Some(down_beam) = walk_beam(
                    Beam {
                        direction: Direction::Down,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(down_beam);
                }
            }
            Direction::Left if tile == '\\' => {
                if let Some(up_beam) = walk_beam(
                    Beam {
                        direction: Direction::Up,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(up_beam);
                }
            }
            Direction::Right if tile == '/' => {
                if let Some(up_beam) = walk_beam(
                    Beam {
                        direction: Direction::Up,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(up_beam);
                }
            }
            Direction::Right if tile == '\\' => {
                if let Some(down_beam) = walk_beam(
                    Beam {
                        direction: Direction::Down,
                        ..*beam
                    },
                    tiles[0].len(),
                    tiles.len(),
                ) {
                    new_beams.push(down_beam);
                }
            }
            _ => {
                if let Some(next_beam) = walk_beam(*beam, tiles[0].len(), tiles.len()) {
                    new_beams.push(next_beam);
                }
            }
        }
    }
    new_beams
}

fn walk_beam(beam: Beam, width: usize, height: usize) -> Option<Beam> {
    match beam.direction {
        Direction::Up => {
            if beam.y > 0 {
                return Some(Beam {
                    y: beam.y - 1,
                    ..beam
                });
            }
        }
        Direction::Down => {
            if beam.y < height - 1 {
                return Some(Beam {
                    y: beam.y + 1,
                    ..beam
                });
            }
        }
        Direction::Left => {
            if beam.x > 0 {
                return Some(Beam {
                    x: beam.x - 1,
                    ..beam
                });
            }
        }
        Direction::Right => {
            if beam.x < width - 1 {
                return Some(Beam {
                    x: beam.x + 1,
                    ..beam
                });
            }
        }
    }
    None
}
