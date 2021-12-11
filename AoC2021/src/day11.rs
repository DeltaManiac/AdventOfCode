use itertools::Itertools;
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect_vec())
        .collect_vec()
}

pub fn solve_part1(input: &Vec<Vec<u8>>) -> u32 {
    let mut grid = input.clone();
    let mut flashes = 0;
    for _step in 0..100 {
        let mut q = std::collections::VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    mark[i][j] = true;
                    q.push_back((i as i32, j as i32));
                    flashes += 1;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0 && x < 10 && y >= 0 && y < 10 && !mark[x as usize][y as usize] {
                    grid[x as usize][y as usize] += 1;
                    if grid[x as usize][y as usize] > 9 {
                        grid[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        q.push_back((x, y));
                        flashes += 1;
                    }
                }
            }
        }
    }
    flashes
}

pub fn solve_part2(input: &Vec<Vec<u8>>) -> u32 {
    let mut grid = input.clone();
    for step in 1.. {
        let mut q = std::collections::VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        let mut flashes = 0;
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    mark[i][j] = true;
                    q.push_back((i as i32, j as i32));
                    flashes += 1;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0 && x < 10 && y >= 0 && y < 10 && !mark[x as usize][y as usize] {
                    grid[x as usize][y as usize] += 1;
                    if grid[x as usize][y as usize] > 9 {
                        grid[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        q.push_back((x, y));
                        flashes += 1;
                    }
                }
            }
        }
        if flashes == 100 {
            return step;
        }
    }
    0
}
