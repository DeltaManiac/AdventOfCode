#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    for i in 25..input.len() {
        let prev = &input[i - 25..i];
        let mut f = false;
        for n in 0..25 {
            for m in 0..25 {
                if prev[n] + prev[m] == input[i] {
                    f = true
                }
            }
        }
        if !f {
            return input[i];
        }
    }
    0
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    for i in 0..input.len() {
        for j in i + 2..input.len() {
            let range = &input[i..j];
            if range.into_iter().sum::<u64>() == 144381670 {
                return range.into_iter().min().unwrap() + range.into_iter().max().unwrap();
            }
        }
    }
    0
}
