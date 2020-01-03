#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|w| !(w.contains("ab") || w.contains("cd") || w.contains("pq") || w.contains("xy")))
        .filter(|x| {
            x.chars()
                .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
                .count()
                > 2
        })
        .filter(|y| {
            let mut c = y.chars().collect::<Vec<char>>();
            let l = c.len();
            c.dedup();
            l != c.len()
        })
        .count()
}

fn repeat_xx(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || repeat_xx(&string[1..])
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|y| repeat_xx(y))
        .filter(|z| z.chars().zip(z.chars().skip(2)).any(|(a, b)| a == b))
        .count()
}
