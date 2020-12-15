use std::collections::HashMap;
use std::collections::VecDeque;
// struct Counter {
//     last_turn : Some<usize>,
//     last_last_turn :Some<uszie>,
// }
#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
let nums = input.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

let mut seen: HashMap<usize, VecDeque<usize>> = HashMap::new();

// starting numbers
for (idx, &num) in nums.iter().enumerate() {
    let e = seen.entry(num).or_default();
    e.push_back(idx + 1); // turns start at 1
}

let mut last = *nums.last().unwrap();
let turn = nums.len() + 1;
for turn in turn..=2020 {
    let e = seen.entry(last).or_default();
    let value = if e.len() == 2 { e[1] - e[0] } else { 0 };
    // add value to seen
    let e = seen.entry(value).or_default();
    e.push_back(turn);
    if e.len() > 2 {
        e.pop_front();
    }
    last = value;
}
last

}


#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
let nums = input.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

let mut seen: HashMap<usize, VecDeque<usize>> = HashMap::new();

// starting numbers
for (idx, &num) in nums.iter().enumerate() {
    let e = seen.entry(num).or_default();
    e.push_back(idx + 1); // turns start at 1
}

let mut last = *nums.last().unwrap();
let turn = nums.len() + 1;
for turn in turn..=30000000 {
    let e = seen.entry(last).or_default();
    let value = if e.len() == 2 { e[1] - e[0] } else { 0 };
    // add value to seen
    let e = seen.entry(value).or_default();
    e.push_back(turn);
    if e.len() > 2 {
        e.pop_front();
    }
    last = value;
}
last

}