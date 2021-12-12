use std::collections::{BTreeSet, HashMap};

pub fn input_generator(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut successors = HashMap::new();
    input.lines().for_each(|l| {
        let (start, end) = l.split_once('-').unwrap();
        successors.entry(start).or_insert_with(Vec::new).push(end);
        successors.entry(end).or_insert_with(Vec::new).push(start);
    });
    successors
}

fn possible_paths<'a>(
    node: &'a str,
    successors: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut BTreeSet<&'a str>,
    traverse_again: bool,
) -> usize {
    let s = seen.contains(&node);
    match node {
        "end" => 1,
        _ if s && !traverse_again => 0,
        _ => {
            let inserted = !s && node.as_bytes()[0] >= b'a' && seen.insert(node);
            let count = successors[&node]
                .iter()
                .filter(|&&n| n != "start")
                .map(|&n| possible_paths(n, successors, seen, traverse_again && !s))
                .sum();
            if inserted {
                seen.remove(&node);
            }
            count
        }
    }
}

pub fn solve_part1(input: &HashMap<&str, Vec<&str>>) -> usize {
    possible_paths("start", input, &mut BTreeSet::new(), false)
}

pub fn solve_part2(input: &HashMap<&str, Vec<&str>>) -> usize {
    possible_paths("start", input, &mut BTreeSet::new(), true)
}
