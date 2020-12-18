use regex::Regex;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Field {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Field {
    pub fn is_valid(&self, field: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&field))
    }
}

type Ticket = Vec<u64>;

#[derive(Debug)]
struct Input {
    fields: Vec<(String, Field)>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut fields = vec![];
    loop {
        if let Some(l) = re.captures(lines.next().unwrap()) {
            let field = Field {
                ranges: vec![
                    l[2].parse().unwrap()..=l[3].parse().unwrap(),
                    l[4].parse().unwrap()..=l[5].parse().unwrap(),
                ],
            };
            fields.push((l[1].to_owned(), field));
        } else {
            break;
        }
    }
    let ticket = lines
        .nth(1)
        .map(|l| l.split(',').map(|w| w.parse().unwrap()).collect())
        .unwrap();
    let nearby_tickets = lines
        .skip(2)
        .map(|l| l.split(',').map(|w| w.parse().unwrap()).collect())
        .collect();
    Input {
        fields,
        ticket,
        nearby_tickets,
    }
}
#[aoc(day16, part1)]
fn part1(input: &Input) -> u64 {
    input
        .nearby_tickets
        .iter()
        .map(|v| {
            v.iter()
                .filter(|&&t| !input.fields.iter().any(|&(_, ref f)| f.is_valid(t)))
                .sum::<u64>()
        })
        .sum()
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> u64 {
    let mut valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|v| {
            v.iter()
                .all(|&t| input.fields.iter().any(|&(_, ref f)| f.is_valid(t)))
        })
        .collect::<Vec<_>>();
    valid_tickets.push(&input.ticket);
    let mut possible = Vec::new();
    for v in 0..input.ticket.len() {
        let mut p = Vec::new();
        for (i, (_, f)) in input.fields.iter().enumerate() {
            if valid_tickets.iter().all(|t| f.is_valid(t[v])) {
                p.push(i);
            }
        }
        possible.push(p);
    }
    let mut seen = BTreeSet::new();
    while let Some(f) = possible
        .iter()
        .find(|p| p.len() == 1 && !seen.contains(&p[0]))
        .map(|f| f[0])
    {
        seen.insert(f);
        possible.iter_mut().for_each(|p| {
            if p.len() > 1 {
                p.retain(|&x| x != f);
            }
        });
    }
    possible
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if input.fields[n[0]].0.contains("departure") {
                Some(input.ticket[i])
            } else {
                None
            }
        })
        .product()
}