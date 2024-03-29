use itertools::Itertools;
use std::collections::HashMap;

pub fn input_generator(input: &str) -> Vec<Vec<Instruction>> {
    let insts = input
        .lines()
        .map(|l| {
            let src = match Source::from_str(&l[4..5]) {
                Source::Reg(src) => src,
                _ => unreachable!(),
            };
            match &l[..3] {
                "inp" => Instruction::Inp(src),
                "add" => Instruction::Add(src, Source::from_str(&l[6..])),
                "mul" => Instruction::Mul(src, Source::from_str(&l[6..])),
                "div" => Instruction::Div(src, Source::from_str(&l[6..])),
                "mod" => Instruction::Mod(src, Source::from_str(&l[6..])),
                "eql" => Instruction::Eql(src, Source::from_str(&l[6..])),
                _ => unreachable!(),
            }
        })
        .collect_vec();
    insts
        .chunks(18)
        .map(|c| c.iter().skip(1).copied().collect())
        .collect::<Vec<_>>()
}

pub fn solve_part1(input: &Vec<Vec<Instruction>>) -> String {
    solve(input, true)
}

pub fn solve_part2(input: &Vec<Vec<Instruction>>) -> String {
    solve(input, false)
}

pub type Cache = HashMap<(i64, usize), Option<i64>>;

#[derive(Clone, Copy)]
pub enum Source {
    Reg(usize),
    Val(i64),
}

impl Source {
    fn from_str(s: &str) -> Self {
        match s {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _ => Self::Val(s.parse().unwrap()),
        }
    }

    fn val(&self, regs: &[i64; 4]) -> i64 {
        match *self {
            Self::Reg(i) => regs[i],
            Self::Val(v) => v,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Inp(usize),
    Add(usize, Source),
    Mul(usize, Source),
    Div(usize, Source),
    Mod(usize, Source),
    Eql(usize, Source),
}

fn find_modelnum(
    memo: &mut Cache,
    blocks: &[Vec<Instruction>],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if let Some(&answer) = memo.get(&(z, block)) {
        return answer;
    }

    for &digit in range {
        let mut regs = [digit, 0, 0, z];
        for &inst in &blocks[block] {
            match inst {
                Instruction::Add(a, b) => regs[a] += b.val(&regs),
                Instruction::Mul(a, b) => regs[a] *= b.val(&regs),
                Instruction::Div(a, b) => regs[a] /= b.val(&regs),
                Instruction::Mod(a, b) => regs[a] %= b.val(&regs),
                Instruction::Eql(a, b) => regs[a] = (regs[a] == b.val(&regs)) as i64,
                Instruction::Inp(_) => unreachable!(),
            }
        }
        let z = regs[3];
        if block + 1 == blocks.len() {
            if z == 0 {
                memo.insert((z, block), Some(digit));
                return Some(digit);
            }
            continue;
        }
        if let Some(best) = find_modelnum(memo, blocks, block + 1, z, range) {
            memo.insert((z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    memo.insert((z, block), None);
    None
}

fn solve(blocks: &[Vec<Instruction>], biggest: bool) -> String {
    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    let answer = find_modelnum(&mut Cache::new(), &blocks, 0, 0, &range).unwrap();
    answer.to_string().chars().rev().collect()
}
