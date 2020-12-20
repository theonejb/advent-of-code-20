use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};

mod tests;

type RuleId = i32;

#[derive(Debug, PartialEq)]
enum Rule {
    Match(char),
    Chain(Vec<RuleId>),
    Options(Box<Rule>, Box<Rule>),
}

impl Rule {
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Match(c) => write!(f, "= {}", c),
            Rule::Chain(rule_ids) => {
                write!(f, "[");
                for (i, rule_id) in rule_ids.iter().enumerate() {
                    write!(f, "{}", rule_id);
                    if i < rule_ids.len() - 1 {
                        write!(f, ",");
                    }
                }
                write!(f, "]")
            },
            Rule::Options(rule1, rule2) => write!(f, "{} | {}", rule1.display(), rule2.display())
        }
    }
}

struct NestedPrinter {
    level: usize,
    min_print_level: usize,
    mute: bool,
}

impl NestedPrinter {
    pub fn new() -> NestedPrinter {
        NestedPrinter { level: 0, min_print_level: 0, mute: false }
    }

    pub fn println(&self, output: &str) {
        if self.mute || self.level < self.min_print_level {
            return;
        }

        for _ in 0..self.level {
            print!("|");
        }

        println!("- {}", output);
    }

    pub fn print_rule_and_input(&self, rule: &Rule, input: &str) {
        if self.mute || self.level < self.min_print_level {
            return;
        }

        for _ in 0..self.level {
            print!("|");
        }
        println!("- Rule: {} Input: {}", rule, input);
    }

    pub fn down(&mut self) {
        self.level += 1;
    }

    pub fn up(&mut self) {
        self.level -= 1;
    }
}

#[derive(Debug, PartialEq)]
struct RuleMatch {
    matched_chars: usize
}

impl Rule {
    //region Input parsing
    fn options_from_input(input: &str) -> Rule {
        let chain_inputs: Vec<&str> = input.split('|').collect();

        Rule::Options(
            Box::new(Rule::chain_from_input(chain_inputs[0].trim())),
            Box::new(Rule::chain_from_input(chain_inputs[1].trim())),
        )
    }

    fn match_from_input(input: &str) -> Rule {
        let starting_quote_index = input.find('"').unwrap();
        Rule::Match(input.chars().nth(starting_quote_index + 1).unwrap())
    }

    fn chain_from_input(input: &str) -> Rule {
        let mut chain_parts = vec![];

        for rule_id in input.split_whitespace() {
            chain_parts.push(
                rule_id.parse::<i32>().unwrap()
            );
        }

        Rule::Chain(chain_parts)
    }

    pub fn from_input(input: &str) -> (RuleId, Rule) {
        let mut rule_parts: Vec<&str> = input.split(':').collect();

        let rule_id = rule_parts[0];
        let rule_id = rule_id.parse::<i32>().unwrap();

        let rule_spec = rule_parts[1].trim();

        return (rule_id, if rule_spec.contains('|') {
            Rule::options_from_input(rule_spec)
        } else if rule_spec.contains('"') {
            Rule::match_from_input(rule_spec)
        } else {
            Rule::chain_from_input(rule_spec)
        });
    }

    pub fn is_match(&self) -> bool {
        return if let Rule::Match(_) = self {
            true
        } else {
            false
        };
    }

    pub fn is_chain(&self) -> bool {
        return if let Rule::Chain(_) = self {
            true
        } else {
            false
        };
    }

    pub fn is_option(&self) -> bool {
        return if let Rule::Options(_, _) = self {
            true
        } else {
            false
        };
    }
    //endregion

    //region Matching
    fn match_chain_rule(&self, rule_book: &RuleBook, input: &str, printer: &mut NestedPrinter) -> Option<RuleMatch> {
        let mut last_match_index = 0;

        if let Rule::Chain(components) = self {
            for component_id in components.iter() {
                printer.println(&format!("Component id {}", component_id));
                let rule = rule_book.get(component_id).unwrap();
                match rule.do_match(rule_book, &input[last_match_index..], printer) {
                    None => {
                        return None;
                    }
                    Some(rule_match) => {
                        last_match_index += rule_match.matched_chars;
                    }
                }
            }
        }

        Some(RuleMatch { matched_chars: last_match_index })
    }

    fn match_options_rule(&self, rule_book: &RuleBook, input: &str, printer: &mut NestedPrinter) -> Option<RuleMatch> {
        if let Rule::Options(option1, option2) = self {
            printer.println("Trying option 1");
            let op1_match = option1.do_match(rule_book, input, printer);
            if op1_match.is_some() {
                return op1_match;
            }

            printer.println("Trying option 2");
            let op2_match = option2.do_match(rule_book, input, printer);
            if op2_match.is_some() {
                return op2_match;
            }
        }

        None
    }

    pub fn do_match(&self, rule_book: &RuleBook, input: &str, printer: &mut NestedPrinter) -> Option<RuleMatch> {
        printer.print_rule_and_input(self, input);
        printer.down();

        if input.is_empty() {
            return None;
        }

        let result = match self {
            Rule::Match(c) => {
                if input.chars().nth(0).unwrap() == *c {
                    Some(RuleMatch { matched_chars: 1 })
                } else {
                    None
                }
            }
            Rule::Chain(_) => {
                self.match_chain_rule(rule_book, input, printer)
            }
            Rule::Options(_, _) => {
                self.match_options_rule(rule_book, input, printer)
            }
        };

        printer.up();

        result
    }

    fn do_complete_match(&self, rule_book: &RuleBook, input: &str, printer: &mut NestedPrinter) -> Option<RuleMatch> {
        match self.do_match(rule_book, input, printer) {
            Some(rule_match) => {
                if rule_match.matched_chars == input.len() {
                    Some(rule_match)
                } else {
                    None
                }
            }
            None => None
        }
    }

    pub fn does_match_completely(&self, rule_book: &RuleBook, input: &str) -> bool {
        let mut printer = NestedPrinter::new();
        printer.mute = true;

        match self.do_match(rule_book, input, &mut printer) {
            Some(rule_match) => rule_match.matched_chars == input.len(),
            None => false
        }
    }
    //endregion

    //region Expanding
    pub fn to_regex_pattern(&self, rule_book: &RuleBook) -> String {
        match self {
            Rule::Match(c) => format!("{}", c),
            Rule::Chain(components) => {
                let mut chain_representation = String::new();
                for (i, component) in components.iter().enumerate() {
                    let rule = rule_book.get(component).unwrap();
                    chain_representation += &rule.to_regex_pattern(rule_book);
                }

                chain_representation
            },
            Rule::Options(rule1, rule2) => {
                format!("({}|{})", rule1.to_regex_pattern(rule_book), rule2.to_regex_pattern(rule_book))
            }
        }
    }
    //endregion
}

type RuleBook = HashMap<i32, Rule>;

fn get_input(rules_filename: &str, data_filename: &str) -> (Vec<String>, Vec<String>) {
    let f = File::open(Path::new(rules_filename)).unwrap();
    let mut rules = vec![];
    for line in BufReader::new(f).lines() {
        rules.push(line.unwrap());
    }

    let f = File::open(Path::new(data_filename)).unwrap();
    let mut data = vec![];
    for line in BufReader::new(f).lines() {
        data.push(line.unwrap());
    }

    (rules, data)
}

fn main() {
    let (rules, inputs) = get_input("input_rules.txt", "input_data.txt");

    let mut rule_book = RuleBook::new();
    for rule_input in rules.iter() {
        let (rule_id, rule) = Rule::from_input(&rule_input);
        rule_book.insert(rule_id, rule);
    }

    let rule_0 = rule_book.get(&0).unwrap();
    let mut num_matches = 0;
    for value in inputs.iter() {
        if rule_0.does_match_completely(&rule_book, &value) {
            num_matches += 1;
        }
    }

    println!("Part 1: {}", num_matches);

    let rule_42 = rule_book.get(&42).unwrap();
    println!("42 expanded\n{}", rule_42.to_regex_pattern(&rule_book));

    let rule_11 = rule_book.get(&11).unwrap();
    println!("11 expanded\n{}", rule_11.to_regex_pattern(&rule_book));
}
