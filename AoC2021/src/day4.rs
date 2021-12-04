use nalgebra::Matrix5;

pub type Sheet = Matrix5<(u32, bool)>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<u32> {
    solve_problem(&parse_input(input))
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Sheet>) {
    let mut lines = input.split("\n\n");
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let sheets = lines
        .map(|sheet| {
            Matrix5::from_iterator(sheet.lines().flat_map(|line| {
                line.split_whitespace()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .map(|x| (x, false))
            }))
        })
        .collect::<Vec<Sheet>>();

    (numbers, sheets)
}

fn solve_problem(input: &(Vec<u32>, Vec<Sheet>)) -> Vec<u32> {
    let mut sheets = input.1.clone();
    let mut result = Vec::new();
    let mut completed_sheets_idx = std::collections::HashSet::new();
    // Update the sheets
    for n in &input.0 {
        sheets
            .iter_mut()
            .enumerate()
            .for_each(|(sheet_idx, sheet)| {
                if !completed_sheets_idx.contains(&sheet_idx) {
                    sheet.iter_mut().for_each(|num| {
                        if num.0 == *n {
                            num.1 = true
                        }
                    });
                    let col_win = sheet
                        .column_iter()
                        .any(|col| col.iter().all(|(_, set)| *set));
                    let row_win = sheet.row_iter().any(|row| row.iter().all(|(_, set)| *set));
                    // Check if the sheet won
                    if row_win || col_win {
                        completed_sheets_idx.insert(sheet_idx);
                        //calculate the sum of nor marked
                        let mut sum = 0;
                        for item in sheet.into_iter() {
                            if !item.1 {
                                sum += item.0
                            }
                        }
                        result.push(sum * n);
                    }
                }
            })
    }
    result.to_owned()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<u32>) -> u32 {
    *input.first().unwrap_or(&0)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<u32>) -> u32 {
    *input.last().unwrap_or(&0)
}
