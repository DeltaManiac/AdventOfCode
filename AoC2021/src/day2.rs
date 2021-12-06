pub fn solve_part1(input: &str) -> u32 {
    let mut h = 0;
    let mut d = 0;
    input.lines().for_each(|line| {
        let split = line.split_once(' ').unwrap();
        let arg = split.1.parse::<u32>().unwrap();

        match split.0 {
            "forward" => h += arg,
            "down" => d += arg,
            "up" => d -= arg,
            _ => {}
        }
    });
    h * d
}

pub fn solve_part2(input: &str) -> u32 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    input.lines().for_each(|line| {
        let split = line.split_once(' ').unwrap();
        let arg = split.1.parse::<u32>().unwrap();

        match split.0 {
            "forward" => {
                h += arg;
                d += aim * arg;
            }
            "down" => aim += arg,
            "up" => aim -= arg,
            _ => {}
        }
    });

    h * d
}
