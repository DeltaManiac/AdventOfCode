use std::collections::HashSet;
#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars())
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .fold(None, |a, l| {
                    let c: HashSet<_> = l.chars().collect();
                    if let Some(a) = a {
                        Some(c.intersection(&a).copied().collect())
                    } else {
                        Some(c)
                    }
                })
                .unwrap()
                .len()
        })
        .sum()
}
