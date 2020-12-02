pub struct Policy {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Policy {

    pub fn is_valid_count(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count();
        if count >= self.min && count <= self.max {
            return true;
        }
        false
    }

    pub fn is_valid_pos(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<char>>();
        match (chars.get(self.min - 1), chars.get(self.max - 1)) {
            (Some(x), Some(y)) => {
                (*x == self.character && *y != self.character)
                    || (*x != self.character && *y == self.character)
            }
            (Some(x), None) | (None, Some(x)) => *x == self.character,
            _ => false,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Policy> {
    input
        .lines()
        .map(|l| {
            let mut items = l.split_whitespace().into_iter();
            let mut min_max = items.next().unwrap().split('-');
            let min = min_max.next().unwrap().parse::<usize>().unwrap();
            let max = min_max.next().unwrap().parse::<usize>().unwrap();
            let character = items.next().unwrap().chars().next().unwrap();
            let password = items.next().unwrap().to_string();
            Policy {
                min,
                max,
                character,
                password,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Policy]) -> usize {
    input.iter().filter(|p| p.is_valid_count()).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Policy]) -> usize {
    input.iter().filter(|p| p.is_valid_pos()).count()
}
