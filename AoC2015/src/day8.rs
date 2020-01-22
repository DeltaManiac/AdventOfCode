#[aoc(day8, part1)]
pub fn part1(input: &str) -> u16 {
    let mut mem_count: Vec<u16> = Vec::new();
    let mut char_count: Vec<u16> = Vec::new();
    input.lines().for_each(|line| {
        char_count.push(line.chars().count() as u16);
        let mut chars = line.chars();
        let mut count: u16 = 0;
        chars.next();
        loop {
            match chars.next() {
                Some(c) => match c {
                    '\"' => {}
                    '\\' => match chars.next() {
                        Some(c) => {
                            if c == 'x' {
                                chars.next();
                                chars.next();
                                count += 1;
                            } else if c == '"' || c == '\\' {
                                count += 1;
                            }
                        }
                        None => (),
                    },
                    _ => {
                        count += 1;
                    }
                },
                None => break,
            }
        }
        mem_count.push(count);
    });
    char_count.iter().sum::<u16>() - mem_count.iter().sum::<u16>()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u16 {
    let mut mem_count: Vec<u16> = Vec::new();
    let mut char_count: Vec<u16> = Vec::new();
    input.lines().for_each(|line| {
        char_count.push(line.chars().count() as u16);
        let mut chars = line.chars();
        let mut esc_count: u16 = 2;
        loop {
            match chars.next() {
                Some(c) => match c {
                    '\"' => {
                        esc_count += 1;
                    }
                    '\\' => match chars.next() {
                        Some(c) => {
                            if c == 'x' {
                                chars.next();
                                chars.next();
                                esc_count += 1;
                            } else if c == '"' || c == '\\' {
                                esc_count += 2;
                            }
                        }
                        None => (),
                    },
                    _ => {}
                },
                None => break,
            }
        }
        mem_count.push(line.chars().count() as u16 + esc_count);
    });
    mem_count.iter().sum::<u16>() - char_count.iter().sum::<u16>()
}
