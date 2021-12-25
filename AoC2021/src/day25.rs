use itertools::Itertools;

fn step(map: &mut Vec<Vec<u8>>, dr: usize, dc: usize) -> bool {
    let (R, C) = (map.len(), map[0].len());
    let mut map2 = vec![vec![b'.'; C]; R];
    let mut moved = false;
    for (r, c) in (0..R).cartesian_product(0..C) {
        match map[r][c] {
            b'>' if map[r][(c + dc) % C] == b'.' => {
                map2[r][(c + dc) % C] = b'>';
                moved = true;
            }
            b'v' if map[(r + dr) % R][c] == b'.' => {
                map2[(r + dr) % R][c] = b'v';
                moved = true;
            }
            b'>' => map2[r][c] = b'>',
            b'v' => map2[r][c] = b'v',
            _ => {}
        }
    }
    *map = map2;
    moved
}

pub fn solve_part1(input: &Vec<Vec<u8>>) -> usize {
    let mut map = input.clone();
    let mut round = 0;
    loop {
        let move1 = step(&mut map, 0, 1);
        let move2 = step(&mut map, 1, 0);
        round += 1;
        if !move1 && !move2 {
            break;
        }
    }
    round
}

pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}
