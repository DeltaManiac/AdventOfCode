#[aoc_generator(day5)]
pub fn generate(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let mut row = 0;
            for part in [64, 32, 16, 8, 4, 2, 1].iter() {
                if chars.next() == Some('B') {
                    row += part;
                }
            }
            let mut col = 0;
            for part in [5, 2, 1].iter() {
                if chars.next() == Some('R') {
                    col += part;
                }
            }
            row * 8 + col
        })
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    let mut i = input.to_vec();
    i.sort();
    for (&s1, &s2) in i.iter().zip(i[1..].iter()) {
        if s2 - s1 == 1 {
            return s2;
        }
    }
    0
}
