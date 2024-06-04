use std::collections::HashSet;

struct Block {
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
    z: usize,
    height: usize,
}

impl Block {
    pub(crate) fn from_str(s: &str) -> Self {
        let vals = s
            .split('~')
            .map(|half| {
                half.split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        Block {
            x_start: vals[0],
            x_end: vals[3],
            y_start: vals[1],
            y_end: vals[4],
            z: vals[2],
            height: vals[5] - vals[2] + 1,
        }
    }
}

pub fn run(input: &str) {
    let mut blocks = input
        .lines()
        .map(|l| Block::from_str(l))
        .collect::<Vec<_>>();

    let mut heights = vec![vec![(1, -1); 10]; 10];
    blocks.sort_unstable_by_key(|b| b.z);
    let mut invalid = HashSet::new();
    for (i, block) in blocks.iter_mut().enumerate() {
        let mut max = 1;
        let mut standing_on = HashSet::new();
        for x in block.x_start..=block.x_end {
            for y in block.y_start..=block.y_end {
                if heights[x][y].0 > max {
                    max = heights[x][y].0;
                    standing_on.clear();
                }
                if heights[x][y].0 == max {
                    standing_on.insert(heights[x][y].1);
                }
            }
        }
        block.z = max;
        let standing_on = standing_on.iter().collect::<Vec<_>>();
        if standing_on.len() == 1 && *standing_on[0] != -1 {
            invalid.insert(standing_on[0].clone());
        }

        for x in block.x_start..=block.x_end {
            for y in block.y_start..=block.y_end {
                heights[x][y] = (block.z + block.height, i as isize);
            }
        }
    }

    println!("{}", blocks.len() - invalid.len());
}
