use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2 as usize);
        for c in first.chars() {
            if second.contains(c) {
                total += score(c);
                break;
            }
        }
    }
    Some(total)
}

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => (c as u32 - 'A' as u32) + 27,
        c => panic!("{c}"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let lines: Vec<_> = input.lines().collect();
    for (first, second, third) in lines.iter().tuples() {
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                total += score(c);
                break;
            }
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
