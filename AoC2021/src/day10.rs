use std::usize;

use itertools::Itertools;
const OPENING: [u8; 4] = [b'(', b'[', b'{', b'<'];
const CLOSING: [u8; 4] = [b')', b']', b'}', b'>'];
const POINTS1: [usize; 4] = [3, 57, 1197, 25137];

pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec()
}

fn parse(line: &Vec<u8>) -> Result<Vec<&u8>, &u8> {
    let mut stack = Vec::new();
    for item in line {
        if OPENING.contains(item) {
            stack.push(item)
        } else {
            let k = stack.pop().unwrap();
            if item != &CLOSING[OPENING.iter().position(|x| x == k).unwrap()] {
                return Err(item);
            }
        }
    }
    Ok(stack)
}

pub fn solve_part1(input: &Vec<Vec<u8>>) -> usize {
    input
        .iter()
        .filter_map(|l| parse(l).err())
        .map(|v| POINTS1[CLOSING.iter().position(|x| x == v).unwrap()] as usize)
        .sum()
}

pub fn solve_part2(input: &Vec<Vec<u8>>) -> usize {
    let v = input
        .iter()
        .filter_map(|e| parse(e).ok())
        .map(|stack| {
            stack.iter().rev().fold(0, |acc, el| {
                5 * acc + OPENING.iter().position(|x| &x == el).unwrap() + 1
            })
        })
        .sorted()
        .collect_vec();
    v[v.len() / 2]
}
