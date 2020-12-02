use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug)]
struct TestCase {
    min: u32,
    max: u32,
    must_have_char: char,
    password: String
}

impl std::fmt::Display for TestCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}-{} {}: {}", self.min, self.max, self.must_have_char, self.password)
    }
}

impl TestCase {
    fn is_valid(&self) -> bool {
        let mut alphabet_counter: HashMap<char, u32> = HashMap::new();
        for c in self.password.chars() {
            match alphabet_counter.get(&c) {
                Some(count) => alphabet_counter.insert(c, *count + 1),
                None => alphabet_counter.insert(c, 1)
            };
        }

        match alphabet_counter.get(&self.must_have_char) {
            Some(count) => self.min <= *count && self.max >= *count,
            None => false
        }
    }

    fn is_valid_according_to_official_rules(&self) -> bool {
        if self.password.len() < (self.min as usize) || self.password.len() < (self.max as usize) {
            return false;
        }

        let char_at_first_pos = self.password.chars().nth((self.min - 1) as usize).unwrap();
        let char_at_second_pos = self.password.chars().nth((self.max - 1) as usize).unwrap();

        (char_at_first_pos == self.must_have_char) ^ (char_at_second_pos == self.must_have_char)
    }
}

fn get_input() -> Vec<TestCase> {
    let input_path = Path::new("input.txt");
    let f = match File::open(input_path) {
        Err(why) => panic!("could not open {}: {}", input_path.display(), why),
        Ok(file) => file
    };

    let mut test_cases = vec![];

    let lines = BufReader::new(f).lines();
    for line_r in lines {
        let line = line_r.unwrap();
        let input_parts: Vec<&str> = line.split_whitespace().collect();

        let length_specification = input_parts[0];
        let character_specification = input_parts[1];
        let password = input_parts[2];

        let length_specification_parts: Vec<&str> = length_specification.split('-').collect();
        let min_length_string = length_specification_parts[0];
        let max_length_string = length_specification_parts[1];

        test_cases.push(TestCase{
            min: min_length_string.parse::<u32>().unwrap(),
            max: max_length_string.parse::<u32>().unwrap(),

            must_have_char: character_specification.chars().nth(0).unwrap(),
            password: String::from(password)
        });
    }

    return test_cases;
}

fn main() {
    let test_cases = get_input();
    let mut correct_cases = 0;

    for test_case in test_cases {
        if test_case.is_valid_according_to_official_rules() {
            correct_cases += 1;
        }
    }

    println!("Correct cases: {}", correct_cases);
}