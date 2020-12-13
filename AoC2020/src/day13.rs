#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let lines = input
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let start_time = lines.get(0).unwrap().parse::<i64>().unwrap();
    (start_time..)
        .find_map(|t| {
            lines
                .get(1)
                .unwrap()
                .split(',')
                .filter(|c| *c != "x")
                .map(|x| x.parse::<i64>().unwrap())
                .find(|b| t % (*b) == 0)
                .map(|b| (t, b))
        })
        .map(|(i, b)| b * (i - start_time))
        .unwrap()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let lines = input
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mods = lines
        .get(1)
        .unwrap()
        .split(',')
        .filter(|c| *c != "x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let res = lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, c)| *c != "x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .map(|(i, b)| b - i)
        .collect::<Vec<_>>();
    chinese_remainder(&res, &mods).unwrap()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
