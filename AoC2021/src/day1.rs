use itertools::Itertools;

//
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

//
pub fn solve_part1(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| b > a).count()
}
//

pub fn solve_part2(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}
