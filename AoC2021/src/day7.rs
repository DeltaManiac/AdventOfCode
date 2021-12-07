pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn solve_part1(input: &Vec<i64>) -> i64 {
    (0..*input.iter().max().unwrap())
        .map(|c| input.iter().map(|v| (c - *v).abs()).sum())
        .min()
        .unwrap()
}

pub fn solve_part2(input: &Vec<i64>) -> i64 {
    (0..*input.iter().max().unwrap())
        .map(|c| {
            input
                .iter()
                .map(|v| (c - *v).abs())
                .map(|b| b * (b + 1) / 2)
                .sum()
        })
        .min()
        .unwrap()
}
