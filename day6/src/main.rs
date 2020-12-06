mod tests;

use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

struct GroupAnswers {
    answers: Vec<String>
}

impl GroupAnswers {
    fn new() -> GroupAnswers {
        GroupAnswers { answers: vec![] }
    }

    fn push_answers_for_member(&mut self, answers: &str) {
        self.answers.push(String::from(answers));
    }

    fn count_yes_answers(&self) -> u32 {
        let mut counter: HashMap<char, u32> = std::collections::HashMap::new();

        for member_answers in self.answers.iter() {
            for yes_answer_label in member_answers.chars() {
                let current_counter = counter.get(&yes_answer_label);
                let mut new_counter = 1;

                if let Some(count) = current_counter {
                    new_counter = count + 1;
                }

                counter.insert(yes_answer_label, new_counter);
            }
        }

        let mut yes_count = 0;
        for (_k, v) in counter.iter() {
            if *v > 0 {
                yes_count += 1;
            }
        }

        yes_count
    }

    fn count_common_yes_answers(&self) -> u32 {
        let mut counter: HashMap<char, u32> = std::collections::HashMap::new();

        for member_answers in self.answers.iter() {
            for yes_answer_label in member_answers.chars() {
                let current_counter = counter.get(&yes_answer_label);
                let mut new_counter = 1;

                if let Some(count) = current_counter {
                    new_counter = count + 1;
                }

                counter.insert(yes_answer_label, new_counter);
            }
        }

        let number_of_members = self.answers.len();
        let mut yes_count = 0;
        for (_k, v) in counter.iter() {
            if *v == number_of_members as u32 {
                yes_count += 1;
            }
        }

        yes_count
    }
}

fn get_input_groups(filename: &str) -> Vec<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut groups = vec![];
    let mut current_group = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            groups.push(current_group);
            current_group = vec![];
        } else {
            current_group.push(line);
        }
    }

    if current_group.len() > 0 {
        groups.push(current_group);
    }

    groups
}

fn get_yes_counts_for_groups(groups: &Vec<Vec<String>>) -> u32 {
    let mut count = 0;

    for group_answer_strings in groups.iter() {
        let mut group_answers = GroupAnswers::new();
        for member_answer in group_answer_strings.iter() {
            group_answers.push_answers_for_member(member_answer.as_str());
        }

        count += group_answers.count_yes_answers();
    }

    count
}

fn get_common_yes_counts_for_groups(groups: &Vec<Vec<String>>) -> u32 {
    let mut count = 0;

    for group_answer_strings in groups.iter() {
        let mut group_answers = GroupAnswers::new();
        for member_answer in group_answer_strings.iter() {
            group_answers.push_answers_for_member(member_answer.as_str());
        }

        count += group_answers.count_common_yes_answers();
    }

    count
}

fn main() {
    let input_groups = get_input_groups("input.txt");
    println!("Count: {}", get_yes_counts_for_groups(&input_groups));
    println!("Common Count: {}", get_common_yes_counts_for_groups(&input_groups));
}
