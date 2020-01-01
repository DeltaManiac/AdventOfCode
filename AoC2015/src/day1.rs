#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum: u32 = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => {
                if let Some(s) = sum.checked_sub(1) {
                    sum = s;
                } else {
                    return i + 1;
                }
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}
