use std::collections::HashSet;
use itertools::Itertools;
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1, 2sum)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let set: HashSet<u32> = input.iter().cloned().collect();
    for x in set.iter() {
        if set.get(&(2020 - x)).is_some() {
            return *set.get(&(2020 - x)).unwrap() * *x;
        }
    }
    0
}

#[aoc(day1, part2, 3sum)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut nums: Vec<u32> = input.iter().cloned().collect();
    nums.sort();
    for i in 0..(nums.len() - 2) {
        let a = nums[i];
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let b = nums[left];
            let c = nums[right];
            if a + b + c == 2020 {
                return a * b * c;
            }
            if a + b + c < 2020 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    0
}


#[aoc(day1, part1, combinator)]
pub fn combinator_part1(input: &[u32]) -> u32 {
    input.iter().copied().combinations(2).find(|v| v.iter().sum::<u32>()==2020).unwrap().iter().product()
}

#[aoc(day1, part2, combinator)]
pub fn combinator_part2(input: &[u32]) -> u32 {
    input.iter().copied().combinations(3).find(|v| v.iter().sum::<u32>()==2020).unwrap().iter().product()
}
