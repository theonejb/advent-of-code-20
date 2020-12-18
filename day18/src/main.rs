use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;

mod tests;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Operator::Add => match other {
                Operator::Mul => Ordering::Greater,
                Operator::Div => Ordering::Greater,
                Operator::Add => Ordering::Equal,
                Operator::Sub => Ordering::Equal
            },
            Operator::Sub => match other {
                Operator::Mul => Ordering::Greater,
                Operator::Div => Ordering::Greater,
                Operator::Add => Ordering::Equal,
                Operator::Sub => Ordering::Equal
            },
            Operator::Mul => match other {
                Operator::Mul => Ordering::Equal,
                Operator::Div => Ordering::Equal,
                Operator::Add => Ordering::Less,
                Operator::Sub => Ordering::Less
            },
            Operator::Div => match other {
                Operator::Mul => Ordering::Equal,
                Operator::Div => Ordering::Equal,
                Operator::Add => Ordering::Less,
                Operator::Sub => Ordering::Less
            },
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Operator {
    pub fn apply(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Operator::Add => op1 + op2,
            Operator::Sub => op1 - op2,
            Operator::Mul => op1 * op2,
            Operator::Div => op1 / op2
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(Operator),
    Operand(i64),
    ParenthesisGroup(Vec<Token>),
    ParenthesisedToken(String),
}

/*
Returns the next token and the left over string
 */
fn next_token(input: &str) -> (Token, &str) {
    let input = input.trim();

    if input.starts_with("(") {
        // Remove the starting parenthesis
        let input = &input[1..];

        let mut parenthesised_group = String::new();

        let mut num_parens = 1;
        for c in input.chars() {
            match c {
                ')' => {
                    num_parens -= 1;

                    if num_parens == 0 {
                        break;
                    }
                }
                '(' => {
                    num_parens += 1;
                }
                _ => {}
            }

            parenthesised_group.push(c);
        }

        // Remove the group we just parsed
        let left_over = input.strip_prefix(&parenthesised_group).unwrap();
        // Remove the ending parent for that group
        let left_over = &left_over[1..];
        // Remove any left over white space after the group ends
        let left_over = left_over.trim();

        return (Token::ParenthesisedToken(parenthesised_group), left_over);
    }

    let split: Vec<&str> = input.splitn(2, " ").collect();
    let token_str = split[0];
    let left_over = if split.len() == 2 {
        split[1]
    } else {
        ""
    };

    let token = match token_str {
        "+" => Token::Operator(Operator::Add),
        "-" => Token::Operator(Operator::Sub),
        "*" => Token::Operator(Operator::Mul),
        "/" => Token::Operator(Operator::Div),
        other => Token::Operand(
            other.parse::<i64>().unwrap()
        )
    };

    (token, left_over)
}

fn parse(input: &str) -> Vec<Token> {
    let mut input = input;
    let mut output = vec![];

    loop {
        let (token, left_over) = next_token(input);
        match token {
            Token::ParenthesisedToken(new_input) => {
                output.push(
                    Token::ParenthesisGroup(parse(&new_input))
                );
            }
            t => {
                output.push(t);
            }
        }

        if left_over.is_empty() {
            break;
        }

        input = left_over;
    }

    output
}

fn value_of(token: &Token, calculate: fn(&Vec<Token>) -> i64) -> i64 {
    match token {
        Token::Operand(v) => *v,
        Token::ParenthesisGroup(v) => calculate(v),
        _ => {
            panic!("This shouldn't happen.");
        }
    }
}

fn calculate(input: &Vec<Token>) -> i64 {
    let mut input_iter = input.iter();

    let mut first_operand = value_of(input_iter.next().unwrap(), calculate);
    let mut operator = input_iter.next().unwrap();
    let mut second_operand = value_of(input_iter.next().unwrap(), calculate);

    if let Token::Operator(op) = operator {
        first_operand = op.apply(first_operand, second_operand);
    }

    for token in input_iter {
        match token {
            Token::Operator(_) => {
                operator = token;
            }
            _ => {
                if let Token::Operator(op) = operator {
                    first_operand = op.apply(first_operand, value_of(token, calculate));
                }
            }
        }
    }

    first_operand
}

fn calculate2(input: &Vec<Token>) -> i64 {
    let mut output_stack: Vec<&Token> = vec![];
    let mut operator_stack: Vec<&Token> = vec![];

    for token in input.iter() {
        match token {
            Token::Operator(op) => {
                while !operator_stack.is_empty() {
                    let other_op_token = *operator_stack.last().unwrap();
                    if let Token::Operator(other_op) = other_op_token {
                        if other_op > op {
                            output_stack.push(
                                operator_stack.pop().unwrap()
                            );
                        } else {
                            break;
                        }
                    }
                }

                operator_stack.push(token);
            }
            _ => {
                output_stack.push(token);
            }
        }
    }

    while !operator_stack.is_empty() {
        output_stack.push(
            operator_stack.pop().unwrap()
        );
    }

    let mut operand_stack: Vec<i64> = vec![];
    for token in output_stack {
        match token {
            Token::Operator(op) => {
                let op1 = operand_stack.pop().unwrap();
                let op2 = operand_stack.pop().unwrap();
                operand_stack.push(
                    op.apply(op1, op2)
                );
            },
            _ => {
                operand_stack.push(
                    value_of(token, calculate2)
                );
            }
        }
    }

    operand_stack.pop().unwrap()
}

fn get_input(filename: &str) -> Vec<String> {
    let p = Path::new(filename);
    let f = File::open(p).unwrap();
    let lines = BufReader::new(f).lines();

    let mut input = vec![];

    for l in lines {
        input.push(l.unwrap());
    }

    input
}

fn main() {
    let input = get_input("input.txt");
    let mut sum = 0;

    for expression in input.iter() {
        let parsed_expression = parse(expression.as_str());
        sum += calculate(&parsed_expression);
    }

    println!("Part 1: {}", sum);

    let mut sum = 0;

    for expression in input.iter() {
        let parsed_expression = parse(expression.as_str());
        sum += calculate2(&parsed_expression);
    }

    println!("Part 2: {}", sum);
}
