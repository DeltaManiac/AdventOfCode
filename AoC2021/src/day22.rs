use std::ops::RangeInclusive;

// const INPUT: &str = include_str!("../../inputs/day22.txt");

// fn main() {
//     let instructions = parse_input(INPUT);
//     println!("Answer 1: {}", num_cubes_on(instructions.clone(), true));
//     println!("Answer 2: {}", num_cubes_on(instructions, false));
// }

pub fn solve_part1(input: &Vec<Instruction>) -> usize {
    num_cubes_on(input.clone(), true)
}

pub fn solve_part2(input: &Vec<Instruction>) -> usize {
    num_cubes_on(input.clone(), false)
}

#[derive(Debug, Clone)]
enum Action {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    action: Action,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
}

#[derive(Debug)]
struct Cuboid {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
    holes: Vec<Cuboid>,
}

impl Cuboid {
    fn from(
        Instruction {
            action: _,
            x_range,
            y_range,
            z_range,
        }: Instruction,
    ) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
            holes: vec![],
        }
    }

    fn poke_hole(&mut self, other: &Self) {
        for hole in &mut self.holes {
            hole.poke_hole(other);
        }
        if let Some(hole) = self.calculate_intersection_hole(other) {
            self.holes.push(hole);
        }
    }

    fn calculate_intersection_hole(&self, other: &Self) -> Option<Self> {
        let calculate_intersection_range_on_axis =
            |range: &RangeInclusive<isize>, other_range: &RangeInclusive<isize>| {
                *range.start().max(other_range.start())..=*range.end().min(other_range.end())
            };
        let x_range = calculate_intersection_range_on_axis(&self.x_range, &other.x_range);
        let y_range = calculate_intersection_range_on_axis(&self.y_range, &other.y_range);
        let z_range = calculate_intersection_range_on_axis(&self.z_range, &other.z_range);

        if x_range.is_empty() || y_range.is_empty() || z_range.is_empty() {
            None
        } else {
            Some(Self {
                x_range,
                y_range,
                z_range,
                holes: vec![],
            })
        }
    }

    fn num_cubes(&self) -> usize {
        let calc_size = |range: &RangeInclusive<isize>| {
            if range.is_empty() {
                0
            } else {
                (range.end() - range.start() + 1) as usize
            }
        };
        let width = calc_size(&self.x_range);
        let height = calc_size(&self.y_range);
        let depth = calc_size(&self.z_range);
        (width * height * depth)
            - self
                .holes
                .iter()
                .fold(0, |acc, hole| acc + hole.num_cubes())
    }

    fn is_empty(&self) -> bool {
        self.holes.iter().any(|hole| {
            hole.x_range == self.x_range
                && hole.y_range == self.y_range
                && hole.z_range == self.z_range
        })
    }
}

fn num_cubes_on(mut instructions: Vec<Instruction>, trim: bool) -> usize {
    if trim {
        let trim_axis = |range: &mut RangeInclusive<isize>| {
            let (start, end) = std::mem::replace(range, 0..=0).into_inner();
            *range = RangeInclusive::new(start.max(-50), end.min(50));
        };

        for ins in &mut instructions {
            trim_axis(&mut ins.x_range);
            trim_axis(&mut ins.y_range);
            trim_axis(&mut ins.z_range);
        }
    }

    let mut cuboids_on = Vec::<Cuboid>::new();
    for instruction in instructions {
        let action = instruction.action.clone();
        cuboids_on
            .iter()
            .filter(|cuboid| cuboid.is_empty())
            .for_each(drop);

        let new_cuboid = Cuboid::from(instruction);
        for cuboid in &mut cuboids_on {
            cuboid.poke_hole(&new_cuboid)
        }

        if let Action::On = action {
            cuboids_on.push(new_cuboid);
        }
    }

    cuboids_on
        .iter()
        .fold(0, |acc, cuboid| acc + cuboid.num_cubes())
}

pub fn input_generator(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| {
            let (action, ranges) = line.split_once(' ').unwrap();
            let action = match action {
                "on" => Action::On,
                "off" => Action::Off,
                _ => unreachable!(),
            };
            let [x_range, y_range, z_range]: [&str; 3] =
                ranges.split(',').collect::<Vec<_>>().try_into().unwrap();
            let str_to_range = |input: &str| {
                let (_, range) = input.split_once('=').unwrap();
                let (start, end) = range.split_once("..").unwrap();
                RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
            };
            let x_range = str_to_range(x_range);
            let y_range = str_to_range(y_range);
            let z_range = str_to_range(z_range);
            Instruction {
                action,
                x_range,
                y_range,
                z_range,
            }
        })
        .collect()
}
