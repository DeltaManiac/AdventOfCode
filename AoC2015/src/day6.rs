enum Action {
    On,
    Off,
    Toggle,
}

pub struct Instruction {
    start: (usize, usize),
    end: (usize, usize),
    action: Action,
}

#[aoc_generator(day6)]
pub fn input_to_struct(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut c = line.split_whitespace().rev();
            let mut d = c.next().unwrap().split(',');
            let end = (
                d.next().unwrap().parse::<usize>().unwrap(),
                d.next().unwrap().parse::<usize>().unwrap(),
            );
            c.next();
            let mut d = c.next().unwrap().split(',');
            let start = (
                d.next().unwrap().parse::<usize>().unwrap(),
                d.next().unwrap().parse::<usize>().unwrap(),
            );
            let action = match c.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                "toggle" => Action::Toggle,
                _ => unreachable!(),
            };

            Instruction {
                start: start,
                end: end,
                action: action,
            }
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Instruction>) -> i64 {
    let mut switch: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let mut count: i64 = 0;
    for x in input {
        for i in x.start.0..=x.end.0 {
            for j in x.start.1..=x.end.1 {
                match x.action {
                    Action::Off => {
                        if switch[i][j] == 1 {
                            count -= 1;
                            switch[i][j] = 0;
                        }
                    }
                    Action::On => {
                        if switch[i][j] == 0 {
                            count += 1;
                            switch[i][j] = 1;
                        }
                    }
                    Action::Toggle => {
                        if switch[i][j] == 0 {
                            count += 1;
                            switch[i][j] = 1;
                        } else {
                            count += -1;
                            switch[i][j] = 0;
                        }
                    }
                }
            }
        }
    }
    count
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<Instruction>) -> i64 {
    let mut switch: [[i16; 1000]; 1000] = [[0; 1000]; 1000];
    let mut bright: i64 = 0;
    for x in input {
        for i in x.start.0..=x.end.0 {
            for j in x.start.1..=x.end.1 {
                match x.action {
                    Action::Off => {
                        if switch[i][j] > 0 {
                            bright -= 1;
                            switch[i][j] -= 1;
                        }
                    }
                    Action::On => {
                        bright += 1;
                        switch[i][j] += 1;
                    }
                    Action::Toggle => {
                        bright += 2;
                        switch[i][j] += 2;
                    }
                }
            }
        }
    }
    bright
}
