// pub fn part_one(input: &str) -> Option<u32> {
//     let mut max = 0;
//     let mut temp = 0;
//     for lin in input.lines(){
//         if lin.is_empty() {
//             if temp > max {
//                 max = temp
//             }
//             temp = 0;
//         } else{
//             temp = temp + lin.parse::<u32>().unwrap();
//         }
//     }
//    Some(max)
// }

// pub fn part_two(input: &str) -> Option<u32> {
//     let mut max = 0;
//     let mut max1 = 0;
//     let mut max2 = 0;
//     let mut temp = 0;
//     for lin in input.lines() {
//         if lin.is_empty() {
//             if temp > max {
//                 max2 = max1;
//                 max1 = max;
//                 max = temp;
//             } else if temp > max1 {
//                 max2 = max1;
//                 max1 = temp;
//             } else if temp > max2 {
//                 max2 = temp;
//             }
//             temp = 0;
//         } else {
//             temp = temp + lin.parse::<u32>().unwrap();
//         }
//     }
//     Some(max + max1 + max2)
// }
pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|list| {
            list.lines()
                .map(|weight| weight.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
    // .unwrap_or(0),
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut c: Vec<u32> = input
        .split("\n\n")
        .map(|list| {
            list.lines()
                .map(|weight| weight.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect();
    c.sort();
    Some(c.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
