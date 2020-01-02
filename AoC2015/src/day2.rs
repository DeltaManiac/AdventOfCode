#[derive(Copy, Clone)]
pub struct Prism {
    x: u32,
    y: u32,
    z: u32,
}

impl Prism {
    pub fn get_smallest_side_area(self) -> u32 {
        *[self.x * self.y, self.y * self.z, self.z * self.x]
            .iter()
            .min()
            .unwrap_or(&0)
    }

    pub fn get_area(self) -> u32 {
        [
            2 * self.x * self.y,
            2 * self.y * self.z,
            2 * self.z * self.x,
        ]
        .iter()
        .sum::<u32>()
    }

    pub fn get_smallest_permiter(self) -> u32 {
        *[
            2 * (self.x + self.y),
            2 * (self.y + self.z),
            2 * (self.z + self.x),
        ]
        .iter()
        .min()
        .unwrap_or(&0)
    }
}

#[aoc_generator(day2)]
pub fn text_to_struct(input: &str) -> Vec<Prism> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.trim().split('x');
            Prism {
                x: chars.next().unwrap().parse().unwrap(),
                y: chars.next().unwrap().parse().unwrap(),
                z: chars.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<Prism>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Prism]) -> u32 {
    input
        .iter()
        .map(|x| x.get_smallest_side_area() + x.get_area())
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Prism]) -> u32 {
    input
        .iter()
        .map(|x| x.get_smallest_permiter() + (x.x * x.y * x.z))
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}
