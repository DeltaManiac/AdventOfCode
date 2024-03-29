use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Hash)]
enum Operation {
    Add,
    Mul,
}

#[derive(Eq, PartialEq)]
enum Token {
    Number(i64),
    Operator(Operation),
    LeftParenthesis,
    RightParenthesis,
}

struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn evaluate(&self) -> i64 {
        let mut stack = Vec::new();

        use Operation::*;
        use Token::*;

        for token in &self.tokens {
            match token {
                Number(number) => {
                    stack.push(*number);
                }
                Operator(operation) if *operation == Add => {
                    let lhs = stack.pop().unwrap();
                    let rhs = stack.pop().unwrap();
                    stack.push(lhs + rhs);
                }
                Operator(operation) if *operation == Mul => {
                    let lhs = stack.pop().unwrap();
                    let rhs = stack.pop().unwrap();
                    stack.push(lhs * rhs);
                }
                _ => {}
            }
        }

        stack.pop().unwrap()
    }
}

fn parse_expression(expression_str: &str, check_precedence: bool) -> Expression {
    let expression_str = expression_str.replace("(", " ( ").replace(")", " ) ");

    use Operation::*;
    use Token::*;

    let mut tokens: Vec<_> = expression_str
        .split_whitespace()
        .map(|token_str| match token_str {
            "*" => Operator(Mul),
            "+" => Operator(Add),
            "(" => LeftParenthesis,
            ")" => RightParenthesis,
            _ => Number(token_str.parse::<i64>().unwrap()),
        })
        .collect();

    let mut operator_stack = Vec::new();
    let mut output = Vec::new();

    for token in tokens.drain(0..) {
        match token {
            Number(_) => output.push(token),
            Operator(_) => {
                while let Some(operator_token) = operator_stack.pop() {
                    if operator_token == LeftParenthesis
                        || (check_precedence
                            && operator_token == Operator(Mul)
                            && token == Operator(Add))
                    {
                        operator_stack.push(operator_token);
                        break;
                    } else {
                        output.push(operator_token);
                    }
                }

                operator_stack.push(token);
            }
            LeftParenthesis => {
                operator_stack.push(token);
            }
            RightParenthesis => {
                while let Some(operator_token) = operator_stack.pop() {
                    if operator_token == LeftParenthesis {
                        break;
                    } else {
                        output.push(operator_token);
                    }
                }
            }
        }
    }

    while let Some(operator_token) = operator_stack.pop() {
        if operator_token != LeftParenthesis {
            output.push(operator_token);
        }
    }

    Expression { tokens: output }
}

#[aoc_generator(day18, part1)]
fn parse_input1(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|line| parse_expression(line, false))
        .collect()
}

#[aoc_generator(day18, part2)]
fn parse_input2(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|line| parse_expression(line, true))
        .collect()
}

#[aoc(day18, part1)]
fn part1(expressions: &[Expression]) -> i64 {
    expressions
        .iter()
        .map(|expression| expression.evaluate())
        .sum()
}

#[aoc(day18, part2)]
fn part2(expressions: &[Expression]) -> i64 {
    expressions
        .iter()
        .map(|expression| expression.evaluate())
        .sum()
}
