use std::collections::BinaryHeap;

pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn shortest_path(maze: &Vec<Vec<i32>>) -> i32 {
    let goal = (maze.len() - 1, maze[0].len() - 1);
    let mut dist = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
    let mut q = BinaryHeap::new();
    q.push((0, 0, 0));
    while let Some((cost, x, y)) = q.pop() {
        if (x, y) == goal {
            return -cost;
        }
        if -cost > dist[x][y] {
            continue;
        }
        for (x1, y1) in [
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ] {
            if maze.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }
            let next_cost = -cost + maze[x1][y1];
            if next_cost < dist[x1][y1] {
                q.push((-next_cost, x1, y1));
                dist[x1][y1] = next_cost;
            }
        }
    }
    unreachable!()
}

pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    let result = shortest_path(input);
    return result;
}

pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {
    let expanded = (0..(5 * input.len()))
        .map(|x| {
            (0..(5 * input[0].len()))
                .map(|y| {
                    let xlevel = (x / input.len()) as i32;
                    let ylevel = (y / input[0].len()) as i32;
                    let cost = input[x % input.len()][y % input[0].len()] + xlevel + ylevel;
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = shortest_path(&expanded);
    return result;
}
