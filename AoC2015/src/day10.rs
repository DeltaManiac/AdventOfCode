#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..40 {
        let mut v: Vec<(u16, char)> = Vec::new();
        let mut iter = s.chars();
        let mut curr = (1, iter.next().unwrap());
        for i in iter {
            if i != curr.1 {
                v.push(curr);
                curr = (1, i);
            } else {
                curr.0 += 1;
            }
        }
        v.push(curr);
        s.clear();
        for i in v {
            s.push_str(&i.0.to_string());
            s.push(i.1)
        }
    }
    s.chars().count()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..50 {
        let mut v: Vec<(u16, char)> = Vec::new();
        let mut iter = s.chars();
        let mut curr = (1, iter.next().unwrap());
        for i in iter {
            if i != curr.1 {
                v.push(curr);
                curr = (1, i);
            } else {
                curr.0 += 1;
            }
        }
        v.push(curr);
        s.clear();
        for i in v {
            s.push_str(&i.0.to_string());
            s.push(i.1)
        }
    }
    s.chars().count()
}
