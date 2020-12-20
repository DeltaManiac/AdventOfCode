use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

enum Rule {
    Character(char),
    Product(Vec<usize>),
    Sum(Vec<Vec<usize>>),
}

fn validate<'a>(message: &'a str, rules: &Rules, rule: &Rule) -> Result<&'a str, ()> {
    use Rule::*;

    match rule {
        Character(c) => {
            if message.starts_with(*c) {
                Ok(&message[1..])
            } else {
                Err(())
            }
        }
        Product(rules_numbers) => {
            let mut remainder = message;

            for number in rules_numbers {
                remainder = validate(remainder, rules, &rules[number])?;
            }

            Ok(remainder)
        }
        Sum(products) => {
            for product in products {
                let result = validate(message, rules, &Product(product.clone()));

                if result.is_ok() {
                    return result;
                }
            }

            Err(())
        }
    }
}

type Rules = HashMap<usize, Rule>;

fn parse_rule(rule_str: &str) -> Rule {
    use Rule::*;

    if rule_str.starts_with('"') {
        Character(rule_str.chars().nth(1).unwrap())
    } else if rule_str.contains('|') {
        Sum(rule_str
            .split('|')
            .map(|product_str| {
                product_str
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect())
    } else {
        Product(
            rule_str
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        )
    }
}

fn parse_rules(rules_str: &str) -> Rules {
    rules_str
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                parse_rule(split.next().unwrap()),
            )
        })
        .collect()
}

type Message = String;
type Messages = Vec<Message>;

fn parse_messages(messages_str: &str) -> Messages {
    messages_str.lines().map(|line| line.to_string()).collect()
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (Rules, Messages) {
    let mut split = input.split("\n\n");

    (
        parse_rules(split.next().unwrap()),
        parse_messages(split.next().unwrap()),
    )
}

#[aoc(day19, part1)]
fn part1((rules, messages): &(Rules, Messages)) -> usize {
    messages
        .iter()
        .filter(|&message| {
            if let Ok(remainder) = validate(&message, &rules, &rules[&0]) {
                remainder.is_empty()
            } else {
                false
            }
        })
        .count()
}

#[aoc(day19, part2)]
fn part2((rules, messages): &(Rules, Messages)) -> usize {
    messages
        .iter()
        .filter(|&message| {
            let mut count_42 = 0;
            let mut remainder = message.as_str();
            let mut result = validate(&remainder, &rules, &rules[&42]);

            while let Ok(new_remainder) = result {
                count_42 += 1;
                remainder = new_remainder;
                result = validate(&remainder, &rules, &rules[&42]);
            }

            if count_42 < 2 {
                return false;
            }

            let mut count_31 = 0;
            result = validate(&remainder, &rules, &rules[&31]);

            while let Ok(new_remainder) = result {
                count_31 += 1;
                remainder = new_remainder;
                result = validate(&remainder, &rules, &rules[&31]);
            }

            remainder.is_empty() && count_31 > 0 && count_42 > count_31
        })
        .count()
}