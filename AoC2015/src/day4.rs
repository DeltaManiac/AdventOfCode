#[aoc(day4, part1)]
pub fn part1(input: &str) -> String {
    for i in 1..std::u32::MAX {
        let mut s = String::from(input);
        s.push_str(&i.to_string()[..]);
        let sha = md5::compute(&s);
        if sha[0] as i32 + sha[1] as i32 + (sha[2] >> 4) as i32 == 0 {
            return i.to_string();
        }
    }
    "NIL".to_string()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> String {
    for i in 1..std::u32::MAX {
        let mut s = String::from(input);
        s.push_str(&i.to_string()[..]);
        let sha = md5::compute(&s);
        if sha[0] as i32 + sha[1] as i32 + sha[2] as i32 == 0 {
            return i.to_string();
        }
    }
    "NIL".to_string()
}
