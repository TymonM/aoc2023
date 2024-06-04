pub fn run(input: &str) {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let start_coords = map
        .iter()
        .enumerate()
        .map(|(y, row)| (row.iter().position(|c| *c == 'S').unwrap_or(0), y))
        .max_by_key(|x| x.0)
        .unwrap();

    let count = reachable_spaces(&mut map, &start_coords, 64);
    // for row in map {
    //     println!("{}", row.iter().collect::<String>());
    // }
    println!("{}", count);
}

pub fn reachable_spaces(
    map: &mut [Vec<char>],
    start_coords: &(usize, usize),
    steps: usize,
) -> usize {
    let mut count = 0;
    let mut boundary = vec![*start_coords];
    for i in 0..steps {
        let mut new_boundary = vec![];
        for tile in &boundary {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (next_x, next_y) = (tile.0 as i32 + dx, tile.1 as i32 + dy);
                if next_x < 0
                    || next_y < 0
                    || next_x >= map[0].len() as i32
                    || next_y >= map.len() as i32
                {
                    continue;
                }
                let tile = &mut map[next_y as usize][next_x as usize];
                if *tile == '#' || *tile == 'O' {
                    continue;
                }
                *tile = 'O';
                new_boundary.push((next_x as usize, next_y as usize));
                if i % 2 != steps % 2 {
                    count += 1;
                }
            }
        }
        boundary = new_boundary;
    }
    count
}
