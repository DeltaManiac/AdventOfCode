use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}
pub fn input_generator(input: &str) -> (usize, usize, Vec<Vec<i64>>) {
    let data: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    (data.len(), data[0].len(), data)
}

pub fn solve_part1(input: &(usize, usize, Vec<Vec<i64>>)) -> i64 {
    let num_cols = input.1;
    let num_rows = input.0;
    let height_map = &input.2;

    let mut low_points: Vec<i64> = Vec::new();
    for row in 0..num_rows {
        for col in 0..num_cols {
            let height = height_map[row][col];

            // upper neighbor
            if row > 0 && height >= height_map[row - 1][col] {
                continue;
            }

            // lower neighbor
            if row < num_rows - 1 && height >= height_map[row + 1][col] {
                continue;
            }

            // left neighbor
            if col > 0 && height >= height_map[row][col - 1] {
                continue;
            }

            // right neighbor
            if col < num_cols - 1 && height >= height_map[row][col + 1] {
                continue;
            }

            low_points.push(height);
        }
    }
    low_points.iter().map(|h| h + 1).sum::<i64>()
}

pub fn solve_part2(input: &(usize, usize, Vec<Vec<i64>>)) -> usize {
    let num_rows = input.0;
    let num_cols = input.1;
    let height_map = &input.2;
    let mut low_points: Vec<Point> = Vec::new();
    for row in 0..num_rows {
        for col in 0..num_cols {
            let height = height_map[row][col];

            // upper neighbor
            if row > 0 && height >= height_map[row - 1][col] {
                continue;
            }

            // lower neighbor
            if row < num_rows - 1 && height >= height_map[row + 1][col] {
                continue;
            }

            // left neighbor
            if col > 0 && height >= height_map[row][col - 1] {
                continue;
            }

            // right neighbor
            if col < num_cols - 1 && height >= height_map[row][col + 1] {
                continue;
            }

            low_points.push(Point { row, col });
        }
    }

    let mut basin_sizes: Vec<usize> = Vec::new();
    for p in low_points.iter() {
        let mut basin_points: HashSet<Point> = HashSet::new();
        let mut q: VecDeque<Point> = VecDeque::new();
        q.push_back(*p);
        basin_points.insert(*p);

        while q.len() > 0 {
            let cur = q.pop_front().unwrap();

            let row = cur.row;
            let col = cur.col;
            let mut neighbors: Vec<Point> = Vec::new();

            // upper neighbor
            if row > 0 {
                neighbors.push(Point { row: row - 1, col })
            }

            // lower neighbor
            if row < num_rows - 1 {
                neighbors.push(Point { row: row + 1, col })
            }

            // left neighbor
            if col > 0 {
                neighbors.push(Point { row, col: col - 1 })
            }

            // right neighbor
            if col < num_cols - 1 {
                neighbors.push(Point { row, col: col + 1 })
            }

            for n in neighbors {
                if basin_points.contains(&n) || height_map[n.row][n.col] == 9 {
                    continue;
                }

                q.push_back(n);
                basin_points.insert(n);
            }
        }

        basin_sizes.push(basin_points.len());
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}
