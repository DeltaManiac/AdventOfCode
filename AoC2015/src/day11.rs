fn condition_3(string: &str) -> bool {
    !string.chars().any(|c| match c {
        'i' | 'o' | 'l' => true,
        _ => false,
    })
}

fn condition_2(string: &str) -> bool {
    string
        .as_bytes()
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn condition_1(string: &str) -> bool {
    let a = string.as_bytes().iter();
    let b = string.as_bytes()[1..].iter();
    let mut i = a.zip(b);
    let mut c = 0;
    while c < 2 {
        if let Some((x, y)) = i.next() {
            if x == y {
                c += 1;
                i.next();
                i.next();
            }
        } else {
            break;
        }
    }
    c >= 2
}

fn next(string: &str) -> String {
    let mut result = String::new();
    for (i, c) in string.chars().rev().enumerate() {
        match c {
            'z' => result.push('a'),
            _ => {
                result.push(((c as u8) + 1) as char);
                result.extend(
                    string
                        .chars()
                        .rev()
                        .skip(i + 1)
                        .take(string.len() - i)
                        .collect::<Vec<_>>(),
                );
                break;
            }
        }
    }
    result.chars().rev().collect::<String>()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let mut password = input.to_string();

    while !(condition_1(&password) && condition_3(&password) && condition_2(&password)) {
        password = next(&password);
    }
    password
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let mut password = part1(input);
    password = next(&password);
    while !(condition_1(&password) && condition_3(&password) && condition_2(&password)) {
        password = next(&password);
    }
    password
}
