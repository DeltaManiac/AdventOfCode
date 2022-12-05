pub fn part_one(input: &str) -> Option<u32> {

	let mut score = 0;
	for line in input.lines() {
		let parts = line.split(" ").collect::<Vec<&str>>();
		match parts[1]  {
			"X" => {
				score += 1;
				if parts[0] == "A" {
					score += 3;
				} else if parts[0] == "C" {
					score += 6;
				}
			},
			"Y" => {
				score += 2;
				if parts[0] == "B" {
					score += 3;
				} else if parts[0] == "A" {
					score += 6;
				}
			},
			"Z" => {
				score += 3;
				if parts[0] == "C" {
					score += 3;
				} else if parts[0] == "B" {
					score += 6;
				}
			},
			_ => unreachable!(),
		}
		//println!("{}", score);
	}
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    	let mut score = 0;
	for line in input.lines() {
		let parts = line.split(" ").collect::<Vec<&str>>();
		match parts[1]  {
			"X" => {
				if parts[0] == "A" {
					score += 3;
				} else if parts[0] == "C" {
					score += 2;
				} else {
					score += 1;
				}
			},
			"Y" => {
				score += 3;
				if parts[0] == "B" {
					score += 2;
				} else if parts[0] == "A" {
					score += 1;
				} else {
					score += 3;
				}
			},
			"Z" => {
				score += 6;
				if parts[0] == "C" {
					score += 1;
				} else if parts[0] == "B" {
					score += 3;
				} else {
					score += 2;
				}
			},
			_ => unreachable!(),
		}
		//println!("{}", score);
	}

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
