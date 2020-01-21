use std::collections::HashMap;
#[derive(Debug)]
pub enum Gate {
    Value(u16, String),
    Source(String, String),
    And(String, String, String),
    AndValue(u16, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

#[aoc_generator(day7)]
pub fn input_to_struct(input: &str) -> Vec<Gate> {
    input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            if words[0] == "NOT" {
                Gate::Not(words[1].to_string(), words[3].to_string())
            } else if words[1] == "LSHIFT" {
                Gate::LShift(
                    words[0].to_string(),
                    words[2].to_string().trim().parse::<u16>().unwrap(),
                    words[4].to_string(),
                )
            } else if words[1] == "RSHIFT" {
                Gate::RShift(
                    words[0].to_string(),
                    words[2].to_string().trim().parse::<u16>().unwrap(),
                    words[4].to_string(),
                )
            } else if words[0].to_string().trim().parse::<i64>().is_ok() && words[1] == "AND" {
                Gate::AndValue(
                    words[0].to_string().trim().parse::<u16>().unwrap(),
                    words[2].to_string(),
                    words[4].to_string(),
                )
            } else if words[1] == "AND" {
                Gate::And(
                    words[0].to_string(),
                    words[2].to_string(),
                    words[4].to_string(),
                )
            } else if words[1] == "OR" {
                Gate::Or(
                    words[0].to_string(),
                    words[2].to_string(),
                    words[4].to_string(),
                )
            } else if words[0].to_string().trim().parse::<i64>().is_ok() && words[1] == "->" {
                Gate::Value(
                    words[0].to_string().trim().parse::<u16>().unwrap(),
                    words[2].to_string(),
                )
            } else if words[1] == "->" {
                Gate::Source(words[0].to_string(), words[2].to_string())
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<Gate>>()
}

fn solve_circuit(gates: &[Gate], values: &mut HashMap<String, u16>) -> HashMap<String, u16> {
    let mut unresolved = true;
    let constants = values.clone();
    while unresolved {
        let values_prev = values.clone();
        gates.iter().for_each(|item| {
            for (k, v) in &constants {
                values.insert(k.clone(), *v);
            }
            match item {
                Gate::Value(ref num, ref dst) => {
                    values.insert(dst.to_string(), *num);
                }
                Gate::Source(ref src0, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0);
                    }
                }
                Gate::And(ref src0, ref src1, ref dst) => {
                    if values.contains_key(src0) && values.contains_key(src1) {
                        let v0 = *values.get(src0).unwrap();
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), v0 & v1);
                    }
                }
                Gate::AndValue(ref v0, ref src1, ref dst) => {
                    if values.contains_key(src1) {
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), *v0 & v1);
                    }
                }
                Gate::Or(ref src0, ref src1, ref dst) => {
                    if values.contains_key(src0) && values.contains_key(src1) {
                        let v0 = *values.get(src0).unwrap();
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), v0 | v1);
                    }
                }
                Gate::LShift(ref src0, ref num, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0 << *num);
                    }
                }
                Gate::RShift(ref src0, ref num, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0 >> *num);
                    }
                }
                Gate::Not(ref src0, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), !v0);
                    }
                }
            }
            unresolved = values_prev != *values;
        });
    }
    values.clone()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Gate]) -> u16 {
    *solve_circuit(input, &mut HashMap::new()).get("a").unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Gate]) -> u16 {
    let mut map = HashMap::new();
    map.insert(
        "b".to_string(),
        *solve_circuit(&input, &mut HashMap::new()).get("a").unwrap(),
    );
    *solve_circuit(&input, &mut map).get("a").unwrap()
}
