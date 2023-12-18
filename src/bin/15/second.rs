use crate::first::hash;
use arr_macro::arr;

struct Lens {
    label: String,
    focal_length: usize,
}

pub fn run(input: &str) {
    let mut boxes = arr![vec![]; 256];
    for instruction in input.split(',') {
        if let Some(label) = instruction.strip_suffix('-') {
            remove(label, &mut boxes);
        } else {
            let splits = instruction.split('=').collect::<Vec<&str>>();
            insert(splits[0], splits[1].parse().unwrap(), &mut boxes);
        }
    }

    let boxes = boxes; // immutable now
    let mut sum = 0;
    for (box_number, lens_box) in boxes.iter().enumerate() {
        for (lens_index, lens) in lens_box.iter().enumerate() {
            sum += focusing_power(box_number, lens_index, lens.focal_length);
        }
    }

    println!("{}", sum);
}

fn remove(label: &str, map: &mut [Vec<Lens>; 256]) {
    let target_box = hash(label);
    map[target_box].retain(|lens| lens.label != label);
}

fn insert(label: &str, focal_length: usize, map: &mut [Vec<Lens>; 256]) {
    let target_box = hash(label);
    if let Some(lens) = map[target_box].iter_mut().find(|l| l.label == label) {
        lens.focal_length = focal_length;
    } else {
        map[target_box].push(Lens {
            label: label.to_string(),
            focal_length,
        });
    }
}

fn focusing_power(box_number: usize, index: usize, focal_length: usize) -> usize {
    (box_number + 1) * (index + 1) * focal_length
}
