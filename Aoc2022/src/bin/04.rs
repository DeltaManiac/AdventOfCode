use itertools::Itertools;
pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let a = line.split(',').collect_vec();
        let b = a[0].split('-').collect_vec();
        let (x1, y1): (u32, u32) = (b[0].parse().unwrap(), b[1].parse().unwrap());
        let c = a[1].split('-').collect_vec();
        let (x2, y2): (u32, u32) = (c[0].parse().unwrap(), c[1].parse().unwrap());
        if x1 >= x2 && y1 <= y2 || x2 >= x1 && y2 <= y1 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let a = line.split(',').collect_vec();
        let b = a[0].split('-').collect_vec();
        let (x1, y1): (u32, u32) = (b[0].parse().unwrap(), b[1].parse().unwrap());
        let c = a[1].split('-').collect_vec();
        let (x2, y2): (u32, u32) = (c[0].parse().unwrap(), c[1].parse().unwrap());
        if !(y1 < x2 || x1 > y2) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
