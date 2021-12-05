use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

type Coord = (i32, i32);
#[derive(Debug)]
pub struct Data {
    starts: Vec<Coord>,
    ends: Vec<Coord>,
}

type Input = Data;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Data {
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    input.lines().for_each(|line| {
        let split = line.split_once(" -> ").unwrap();
        let s = split.0.split_once(',').unwrap();
        let x_start = s.0.parse::<i32>().unwrap();
        let y_start = s.1.parse::<i32>().unwrap();
        let s1 = split.1.split_once(',').unwrap();
        let x_end = s1.0.parse::<i32>().unwrap();
        let y_end = s1.1.parse::<i32>().unwrap();
        starts.push((x_start, y_start));
        ends.push((x_end, y_end));
    });

    Data { starts, ends }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut counter = HashMap::new();
    let walker = input.starts.iter().zip(input.ends.iter());
    for ((x1, y1), (x2, y2)) in walker {
        if x1 == x2 {
            for y in *min(y1, y2)..*max(y1, y2) + 1 {
                counter.insert((*x1, y), counter.get(&(*x1, y)).unwrap_or(&0) + 1);
            }
        } else {
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 {
                1
            } else if y1 > y2 {
                -1
            } else {
                0
            };
            let mut x = *x1;
            let mut y = *y1;
            while (x <= *x2 && x_step > 0) || (x >= *x2 && x_step < 0) {
                if y_step == 0 {
                    counter.insert((x, y), counter.get(&(x, y)).unwrap_or(&0) + 1);
                }
                x += x_step;
                y += y_step;
            }
        }
    }
    counter.iter().filter(|(_, v)| **v > 1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut counter = HashMap::new();
    let walker = input.starts.iter().zip(input.ends.iter());
    for ((x1, y1), (x2, y2)) in walker {
        if x1 == x2 {
            for y in *min(y1, y2)..*max(y1, y2) + 1 {
                counter.insert((*x1, y), counter.get(&(*x1, y)).unwrap_or(&0) + 1);
            }
        } else {
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 {
                1
            } else if y1 > y2 {
                -1
            } else {
                0
            };
            let mut x = *x1;
            let mut y = *y1;
            while (x <= *x2 && x_step > 0) || (x >= *x2 && x_step < 0) {
                counter.insert((x, y), counter.get(&(x, y)).unwrap_or(&0) + 1);
                x += x_step;
                y += y_step;
            }
        }
    }
    counter.iter().filter(|(_, v)| **v > 1).count()
}
