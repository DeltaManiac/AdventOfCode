use itertools::Itertools;
pub fn solve_part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, num) = line.split(" | ").collect_tuple().unwrap();
            num.split_whitespace()
                .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let i = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" | ").unwrap();
            let c = a.split_whitespace().collect();
            let d = b.split_whitespace().collect();
            (c, d)
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    i.iter()
        .map(|x| {
            "abcdefg"
                .chars()
                .permutations(7)
                .find_map(|p| {
                    let not_valid = x.0.iter().map(|s| show_digit(&p, s)).any(|y| y.is_none());
                    if not_valid {
                        return None;
                    }
                    let ans: usize =
                        x.1.iter()
                            .rev()
                            .enumerate()
                            .map(|(i, s)| show_digit(&p, s).unwrap() * 10usize.pow(i as u32))
                            .sum();
                    return Some(ans);
                })
                .unwrap()
        })
        .sum::<usize>()
}

fn show_digit(perm: &[char], s: &str) -> Option<usize> {
    let decoded = s
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();
    let digit = match decoded.as_str() {
        "abcdefg" => 8,
        "bcdef" => 5,
        "acdfg" => 2,
        "abcdf" => 3,
        "abd" => 7,
        "abcdef" => 9,
        "bcdefg" => 6,
        "abef" => 4,
        "abcdeg" => 0,
        "ab" => 1,
        _ => return None,
    };
    Some(digit)
}
