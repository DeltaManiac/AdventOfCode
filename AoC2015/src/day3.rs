use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut last_pos = (0, 0);
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    map.insert(last_pos, 0);
    input.chars().for_each(|d| {
        last_pos = match d {
            '^' => (last_pos.0 + 1, last_pos.1),
            'v' => (last_pos.0 - 1, last_pos.1),
            '>' => (last_pos.0, last_pos.1 + 1),
            '<' => (last_pos.0, last_pos.1 - 1),
            _ => unreachable!(),
        };
        map.entry(last_pos).and_modify(|x| *x += 1).or_insert(1);
    });
    map.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut last_pos = (0, 0);
    let mut last_pos_clone = (0, 0);
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    map.insert(last_pos, 0);
    input.chars().enumerate().for_each(|(i, d)| {
        if i % 2 == 0 {
            last_pos = match d {
                '^' => (last_pos.0 + 1, last_pos.1),
                'v' => (last_pos.0 - 1, last_pos.1),
                '>' => (last_pos.0, last_pos.1 + 1),
                '<' => (last_pos.0, last_pos.1 - 1),
                _ => unreachable!(),
            };
            map.entry(last_pos).and_modify(|x| *x += 1).or_insert(1);
        } else {
            last_pos_clone = match d {
                '^' => (last_pos_clone.0 + 1, last_pos_clone.1),
                'v' => (last_pos_clone.0 - 1, last_pos_clone.1),
                '>' => (last_pos_clone.0, last_pos_clone.1 + 1),
                '<' => (last_pos_clone.0, last_pos_clone.1 - 1),
                _ => unreachable!(),
            };
            map.entry(last_pos_clone)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    });
    map.len()
}
