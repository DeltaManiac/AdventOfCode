#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for l in input.lines() {
        map.push(l.chars().collect::<Vec<char>>().to_owned())
    }
    map
}

fn get_trees(dx: i32, dy: i32, input: &[Vec<char>]) -> u64 {
    let height = input.len() - 1;
    let width = input[0].len();
    let (mut x, mut y) = (dx, dy);
    let mut trees = 0;
    while y as usize <= height {
        if input[y as usize][x as usize] == '#' {
            trees += 1;
        }
        x += dx;
        if x >= width as i32 {
            x = x as i32 - width as i32;
        }
        y += dy;
    }
    trees
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u64 {
    get_trees(3, 1, input)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u64 {
    get_trees(1, 1, input)
        * get_trees(3, 1, input)
        * get_trees(5, 1, input)
        * get_trees(7, 1, input)
        * get_trees(1, 2, input)
}
