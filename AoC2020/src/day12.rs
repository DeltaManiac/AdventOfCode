#![allow(unused_variables)]

pub enum Ins {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|l| match &l[..1] {
            "N" => Ins::N(l[1..].parse::<i32>().unwrap()),
            "E" => Ins::E(l[1..].parse::<i32>().unwrap()),
            "S" => Ins::S(l[1..].parse::<i32>().unwrap()),
            "W" => Ins::W(l[1..].parse::<i32>().unwrap()),
            "L" => Ins::L(l[1..].parse::<i32>().unwrap()),
            "R" => Ins::R(l[1..].parse::<i32>().unwrap()),
            "F" => Ins::F(l[1..].parse::<i32>().unwrap()),
            _ => panic!("{} is not a valid instruction", l),
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(inp: &[Ins]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut r = 90;
    for ins in inp {
        match ins {
            Ins::N(u) => {
                y -= u;
            }
            Ins::E(u) => {
                x += u;
            }
            Ins::S(u) => {
                y += u;
            }
            Ins::W(u) => {
                x -= u;
            }
            Ins::L(u) => {
                r -= u;
                if r < 0 {
                    r += 360;
                }
            }
            Ins::R(u) => {
                r += u;
                if r >= 360 {
                    r -= 360;
                }
            }
            Ins::F(u) => match r {
                0 => y -= u,
                90 => x += u,
                180 => y += u,
                270 => x -= u,
                _ => panic!("the ship can't be rotated {} degrees.", r),
            },
        }
    }
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn part2(inp: &[Ins]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = -1;
    for ins in inp {
        match ins {
            Ins::N(u) => {
                wy -= u;
            }
            Ins::E(u) => {
                wx += u;
            }
            Ins::S(u) => {
                wy += u;
            }
            Ins::W(u) => {
                wx -= u;
            }
            Ins::L(u) => {
                for _ in 0..(u / 90) {
                    let temp = wy;
                    wy = -wx;
                    wx = temp;
                }
            }
            Ins::R(u) => {
                for _ in 0..(u / 90) {
                    let temp = wy;
                    wy = wx;
                    wx = -temp;
                }
            }
            Ins::F(u) => {
                x += wx * u;
                y += wy * u;
            }
        }
    }
    x.abs() + y.abs()
}
