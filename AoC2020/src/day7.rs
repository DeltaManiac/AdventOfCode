use std::collections::HashMap;

type Rules = HashMap<String, HashMap<String, usize>>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Rules {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" bags contain ");
            let color = parts.next().unwrap().to_string();
            let rules = parts
                .next()
                .unwrap()
                .split(", ")
                .filter_map(|element| {
                    let mut words = element.splitn(2, ' ');
                    let n = match words.next()? {
                        "no" => None,
                        n => n.parse::<usize>().ok(),
                    }?;
                    let inner = words.next()?.rsplitn(2, ' ').skip(1).next()?.to_string();
                    (inner, n).into()
                })
                .collect::<HashMap<String, usize>>();
            (color, rules)
        })
        .collect()
}

fn can_reach_gold_bag(rules: &Rules, color: &str) -> bool {
    rules[color]
        .iter()
        .any(|(color, _)| color == "shiny gold" || can_reach_gold_bag(rules, color))
}

#[aoc(day7, part1)]
fn part1(rules: &Rules) -> usize {
    rules
        .keys()
        .filter(|color| can_reach_gold_bag(rules, color))
        .count()
}

fn bag_count(rules: &Rules, color: &str) -> usize {
    1_usize
        + rules[color]
            .iter()
            .map(|(color, count)| count * bag_count(rules, color))
            .sum::<usize>()
}

#[aoc(day7, part2)]
fn part2(rules: &Rules) -> usize {
    bag_count(rules, "shiny gold") - 1
}
