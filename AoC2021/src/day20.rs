use std::collections::HashMap;

pub type Image = HashMap<(isize, isize), i8>;

pub fn input_generator(input: &str) -> (Vec<i8>, Image) {
    let lines: Vec<Vec<i8>> = input
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let algorithm = lines[0].clone();

    let map = &lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), *c))
                .collect::<Vec<_>>()
        })
        .collect::<Image>();
    (algorithm, map.to_owned())
}

pub fn solve_part1(input: &(Vec<i8>, Image)) -> usize {
    let mut image = input.1.clone();
    for i in 0..2 {
        image = enhance(infinity(&input.0, i), &input.0, &image);
    }
    image.iter().filter(|(_, v)| **v == 1).count()
}

pub fn solve_part2(input: &(Vec<i8>, Image)) -> usize {
    let mut image = input.1.clone();
    for i in 0..50 {
        image = enhance(infinity(&input.0, i), &input.0, &image);
    }
    image.iter().filter(|(_, v)| **v == 1).count()
}

fn enhance(i: i8, algorithm: &[i8], image: &Image) -> Image {
    let mut output = Image::new();
    let min = image.iter().min_by_key(|(k, _)| *k).unwrap().0;
    let max = image.iter().max_by_key(|(k, _)| *k).unwrap().0;
    for y in (min.1 - 1)..=(max.1 + 1) {
        for x in (min.0 - 1)..=(max.0 + 1) {
            *output.entry((x, y)).or_default() = output_pixel(i, algorithm, image, (x, y))
        }
    }
    output
}

fn output_pixel(i: i8, algorithm: &[i8], image: &Image, pixel: (isize, isize)) -> i8 {
    let (x, y) = pixel;
    let mut value: isize = 0;
    for (x, y) in [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        value = (value << 1) | (image.get(&(x, y)).unwrap_or(&i) & 1) as isize;
    }
    algorithm[value as usize]
}

fn infinity(algorithm: &[i8], i: i8) -> i8 {
    match algorithm[0] {
        0 => 0,
        1 => i % 2,
        _ => unreachable!(),
    }
}
