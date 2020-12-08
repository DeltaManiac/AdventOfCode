use std::collections::HashSet;
pub struct CPU {
    acc: i32,
    ip: isize,
}

type Instruction = (String, i32);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| -> Instruction {
            let i = l.split(' ').collect::<Vec<&str>>();
            (
                i.get(0).unwrap().to_string(),
                i.get(1).unwrap().parse::<i32>().unwrap_or(0),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut cpu = CPU { acc: 0, ip: 0 };
    let mut executed: HashSet<isize> = HashSet::new();

    while !executed.contains(&cpu.ip) {
        let instruction = &input[cpu.ip as usize];
        match (instruction.0.as_str(), instruction.1) {
            ("acc", val) => {
                cpu.acc += val;
                executed.insert(cpu.ip);
                cpu.ip += 1;
            }
            ("jmp", val) => {
                executed.insert(cpu.ip);
                cpu.ip += val as isize;
            }
            _ => {
                executed.insert(cpu.ip);
                cpu.ip += 1;
            }
        }
    }
    cpu.acc
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    for (idx, instr) in input.iter().enumerate() {
        let mut prg = Vec::new();
        prg.clone_from(&input.to_vec());
        if instr.0.as_str() == "jmp" {
            prg[idx].0 = "nop".to_string();
        }
        if instr.0.as_str() == "nop" {
            prg[idx].0 = "jmp".to_string();
        }

        let mut executed: HashSet<isize> = HashSet::new();
        let mut cpu = CPU { acc: 0, ip: 0 };
        while cpu.ip < prg.len() as isize && !executed.contains(&cpu.ip) {
            let instruction = &prg[cpu.ip as usize];
            match (instruction.0.as_str(), instruction.1) {
                ("acc", val) => {
                    cpu.acc += val;
                    executed.insert(cpu.ip);
                    cpu.ip += 1;
                }
                ("jmp", val) => {
                    executed.insert(cpu.ip);
                    cpu.ip += val as isize;
                }
                _ => {
                    executed.insert(cpu.ip);
                    cpu.ip += 1;
                }
            }
            if cpu.ip == prg.len() as isize {
                return cpu.acc;
            }
        }
    }
    0
}
