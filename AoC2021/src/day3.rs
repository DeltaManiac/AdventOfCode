#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0..lines[0].len() {
        gamma <<= 1;
        epsilon <<= 1;
        let ones = lines
            .iter()
            .map(|x| x.chars().nth(pos).unwrap())
            .filter(|x| *x == '1')
            .count();
        if ones >= (lines.len() - ones) {
            gamma += 1
        } else {
            epsilon += 2
        }
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    part2_counter(&lines.clone(), true) * part2_counter(&lines.clone(), false)
}

fn part2_counter(lines: &Vec<&str>, most: bool) -> i32 {
    let mut pos = 0;
    let mut values = lines.clone();
    while values.len() > 1 {
        let ones_count = values
            .iter()
            .map(|x| x.chars().nth(pos).unwrap())
            .filter(|x| *x == '1')
            .count();
        if ones_count >= (values.len() - ones_count) {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '1' } else { '0' })
        } else {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '0' } else { '1' })
        }
        pos += 1
    }
    i32::from_str_radix(values[0], 2).unwrap()
}
