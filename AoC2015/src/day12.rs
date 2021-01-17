use serde_json::Value;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    let mut value = 0i32;
    let mut tmp_val = 0i32;
    let mut last: char = '\x00';
    let mut multiplier = 1;
    for ch in input.chars() {
        value += match ch {
            '0'...'9' => {
                if last == '-' {
                    multiplier = -1;
                }
                tmp_val = tmp_val * 10
                    + match ch.to_string().parse::<i32>() {
                        Ok(x) => x,
                        Err(e) => panic!("Help! {}", e),
                    };

                0
            }
            _ if last.is_digit(10) => {
                let tmp = tmp_val * multiplier;
                tmp_val = 0;
                multiplier = 1;

                tmp
            }
            _ => 0,
        };
        last = ch;
    }
    value
}

#[aoc(day12, part1, Serde)]
pub fn part1_serde(input: &str) -> i64 {
    let a = serde_json::from_str(&input).unwrap();
    sum(a, false)
}

#[aoc(day12, part2, Serde)]
pub fn part2(input: &str) -> i64 {
    let a = serde_json::from_str(&input).unwrap();
    sum(a, false)
}

fn sum(v: Value, use_red: bool) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.into_iter().map(|e| sum(e, use_red)).sum(),
        Value::Object(v) => {
            let mut max = 0;
            for v in v.values() {
                if (v == "red") && !use_red {
                    return 0;
                }
                max += sum(v.clone(), use_red);
            }
            return max;
        }
    }
}
